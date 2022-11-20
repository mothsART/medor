#[macro_use]
extern crate clap;
extern crate desktopd;

use std::env;

use desktopd::db::basic::{Db, DesktopDDb};
use desktopd::db::search::SearchDb;

use medor::cli::build_cli;

fn get_local() -> String {
    if let Ok(_l) = env::var("LANG") {
        return _l;
    }
    "en_EN.UTF-8".to_string()
}

fn main() {
    let matches = build_cli(crate_name!(), crate_version!()).get_matches();
    let mut search_db = DesktopDDb::new();

    if let Some(keyword_search) = matches.get_one::<String>("app") {
        let results = search_db.get(keyword_search, &get_local());

        println!("{:?}", results);
        /*
        for r in results {
            println!("{:?}", r.0);
        }*/
    }
}
