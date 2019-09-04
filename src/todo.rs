use chrono::{Date, Local};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use console::style;
use console::Term;

use crate::controls;

#[derive(Debug)]
pub struct Todo<'a> {
    pub title: &'a str,
    pub date: &'a str,
    pub tasks: Box<Vec<String>>,
}

impl<'a> Todo<'a> {
    pub fn new(title: &'a str) {
        let date: Date<Local> = Local::now().date();

        let mut todo = Todo {
            title: title,
            date: &date.to_string(),
            tasks: Box::new(Vec::new()),
        };

        let term = Term::stdout();

        term.clear_screen().unwrap();
        println!("\n{}", style("[TODO]").yellow().bold());
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
            "\n📝 Enter tasks for session ({}): ",
            style(format!("{}", title)).bold()
        );
        controls::Ctrl::ctrl_todo(todo);
    }

    pub fn create_todo_file(&self) {
        let path = format!("iwah/todo/{}.md", self.title);
        let path = Path::new(&path);
        let display = path.display();
        let mut todo_output = format!("# Todo {}\n{}", self.title, self.date);

        for task in self.tasks.iter() {
            todo_output = format!("{}\n- [ ] {}", todo_output, task);
        }

        todo_output = format!("{}\n", todo_output);

        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why.description()),
            Ok(file) => file,
        };

        match file.write_all(todo_output.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
            Ok(_) => println!(
                "{}",
                style(format!("\n💾 successfully wrote to {}\n", display)).green()
            ),
        }
    }
}
