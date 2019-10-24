use dirs;
use serde_json::{Result, Value};
use std::error::Error;
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::Path;

use console::style;

use crate::date;
use crate::todo::{Pomodoro, Todo};

pub struct Database {}

// Handles JSON disk storage and querying
impl Database {
    // generate json and store to disk
    pub fn store(todo: &Todo) -> Result<()> {
        let json_string = serde_json::to_string(&todo)?;
        save_to_disk(todo.title, &json_string);
        Ok(())
    }

    pub fn save_pomo(todo_name: &str, duration: u64) -> Result<()> {
        let home = dirs::home_dir().unwrap();
        let date = date::Date::today();

        let contents = fs::read_to_string(format!("{}/.pomocli/{}.json", home.display(), todo_name))
            .expect("Something went wrong reading Todo  file");

        let mut todo: Todo =
            serde_json::from_str(contents.as_str()).expect("Todo is not a valid json value");

        todo.pomodoros.push(Pomodoro {
            date: String::from(&date.to_string()),
            duration,
        });

        todo.total_time_spend = todo.total_time_spend + duration;

        let json_string = serde_json::to_string(&todo)?;

        save_to_disk(todo_name, &json_string);
        Ok(())
    }
}

// Save json file to disk
fn save_to_disk(filename: &str, json_string: &str) {
    let home = dirs::home_dir().unwrap();
    let path = format!("{}/.pomocli/{}.json", home.display(), filename);
    let path = Path::new(&path);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };

    match file.write_all(json_string.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
        Ok(_) => println!(
            "{}",
            style(format!("\n successfully wrote to {}\n", display)).green()
        ),
    }
}
