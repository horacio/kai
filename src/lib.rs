use std::error::Error;
extern crate chrono;

pub mod clock;
pub mod controls;
pub mod todo;

use clock::countdown;
use chrono::{Local, Date};

pub struct App {}

impl App {
    pub fn run(&self, input: controls::Ctrl) -> Result<(), Box<dyn Error>> {
        let title = input.cmd.clone();

        match (input.stage.as_ref(), input.cmd.as_ref()) {
            ("todo", "today") => {
                let date :Date<Local> = Local::today();
                todo::Todo::new(&date.to_string());
            }
            ("todo", title) => todo::Todo::new(title),
            ("clock", "today") => countdown(5).unwrap(),
            (&_, &_) => (),
        }

        Ok(())
    }
}
