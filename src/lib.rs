use std::error::Error;

pub mod clock;
pub mod controls;
pub mod date;
pub mod todo;

use clock::countdown;
use date::Date;

pub struct App {}

impl App {
    pub fn run(&self, input: controls::Ctrl) -> Result<(), Box<dyn Error>> {
        let title = input.cmd.clone();

        match (input.stage.as_ref(), input.cmd.as_ref()) {
            ("todo", "today") => todo::Todo::new(Date::now().unwrap().date),
            ("todo", title) => todo::Todo::new(String::from(title)),
            ("clock", "today") => countdown(5).unwrap(),
            (&_, &_) => (),
        }

        Ok(())
    }
}
