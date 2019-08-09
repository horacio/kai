use std::error::Error;

pub mod controls;
pub mod todo;
pub mod utils;

use utils::date::Date;

pub fn run(input: controls::Ctrl) -> Result<(), Box<dyn Error>> {

    let title = input.cmd.clone();
    
    match (input.stage.as_ref(), input.cmd.as_ref()) {
        ("todo", "today") => {
            todo::Todo::new(Date::now().unwrap().date);
        },
        ("todo", title) => {
            todo::Todo::new(String::from(title));
        },
        ("start", "today") => {
            
        },
        (&_, &_) => (),
    }

    Ok(())
}
