use std::error::Error;

pub mod clock;
pub mod date;
pub mod controls;
pub mod todo;

use clock::countdown;
use date::Date;

const POMO_DEFAULT_TIME: i32 = 25;

pub struct App {}

impl App {
    pub fn run(&self, input: controls::Ctrl) -> Result<(), Box<dyn Error>> {
        match (input.stage.as_ref(), input.cmd.as_ref()) {
            ("todo", "today") => todo::Todo::new(&Date::today()),
            ("todo", title) => todo::Todo::new(title),
            ("clock", "today") => countdown(POMO_DEFAULT_TIME, &Date::today()).unwrap(),
            (&_, &_) => (),
        }

        Ok(())
    }
}
