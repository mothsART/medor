#[macro_use]
extern crate clap;
extern crate desktopd;

use std::env;
use std::io::stdin;
use std::process::Command;

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

fn launch_app(result: &SearchResult) {
    println!("{:?}", result);
    //if 
    //Command::new(result.).output();
}

fn prompt(title: &str, results: &Vec<Option<SearchResult>>, len: usize) {
    if len == 1 {
        let first_r = results.first();
        if let Some(Some(r)) = &first_r {
            let title = &r.title;
            let c = r.comment.as_ref();
            println!(
                "Launch \"{}\" : {} [yn]?", 
                &title, &c.unwrap_or(&"".to_string())
            );
            let mut input = String::new();
            let read_input = stdin().read_line(&mut input);
            if let Ok(input_value) = input.trim().parse::<char>() {
                if input_value == 'y' {
                    launch_app(r);
                } else if input_value != 'n' {
                    prompt("", results, len);
                }
            }
        } else {
            println!("No results");
        }
        return;
    }
    println!("{}", title);
    let mut inc = 0;
    for result in results {
        if let Some(r) = result {
            let c = r.comment.as_ref();
            println!("{}) {} :  \"{}\"", inc, r.title, &c.unwrap_or(&"".to_string()));
            inc += 1;
        }
    }
    let mut input = String::new();
    let read_input = stdin().read_line(&mut input);
    
    if let Ok(input_value) = input.trim().parse::<usize>() {
        //if let Some(trim_value) = input_value.trim() {
        println!("yolo {}", input_value);
        //}
        /*match i.trim().parse::<usize>() {
            Ok(value) if value > 0 && value <= len => {
                /*let program = kill_list.get(value - 1).unwrap();
                let killall_cmd = Command::new("killall").arg(program).output();
                match killall_cmd {
                    Ok(_v) => {
                        println!("\"{}\" has been killed", program);
                    }
                    Err(_e) => {
                        eprintln!("{} did'nt succeed in killing {}", crate_name!(), program);
                    }
                }
                */
            }
            Ok(input) => prompt(&format!("Wrong input \"{}\". Retry :", input), results),
            Err(_e) => prompt(&format!("Wrong input \"{}\". Retry :", input), results),
        };*/
    } else {
        return prompt("Wrong input \"a\". Retry :", results, len);
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
                println!("{}", len);
                prompt("Choose application to launch :", r, len);
            }
        } else {
            println!("No results");
        }
    }
}
