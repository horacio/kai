use serde_json::Result;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use console::style;

use crate::todo::Todo;

pub struct Database {}

// Handles JSON disk storage and querying
impl Database {
    // generate json and store to disk
    pub fn store(todo: &Todo) -> Result<()> {
        let json_string = serde_json::to_string(&todo)?;
        save_to_disk(todo.title, &json_string);
        Ok(())
    }
}

// Save json file to disk
fn save_to_disk(filename: &str, json_string: &str) {
    let path = format!("db/{}.json", filename);
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
