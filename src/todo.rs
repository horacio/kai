use crate::utils::date::Date;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug)]
pub struct Todo {
    title: String,
    date: String,
    tasks: Vec<String>,
}

impl Todo {
    pub fn new(title: String) {
        let date = Date::now().unwrap().date;

        let mut todo = Todo {
            title: title,
            date: date.clone(),
            tasks: Vec::new(),
        };

        let mut user_input = String::new();
        println!("\n\niwah todo");
        println!("++------------------------------++\ntype and 'enter' to input a task\n's' to save all inputed tasks
'q' to cancel and quit\n++------------------------------++\n\n");

        loop {
            io::stdin()
                .read_line(&mut user_input)
                .expect("Failed to read line");

            match user_input.as_ref() {
                "\n" | "q\n" | "Q\n" => {
                    println!("Quit");
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
    let path = format!("kai/todo/{}.md", &todo.title);
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
        Ok(_) => println!("successfully wrote to {}", display),
    }
}
