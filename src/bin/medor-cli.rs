#[macro_use]
extern crate clap;
extern crate desktopd;

use std::env;

use desktopd::db::basic::{DesktopDDb, Db};
use desktopd::db::search::SearchDb;

use medor::cli::build_cli;

fn main() {
    let matches =
        build_cli(crate_name!(), crate_version!()).get_matches();
    let locale = env::var("LANG").unwrap();
    //println!("{}", locale);
    let search_db = DesktopDDb::new();
    search_db.get("nav", &locale);

/*
    for r in results.unwrap() {
        println!("{:?}", r.0);
    }
*/
    println!("Hello, world!");
}
