use crate::db::Database;
use crate::todo::{Task, Todo};

use std::io;

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

}

pub fn ctrl_todo(mut todo: Todo) {
    let mut user_input = String::new();
    loop {
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        match user_input.as_ref() {
            "q\n" | "Q\n" => {
                println!("Quit");
                break;
            }
            "s\n" | "S\n" | "\n" => {
                Database::store(&todo).unwrap();
                break;
            }
            _ => {
                todo.tasks.push(Task {
                    title: user_input.trim().to_string(),
                    checked: false,
                });
            }
        }

        user_input = String::new();
    }
}

pub fn ctrl_pomo(todo: &str, duration: u64) {
    let mut user_input = String::new();
    loop {
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        match user_input.as_ref() {

            // Save pomodoro session
            "s\n" => {
                Database::save_pomo(todo, duration);
                break;
            }

            // Check of a task
            "c\n" => {
                let json_string = Database::get_todo(todo);                    
                let todo: Todo =
                    serde_json::from_str(json_string.as_ref()).expect("Todo is not a valid json value");
                let mut count = 1;
                
                println!("\n[CHECK tasks]");
                println!("+--------------------------------------------------------+");
                println!(" Enter the task number to check/uncheck a task");                    
                for task in &todo.tasks {
                    if task.checked {
                        println!("  {}. [x] {}", count, task.title);
                    }
                    else {
                        println!("  {}. [ ] {}", count, task.title);
                    }
                    
                    count = count + 1;
                }                    
                println!("+--------------------------------------------------------+");
                println!("Task number: ");

                let mut task_number = String::new();
                
                io::stdin()
                    .read_line(&mut task_number)
                    .expect("Failed to read line");

                let task_number = task_number.trim();
                let number_of_tasks = todo.tasks.len();
                let todo_name = todo.title;
                
                match task_number.parse::<usize>() {
                    Ok(i) => {
                        if i <= 0 || i > number_of_tasks {
                            panic!("Task number {} not found", i);
                        } else {
                            Database::check_task(todo, i-1).unwrap();
                            Database::save_pomo(todo_name, duration);
                        }
                    }
                    Err(..) => panic!("Invalid task number: {}", task_number),
                };
                break;
            }
            
            _ => break
        }

        user_input = String::new();
    }
}
