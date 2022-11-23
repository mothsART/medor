#[macro_use]
extern crate clap;
extern crate desktopd;

use std::env;

use desktopd::models::SearchResult;
use desktopd::db::basic::{Db, DesktopDDb};
use desktopd::db::search::SearchDb;

use medor::cli::build_cli;

fn get_local() -> String {
    if let Ok(_l) = env::var("LANG") {
        return _l;
    }
    "en_EN.UTF-8".to_string()
}

fn choice(results: &Vec<Option<SearchResult>>, len: usize) {
    println!("nb results : {}", len);
    for r in results {
        println!("{:?}", r);
    }
}

fn main() {
    let matches = build_cli(crate_name!(), crate_version!()).get_matches();
    let mut debug_value = false;
    if let Some(d) = matches.get_one::<bool>("debug") {
        debug_value = *d;
    }

    let mut search_db = DesktopDDb::new(debug_value);

    if let Some(keyword_search) = matches.get_one::<String>("app") {
        let results = search_db.get(keyword_search, &get_local(), 10);

        if let Ok(ref r) = results {
            if r.len() == 0 {
                println!("No results");
            } else {
                choice(r, r.len());
            }
        } else {
            println!("No results");
        }
    }
}
