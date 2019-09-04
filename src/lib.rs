use std::error::Error;

pub mod controls;
pub mod sesh;
pub mod date;
pub mod clock;

use date::Date;
use clock::countdown;

pub struct App{}

impl App{
    pub fn run(&self, input: controls::Ctrl) -> Result<(), Box<dyn Error>> {

        let title = input.cmd.clone();
        
        match (input.stage.as_ref(), input.cmd.as_ref()) {
            ("sesh", "today") => sesh::Sesh::new(Date::now().unwrap().date),
            ("sesh", title) => sesh::Sesh::new(String::from(title)),
            ("clock", "today") => countdown(5).unwrap(),
            (&_, &_) => (),
        }

        Ok(())
    }
}
