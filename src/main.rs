extern crate clap;
extern crate dirs;

use clap::{Arg, App, SubCommand};
use std::error::Error;
use std::fs;

pub mod clock;
pub mod controls;
pub mod date;
pub mod db;
pub mod errors;
pub mod log;
pub mod todo;

const KAI_DEFAULT_TIME: u64 = 1; // 1 KAI = 25 minutes

fn main() {
    let commands = App::new("kai")
        .version("1.0")
        .author("cy6erlion <cy6erlion@protonmail.com>")
        .about("Time management system.")
        .arg(Arg::with_name("config")
             .short("c")
             .long("config")
             .value_name("FILE")
             .help("Sets a custom config file")
             .takes_value(true))
        .subcommand(SubCommand::with_name("todo")
                    .name("todo")
                    .about("Create a new todo")
                    .arg(Arg::with_name("input")
                         .short("i")
                         .long("input")
                         .takes_value(true)
                         .value_name("INPUT")
                         .default_value("today")
                         .number_of_values(1)))
        .subcommand(SubCommand::with_name("clock")
                    .name("clock")
                    .about("Countdown timer (defaults to 1 KAI (25min))")
                    .arg(Arg::with_name("input")
                         .short("i")
                         .long("input")
                         .takes_value(true)
                         .value_name("INPUT")
                         .default_value("today")
                         .number_of_values(1)))
        .subcommand(SubCommand::with_name("log")
                    .name("log")
                    .about("Analyse and track your work habbits")
                    .arg(Arg::with_name("input")
                         .short("i")
                         .long("input")
                         .takes_value(true)
                         .value_name("INPUT")
                         .default_value("all")
                         .number_of_values(1)))
        .get_matches();
    
    match commands.subcommand() {
        ("todo",  Some(todo)) => {
            match todo.value_of("input") {
                Some("today") => todo::Todo::new(date::Date::today()),
                Some(title) => {
                    if title.ends_with(".md") {
                        let parsed_todo = todo::parse(title);
                    } else {
                        todo::Todo::new(String::from(title))
                    }                
                }
                _ => {},
            }
        }
        
        ("clock", Some(clock)) => {
            match clock.value_of("input") {
                Some("today") => {
                    if let Err(e) = detect_todo(&date::Date::today()) {
                        println!("{}", e);
                    } else {
                        clock::countdown(KAI_DEFAULT_TIME, date::Date::today()).unwrap();
                    }
                },
                Some(title) => {
                    if let Err(e) = detect_todo(title) {
                        println!("{}", e);
                    } else {
                        clock::countdown(KAI_DEFAULT_TIME, title.to_string()).unwrap();
                    }
                },
                _ => {},
            }
        },

        ("log", Some(log)) => {
            match log.value_of("input") {
                Some("all") => {
                    log::all();
                },
                Some("today") => {
                    if let Err(e) = detect_todo(&date::Date::today()) {
                        println!("{}", e);
                    } else {
                        log::basic(&date::Date::today());
                    }
                },
                Some(title) => {
                    if let Err(e) = detect_todo(title) {
                        println!("{}", e);
                    } else {
                        log::basic(title);
                    }
                },
                _ => {},
            }
        },
        _ => {},
    }
}

// check if todo file exists
fn detect_todo(filename: &str) -> Result<(), Box<dyn Error>> {
    let kai_store = format!("{}/.kai", dirs::home_dir().unwrap().display());
    let entries = fs::read_dir(kai_store).unwrap();
    let file = format!("{}.json", filename);

    for entry in entries {
        if file == entry.unwrap().file_name().to_str().unwrap() {
            return Ok(());
        }
    }

    return Result::Err(Box::new(errors::TodoErr(
        "Todo entry does not exist".into(),
    )));
}
