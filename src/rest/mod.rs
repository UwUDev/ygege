use crate::rest::bench::*;
use crate::rest::categories::*;
use crate::rest::homepage::*;
use crate::rest::infos::*;
use crate::rest::search::*;
use crate::rest::torrent::*;
use crate::rest::user::*;
use actix_web::web;

mod bench;
mod categories;
mod homepage;
mod infos;
pub(crate) mod search;
mod torrent;
mod user;

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(categories)
        .service(ygg_search)
        .service(torrent_info)
        .service(download_torrent_direct)
        .service(download_torrent)
        .service(torrent_files)
        .service(get_user_info)
        .service(health_check)
        .service(status_check)
        .service(index)
        .service(bench_mark);
}
