#[macro_use]
extern crate clap;
extern crate desktopd;

use std::env;
use std::io::stdin;
use std::process::{Command, Stdio};

use desktopd::db::basic::{Db, DesktopDDb};
use desktopd::db::search::SearchDb;
use desktopd::desktop::FIELDS_CODE;
use desktopd::models::SearchResult;

use medor::cli::build_cli;

fn get_local() -> String {
    if let Ok(_l) = env::var("LANG") {
        return _l;
    }
    "en_EN.UTF-8".to_string()
}

fn launch_app(result: &SearchResult) {
    let mut exec_cmd = result.exec.clone();
    for field in FIELDS_CODE {
        exec_cmd = exec_cmd.replace(&format!("%{field}"), "");
    }
    Command::new(exec_cmd.trim())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap_or_else(|_| panic!("\"{}\" failed to start", exec_cmd.trim()));
}

fn prompt(title: &str, results: &Vec<SearchResult>, len: usize) {
    if len == 1 {
        let first_r = results.first();
        if let Some(r) = &first_r {
            let title = &r.title;
            println!("Launch \"{}\" {} : [yn]?", &title, r.comment);
            let mut input = String::new();
            let _read_input = stdin().read_line(&mut input);
            if let Ok(input_value) = input.trim().parse::<char>() {
                if input_value == 'y' {
                    launch_app(r);
                } else if input_value != 'n' {
                    prompt(
                        &format!("Wrong input \"{input_value}\". Retry :"),
                        results,
                        len,
                    );
                }
            }
        } else {
            println!("No results");
        }
        return;
    }
    println!("{title}");
    for (i, result) in results.iter().enumerate() {
        println!("{}) : {} \"{}\"", i, result.title, result.comment);
    }
    let mut input = String::new();
    let _read_input = stdin().read_line(&mut input);

    if let Ok(input_value) = input.trim().parse::<usize>() {
        if input_value >= len {
            return prompt(
                &format!("Wrong input \"{input_value}\". Retry :"),
                results,
                len,
            );
        }
        if let Some(r) = results.get(input_value) {
            launch_app(r);
        }
    } else {
        prompt(
            &format!("Wrong input \"{}\". Retry :", input.trim()),
            results,
            len,
        );
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
            let len = r.len();
            if len == 0 {
                println!("No results");
            } else {
                prompt("Choose an application to launch :", r, len);
            }
        } else {
            println!("No results");
        }
    }
}
