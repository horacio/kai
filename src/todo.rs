use serde::{Deserialize, Serialize};

use crate::controls;
use crate::date;

#[derive(Debug, Serialize, Deserialize)]
pub struct Pomodoro {
    pub date: String,
    pub duration: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub title: String,
    pub checked: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo<'a> {
    pub title: &'a str,
    pub date_started: &'a str,
    pub date_ended: &'a str,
    pub tasks: Vec<Task>,
    pub pomodoros: Vec<Pomodoro>,
    pub total_time_spend: u64,
}

impl<'a> Todo<'a> {
    pub fn new(title: &'a str) {
        let date = date::Date::today();

        let todo = Todo {
            title,
            date_started: &date.to_string(),
            date_ended: "Ongoing",
            tasks: Vec::new(),
            pomodoros: Vec::new(),
            total_time_spend: 0,
        };

        println!("[TODO]");
        println!("+-------------------------------------------------+");
        println!(" Type task and <enter> to add it to the todo list");
        println!(" Type <s> to save the todo list");
        println!(" Type <q> to cancel and quit");
        println!("+-------------------------------------------------+");
        println!("Enter tasks for [{}]: ", title.to_string());

        controls::Ctrl::ctrl_todo(todo);
    }
}
