use std::error::Error;
extern crate chrono;

pub mod clock;
pub mod controls;
pub mod todo;

use chrono::{Local};
use clock::countdown;

const POMO_DEFAULT_TIME :i32 = 25;

pub struct App {}

impl App {
    pub fn run(&self, input: controls::Ctrl) -> Result<(), Box<dyn Error>> {

        match (input.stage.as_ref(), input.cmd.as_ref()) {
            ("todo", "today") => todo::Todo::new(&Local::today().to_string()),
            ("todo", title) => todo::Todo::new(title),
            ("clock", "today") => countdown(POMO_DEFAULT_TIME).unwrap(),
            (&_, &_) => (),
        }

        Ok(())
    }
}
