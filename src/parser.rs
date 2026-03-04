use crate::search::{Order, Sort};
use serde::Serialize;
use serde_json::Value;
use std::cmp::PartialEq;

#[derive(Debug, Serialize, Clone, Eq, Hash, PartialEq)]
pub struct Torrent {
    pub id: String,
    pub category_id: usize,
    pub name: String,
    pub age_stamp: usize,
    pub size: u64,
    pub completed: usize,
    pub seed: usize,
    pub leech: usize,
    pub magnet: String,
    pub link: String,
    pub file_count: usize,
}

impl PartialEq for Order {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Order::Ascending, Order::Ascending) | (Order::Descending, Order::Descending)
        )
    }
}

impl Torrent {
    pub fn to_json(&self) -> Value {
        serde_json::to_value(self).unwrap()
    }

    pub fn sort(torrents: &mut Vec<Torrent>, sort: Option<Sort>, order: Option<Order>) {
        let sort = sort.unwrap_or(Sort::PublishDate);
        let order = order.unwrap_or(Order::Descending);

        match sort {
            Sort::Name => {
                if order == Order::Ascending {
                    torrents.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
                } else {
                    torrents.sort_by(|a, b| b.name.to_lowercase().cmp(&a.name.to_lowercase()));
                }
            }
            Sort::Seed => {
                if order == Order::Ascending {
                    torrents.sort_by(|a, b| a.seed.cmp(&b.seed));
                } else {
                    torrents.sort_by(|a, b| b.seed.cmp(&a.seed));
                }
            }
            Sort::Comments => {
                // No comments in Nostr events; fall back to date
                if order == Order::Ascending {
                    torrents.sort_by(|a, b| a.age_stamp.cmp(&b.age_stamp));
                } else {
                    torrents.sort_by(|a, b| b.age_stamp.cmp(&a.age_stamp));
                }
            }
            Sort::PublishDate => {
                if order == Order::Ascending {
                    torrents.sort_by(|a, b| a.age_stamp.cmp(&b.age_stamp));
                } else {
                    torrents.sort_by(|a, b| b.age_stamp.cmp(&a.age_stamp));
                }
            }
            Sort::Completed => {
                if order == Order::Ascending {
                    torrents.sort_by(|a, b| a.completed.cmp(&b.completed));
                } else {
                    torrents.sort_by(|a, b| b.completed.cmp(&a.completed));
                }
            }
            Sort::Leech => {
                if order == Order::Ascending {
                    torrents.sort_by(|a, b| a.leech.cmp(&b.leech));
                } else {
                    torrents.sort_by(|a, b| b.leech.cmp(&a.leech));
                }
            }
        }
    }
}
