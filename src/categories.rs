use serde::Serialize;
use tokio::sync::OnceCell;

pub static CATEGORIES_CACHE: OnceCell<Vec<Category>> = OnceCell::const_new();

#[derive(Debug, Serialize, Clone)]
pub struct Category {
    pub id: usize,
    pub name: String,
    pub sub_categories: Vec<Category>,
}

/// Map from Nostr #t tag value to numeric category ID (matching ygege.yml).
/// Returns None if the tag is not recognized.
pub fn nostr_tag_to_cat_id(tag: &str) -> Option<usize> {
    let normalized = tag.to_lowercase();
    let id = match normalized.as_str() {
        // Film/Vidéo
        "film/vidéo" | "film/video" => 2145,
        "animation" => 2178,
        "animation-série" | "animation-serie" | "animation série" | "animation serie" => 2179,
        "concert" => 2180,
        "documentaire" => 2181,
        "emission-tv" | "émission-tv" | "emission tv" | "émission tv" => 2182,
        "film" => 2183,
        "série-tv" | "serie-tv" | "série tv" | "serie tv" => 2184,
        "spectacle" => 2185,
        "sport" => 2186,
        "vidéo-clips" | "video-clips" | "vidéo clips" | "video clips" => 2187,
        // Audio
        "audio" => 2139,
        "karaoké" | "karaoke" => 2147,
        "musique" => 2148,
        "podcast-radio" | "podcast radio" | "podcast/radio" => 2150,
        "samples" => 2149,
        // Application
        "application" => 2144,
        "application-autre" | "application autre" => 2177,
        "formation" | "application-formation" | "application formation" => 2176,
        "linux" | "application-linux" | "application linux" => 2171,
        "macos" | "application-macos" | "application macos" => 2172,
        "smartphone" | "application-smartphone" | "application smartphone" => 2174,
        "tablette" | "application-tablette" | "application tablette" => 2175,
        "windows" | "application-windows" | "application windows" => 2173,
        // Jeu vidéo
        "jeu vidéo" | "jeu-vidéo" | "jeu video" | "jeu-video" => 2142,
        "jeu-autre" | "jeu autre" => 2167,
        "jeu-linux" | "jeu linux" => 2159,
        "jeu-macos" | "jeu macos" => 2160,
        "microsoft" | "jeu-microsoft" | "jeu microsoft" => 2162,
        "nintendo" | "jeu-nintendo" | "jeu nintendo" => 2163,
        "jeu-smartphone" | "jeu smartphone" => 2165,
        "sony" | "jeu-sony" | "jeu sony" => 2164,
        "jeu-tablette" | "jeu tablette" => 2166,
        "jeu-windows" | "jeu windows" => 2161,
        // eBook
        "ebook" | "e-book" | "e book" => 2140,
        "ebook-audio" | "ebook audio" | "livre audio" | "audiobook" => 2151,
        "bd" | "bds" | "ebook-bd" | "ebook bd" => 2152,
        "comics" | "ebook-comics" | "ebook comics" => 2153,
        "livres" | "livre" | "ebook-livres" | "ebook livres" => 2154,
        "mangas" | "manga" | "ebook-mangas" | "ebook mangas" => 2155,
        "presse" | "ebook-presse" | "ebook presse" => 2156,
        // Other
        "imprimante-3d" | "imprimante 3d" | "3d" => 2200,
        "imprimante-3d-objets" | "objets 3d" => 2201,
        "imprimante-3d-personnages" | "personnages 3d" => 2202,
        "emulation" | "émulation" => 2141,
        "emulateurs" | "émulateurs" => 2157,
        "roms" => 2158,
        "gps" => 2143,
        "gps-applications" | "gps applications" => 2168,
        "gps-cartes" | "gps cartes" => 2169,
        "gps-divers" | "gps divers" => 2170,
        "xxx" => 2188,
        "xxx-films" | "xxx films" => 2189,
        "xxx-hentai" | "xxx hentai" => 2190,
        "xxx-images" | "xxx images" => 2191,
        _ => return None,
    };
    Some(id)
}

/// Reverse lookup: numeric category ID → Nostr #t tag value used in relay filters.
pub fn cat_id_to_nostr_tag(id: usize) -> Option<&'static str> {
    let tag = match id {
        2145 => "film/vidéo",
        2178 => "animation",
        2179 => "animation-série",
        2180 => "concert",
        2181 => "documentaire",
        2182 => "emission-tv",
        2183 => "film",
        2184 => "série-tv",
        2185 => "spectacle",
        2186 => "sport",
        2187 => "vidéo-clips",
        2139 => "audio",
        2147 => "karaoké",
        2148 => "musique",
        2150 => "podcast-radio",
        2149 => "samples",
        2144 => "application",
        2177 => "application-autre",
        2176 => "formation",
        2171 => "linux",
        2172 => "macos",
        2174 => "smartphone",
        2175 => "tablette",
        2173 => "windows",
        2142 => "jeu vidéo",
        2167 => "jeu-autre",
        2159 => "jeu-linux",
        2160 => "jeu-macos",
        2162 => "microsoft",
        2163 => "nintendo",
        2165 => "jeu-smartphone",
        2164 => "sony",
        2166 => "jeu-tablette",
        2161 => "jeu-windows",
        2140 => "ebook",
        2151 => "ebook-audio",
        2152 => "bds",
        2153 => "comics",
        2154 => "livres",
        2155 => "mangas",
        2156 => "presse",
        2200 => "imprimante-3d",
        2201 => "imprimante-3d-objets",
        2202 => "imprimante-3d-personnages",
        2141 => "emulation",
        2157 => "emulateurs",
        2158 => "roms",
        2143 => "gps",
        2168 => "gps-applications",
        2169 => "gps-cartes",
        2170 => "gps-divers",
        2188 => "xxx",
        2189 => "xxx-films",
        2190 => "xxx-hentai",
        2191 => "xxx-images",
        _ => return None,
    };
    Some(tag)
}

pub fn init_categories() -> Vec<Category> {
    vec![
        Category {
            id: 2145,
            name: "Film/Vidéo".to_string(),
            sub_categories: vec![
                cat(2178, "Animation"),
                cat(2179, "Animation Série"),
                cat(2180, "Concert"),
                cat(2181, "Documentaire"),
                cat(2182, "Emission TV"),
                cat(2183, "Film"),
                cat(2184, "Série TV"),
                cat(2185, "Spectacle"),
                cat(2186, "Sport"),
                cat(2187, "Vidéo-clips"),
            ],
        },
        Category {
            id: 2139,
            name: "Audio".to_string(),
            sub_categories: vec![
                cat(2147, "Karaoké"),
                cat(2148, "Musique"),
                cat(2150, "Podcast Radio"),
                cat(2149, "Samples"),
            ],
        },
        Category {
            id: 2144,
            name: "Application".to_string(),
            sub_categories: vec![
                cat(2177, "Autre"),
                cat(2176, "Formation"),
                cat(2171, "Linux"),
                cat(2172, "MacOS"),
                cat(2174, "Smartphone"),
                cat(2175, "Tablette"),
                cat(2173, "Windows"),
            ],
        },
        Category {
            id: 2142,
            name: "Jeu vidéo".to_string(),
            sub_categories: vec![
                cat(2167, "Autre"),
                cat(2159, "Linux"),
                cat(2160, "MacOS"),
                cat(2162, "Microsoft"),
                cat(2163, "Nintendo"),
                cat(2165, "Smartphone"),
                cat(2164, "Sony"),
                cat(2166, "Tablette"),
                cat(2161, "Windows"),
            ],
        },
        Category {
            id: 2140,
            name: "eBook".to_string(),
            sub_categories: vec![
                cat(2151, "Audio"),
                cat(2152, "Bds"),
                cat(2153, "Comics"),
                cat(2154, "Livres"),
                cat(2155, "Mangas"),
                cat(2156, "Presse"),
            ],
        },
        Category {
            id: 2200,
            name: "Imprimante 3D".to_string(),
            sub_categories: vec![cat(2201, "Objets"), cat(2202, "Personnages")],
        },
        Category {
            id: 2141,
            name: "Emulation".to_string(),
            sub_categories: vec![cat(2157, "Emulateurs"), cat(2158, "Roms")],
        },
        Category {
            id: 2143,
            name: "GPS".to_string(),
            sub_categories: vec![
                cat(2168, "Applications"),
                cat(2169, "Cartes"),
                cat(2170, "Divers"),
            ],
        },
        Category {
            id: 2188,
            name: "XXX".to_string(),
            sub_categories: vec![
                cat(2189, "Films"),
                cat(2190, "Hentai"),
                cat(2191, "Images"),
            ],
        },
    ]
}

fn cat(id: usize, name: &str) -> Category {
    Category {
        id,
        name: name.to_string(),
        sub_categories: vec![],
    }
}
