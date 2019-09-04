use chrono::{Date, Local};
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

use console::style;
use console::Term;

#[derive(Debug)]
pub struct Todo<'a> {
    title: &'a str,
    date: &'a str,
    tasks: Vec<String>,
}

impl<'a> Todo<'a> {
    pub fn new(title: &'a str) {
        let date: Date<Local> = Local::now().date();

        let mut todo = Todo {
            title: title,
            date: &date.to_string(),
            tasks: Vec::new(),
        };

        let mut user_input = String::new();
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
        loop {
            io::stdin()
                .read_line(&mut user_input)
                .expect("Failed to read line");

            match user_input.as_ref() {
                "\n" | "q\n" | "Q\n" => {
                    println!("{}", style("Quit").red());
                    break;
                }
                "s\n" | "S\n" => {
                    create_todo_file(&todo);
                    break;
                }
                _ => {
                    todo.tasks.push(user_input.trim().to_string());
                }
            }

            user_input = String::new();
        }
    }
}

fn create_todo_file(todo: &Todo) {
    let path = format!("iwah/todo/{}.md", &todo.title);
    let path = Path::new(&path);
    let display = path.display();
    let mut todo_output = format!("# Todo {}\n{}", todo.title, todo.date);

    for task in &todo.tasks {
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
