use crate::LOGIN_PAGE;
use serde::Deserialize;
use serde_bencode::de;
use sha1::{Digest, Sha1};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct TorrentBEncode {
    info: serde_bencode::value::Value,
}

#[derive(Debug, Deserialize)]
struct TorrentInfo {
    name: String,
    #[serde(default)]
    files: Option<Vec<TorrentFile>>,
    #[serde(default)]
    length: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct TorrentFile {
    pub length: i64,
    pub path: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub enum FileNode {
    File {
        name: String,
        size: i64,
    },
    Directory {
        name: String,
        size: i64, // cumulative size of all files in this directory
        children: Vec<FileNode>,
    },
}

impl FileNode {
    pub fn get_size(&self) -> i64 {
        match self {
            FileNode::File { size, .. } => *size,
            FileNode::Directory { size, .. } => *size,
        }
    }

    #[allow(dead_code)]
    pub fn get_name(&self) -> &str {
        match self {
            FileNode::File { name, .. } => name,
            FileNode::Directory { name, .. } => name,
        }
    }
}

pub fn flatten_tree(node: &FileNode) -> Vec<(String, i64)> {
    flatten_tree_helper(node, String::new())
}

fn flatten_tree_helper(node: &FileNode, current_path: String) -> Vec<(String, i64)> {
    match node {
        FileNode::File { name, size } => {
            let full_path = if current_path.is_empty() {
                name.clone()
            } else {
                format!("{}/{}", current_path, name)
            };
            vec![(full_path, *size)]
        }
        FileNode::Directory { name, children, .. } => {
            let new_path = if current_path.is_empty() {
                name.clone()
            } else {
                format!("{}/{}", current_path, name)
            };

            let mut result = Vec::new();
            for child in children {
                result.extend(flatten_tree_helper(child, new_path.clone()));
            }
            result
        }
    }
}

pub fn calculate_torrent_hash(torrent_bytes: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
    let torrent: TorrentBEncode = de::from_bytes(torrent_bytes)?;

    let info_bencoded = serde_bencode::to_bytes(&torrent.info)?;

    let mut hasher = Sha1::new();
    hasher.update(&info_bencoded);
    let result = hasher.finalize();

    Ok(format!("{:x}", result))
}

pub fn parse_torrent_files(torrent_bytes: &[u8]) -> Result<FileNode, Box<dyn std::error::Error>> {
    let torrent: TorrentBEncode = de::from_bytes(torrent_bytes)?;

    let info_bytes = serde_bencode::to_bytes(&torrent.info)?;
    let info: TorrentInfo = de::from_bytes(&info_bytes)?;

    // single-file torrent
    if let Some(length) = info.length {
        return Ok(FileNode::File {
            name: info.name,
            size: length,
        });
    }

    // multi-file torrent
    let files = info.files.ok_or("Missing files in multi-file torrent")?;
    let mut root_children: HashMap<String, Vec<(Vec<String>, i64)>> = HashMap::new();

    for file in files {
        if file.path.is_empty() {
            continue;
        }

        root_children
            .entry(file.path[0].clone())
            .or_insert_with(Vec::new)
            .push((file.path.clone(), file.length));
    }

    // tree building
    let children: Vec<FileNode> = root_children
        .into_iter()
        .map(|(name, paths)| build_node(name, paths))
        .collect();

    let total_size: i64 = children.iter().map(|n| n.get_size()).sum();

    Ok(FileNode::Directory {
        name: info.name,
        size: total_size,
        children,
    })
}

fn build_node(name: String, paths: Vec<(Vec<String>, i64)>) -> FileNode {
    if paths.len() == 1 && paths[0].0.len() == 1 {
        return FileNode::File {
            name,
            size: paths[0].1,
        };
    }

    let all_direct = paths.iter().all(|(p, _)| p.len() == 1);
    if all_direct && paths.len() == 1 {
        return FileNode::File {
            name,
            size: paths[0].1,
        };
    }

    let mut children_map: HashMap<String, Vec<(Vec<String>, i64)>> = HashMap::new();

    for (mut path, size) in paths {
        if path.len() == 1 {
            // direct file in this directory
            children_map
                .entry(path[0].clone())
                .or_insert_with(Vec::new)
                .push((path, size));
        } else {
            // nested path
            path.remove(0); // remove current directory name
            let next_name = path[0].clone();
            children_map
                .entry(next_name)
                .or_insert_with(Vec::new)
                .push((path, size));
        }
    }

    let children: Vec<FileNode> = children_map
        .into_iter()
        .map(|(child_name, child_paths)| build_node(child_name, child_paths))
        .collect();

    let total_size: i64 = children.iter().map(|n| n.get_size()).sum();

    FileNode::Directory {
        name,
        size: total_size,
        children,
    }
}

pub fn check_session_expired(response: &wreq::Response) -> bool {
    if !response.status().is_success() {
        let code = response.status();
        debug!("Response status code: {}", code);
        if code == 307 {
            warn!("Session expired...");
            return true;
        }
    }

    let final_url = response.url().as_str().to_string();
    if final_url.contains(LOGIN_PAGE) {
        warn!("Session expired...");
        return true;
    }

    false
}

#[cfg(test)]
mod test_utils {
    use super::*;

    pub fn print_file_tree(node: &FileNode, indent: usize) {
        let indent_str = "  ".repeat(indent);
        match node {
            FileNode::File { name, size } => {
                println!("{}📄 {} ({} bytes)", indent_str, name, size);
            }
            FileNode::Directory {
                name,
                size,
                children,
            } => {
                println!("{}📁 {} ({} bytes - total)", indent_str, name, size);
                for child in children {
                    print_file_tree(child, indent + 1);
                }
            }
        }
    }

    fn print_flatten(node: &FileNode, path: String) {
        match node {
            FileNode::File { name, size } => {
                let full_path = format!("{}/{}", path, name);
                println!("{} ({} bytes)", full_path, size);
            }
            FileNode::Directory { name, children, .. } => {
                let new_path = format!("{}/{}", path, name);
                for child in children {
                    print_flatten(child, new_path.clone());
                }
            }
        }
    }

    #[tokio::test]
    async fn test_torrent_hash() {
        let torrent = include_bytes!("../tests/test.torrent");
        let hash = calculate_torrent_hash(torrent).expect("Failed to calculate hash");
        println!("Torrent info hash: {}", hash);

        assert_eq!(hash.len(), 40);
        assert_eq!(hash, "d984f67af9917b214cd8b6048ab5624c7df6a07a");
    }

    #[tokio::test]
    async fn test_torrent_files() {
        let torrent = include_bytes!("../tests/test.torrent");
        let tree = parse_torrent_files(torrent).expect("Failed to parse torrent files");

        println!("\n=== Torrent File Tree (Pretty) ====");
        print_file_tree(&tree, 0);
        println!("=====================================\n");

        // flatten view
        println!("\n=== Torrent File Tree (flatten) ===");
        print_flatten(&tree, "".to_string());
        println!("=====================================\n");

        let json = serde_json::to_string_pretty(&tree).expect("Failed to convert to JSON");
        println!("\n=== Torrent File Tree (JSON) ====");
        println!("{}", json);
        println!("===================================\n");

        match &tree {
            FileNode::Directory {
                name,
                size,
                children,
            } => {
                println!("Root directory: {}", name);
                println!("Total size: {} bytes", size);
                println!("Number of items: {}", children.len());
            }
            FileNode::File { name, size } => {
                println!("Single file: {}", name);
                println!("Size: {} bytes", size);
            }
        }

        // assert total size is 19296724
        assert_eq!(tree.get_size(), 19296724);
        // assert there is a README file in the root directory 20bytes
        if let FileNode::Directory { children, .. } = &tree {
            let readme = children.iter().find(|n| n.get_name() == "README");
            assert!(readme.is_some());
            if let Some(FileNode::File { size, .. }) = readme {
                assert_eq!(*size, 20);
            }
        }
        // assert images folder is 19296704 bytes
        if let FileNode::Directory { children, .. } = &tree {
            let images = children.iter().find(|n| n.get_name() == "images");
            assert!(images.is_some());
            if let Some(FileNode::Directory { size, .. }) = images {
                assert_eq!(*size, 19296704);

                // assert melk-abbey-library.jpg is 1682177 bytes inside images folder
                if let FileNode::Directory { children, .. } = images.unwrap() {
                    let melk = children
                        .iter()
                        .find(|n| n.get_name() == "melk-abbey-library.jpg");
                    assert!(melk.is_some());
                    if let Some(FileNode::File { size, .. }) = melk {
                        assert_eq!(*size, 1682177);
                    }
                }

                // assert LOC_Main_Reading_Room_Highsmith.jpg is 17614527 bytes inside images folder
                if let FileNode::Directory { children, .. } = images.unwrap() {
                    let loc = children
                        .iter()
                        .find(|n| n.get_name() == "LOC_Main_Reading_Room_Highsmith.jpg");
                    assert!(loc.is_some());
                    if let Some(FileNode::File { size, .. }) = loc {
                        assert_eq!(*size, 17614527);
                    }
                }
            }
        }
    }
}
