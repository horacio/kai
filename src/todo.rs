use serde::{Deserialize, Serialize};

use console::style;
use console::Term;

use crate::controls;
use crate::date;

#[derive(Debug, Serialize, Deserialize)]
pub struct Pomodoro {
    pub date: String,
    pub duration: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo<'a> {
    pub title: &'a str,
    pub date_started: &'a str,
    pub date_ended: &'a str,
    pub tasks: Vec<String>,
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

        let term = Term::stdout();

        term.clear_screen().unwrap();
        println!(
            "[{} {}] new",
            style("pomocli").yellow().bold(),
            style("todo").dim().yellow()
        );

        println!(
            "{}",
            style("++-----------------------------------------------------++")
                .yellow()
                .bold()
        );

        println!("Type task and click on the {} key to input a task,\nType the {} key to save all inputed tasks,\nType the {} key to cancel and quit",
                 style("enter").bold(),
                 style("s").bold(),
                 style("q").bold()
        );

        println!(
            "{}",
            style("++-----------------------------------------------------++")
                .yellow()
                .bold()
        );

        println!(
            "\n Enter tasks for session ({}): ",
            style(title.to_string()).bold()
        );
        controls::Ctrl::ctrl_todo(todo);
    }
}
