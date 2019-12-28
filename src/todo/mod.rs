mod parser;

use console::style;
use console::Term;

use serde::{Deserialize, Serialize};

use crate::controls::{ctrl_todo, Ctrl};
use crate::date;

pub use parser::parse;

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
        let term = Term::stdout();
        let date = date::Date::today();

        let todo = Todo {
            title,
            date_started: &date.to_string(),
            date_ended: "Ongoing",
            tasks: Vec::new(),
            pomodoros: Vec::new(),
            total_time_spend: 0,
        };

        term.clear_screen();
        println!("{}", style("[TODO]").blue().dim());
        println!(
            "{}",
            style("+-----------------------------------------+").blue()
        );
        println!(
            " Type task and <{}> to add it to the todo list",
            style("enter").blue().bold()
        );
        println!(" Type <{}> to save the todo list", style("s").blue().bold());
        println!(" Type <{}> to cancel and quit", style("q").blue().bold());
        println!(
            "{}",
            style("+-----------------------------------------+").blue()
        );
        println!(
            "Enter tasks for [{}]: ",
            style(format!("{}", title.to_string())).blue()
        );

        ctrl_todo(todo);
    }
}
