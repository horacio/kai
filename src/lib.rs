extern crate dirs;

use std::error::Error;
use std::fs;

pub mod clock;
pub mod controls;
pub mod date;
pub mod db;
pub mod todo;
pub mod errors;

const POMO_DEFAULT_TIME: u64 = 1500; // 1500 seconds = 25 minutes

pub struct App {}

impl App {
    pub fn run(&self, input: controls::Ctrl) -> Result<(), Box<dyn Error>> {
        match (input.stage.as_ref(), input.cmd.as_ref()) {
            ("todo", "today") => todo::Todo::new(&date::Date::today()),
            ("todo", title) => todo::Todo::new(title),
            ("clock", "today") => {                

                if let Err(e) = detect_todo(&date::Date::today()) {
                    println!("{}", e);
                } else {
                    clock::countdown(POMO_DEFAULT_TIME, &date::Date::today()).unwrap();
                }
            }
            (&_, &_) => (),
        }

        Ok(())
    }
}

// check if todo file exists
fn detect_todo(filename: &str) -> Result<(), Box<dyn Error>>{
    let sesh_store = format!("{}/.sesh", dirs::home_dir().unwrap().display());
    let entries = fs::read_dir(sesh_store).unwrap();
    let file = format!("{}.json", filename);
    
    for entry in entries {
        if file == entry.unwrap().file_name().to_str().unwrap(){
            return Ok(())
        } 
    };

    return Result::Err(Box::new(errors::TodoErr("Todo entry does not exist".into())))
}
