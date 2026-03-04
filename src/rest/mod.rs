use crate::rest::bench::*;
use crate::rest::categories::*;
use crate::rest::homepage::*;
use crate::rest::infos::*;
use crate::rest::search::*;
use crate::rest::torrent::*;
use actix_web::web;

mod bench;
mod categories;
mod homepage;
mod infos;
pub mod search;
mod torrent;

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(categories)
        .service(ygg_search)
        .service(download_torrent)
        .service(health_check)
        .service(status_check)
        .service(index)
        .service(bench_mark);
}
