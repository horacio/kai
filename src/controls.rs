use crate::todo::Todo;
use std::io;

use console::style;

pub struct Ctrl {
    pub stage: String,
    pub cmd: String,
}

impl Ctrl {
    pub fn new(args: &[String]) -> Result<Ctrl, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let stage = args[1].clone();
        let cmd = args[2].clone();

        Ok(Ctrl { stage, cmd })
    }

    pub fn ctrl_todo(mut todo: Todo) {
        let mut user_input = String::new();
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
                    todo.create_todo_file();
                    break;
                }
                _ => {
                    todo.tasks.push(user_input.trim().to_string());
                }
            }

            user_input = String::new();
        }
    }

    pub fn ctrl_pomo() {}
}
