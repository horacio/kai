use std::fs;
use std::str;
use std::error::Error;
use crate::date;
use crate::errors;

use super::{Todo, Task};

pub fn parse(filename: &str) -> Result<Todo, Box<dyn Error>>{
    let file_bytes = fs::read(filename).expect("file not found");
    let contents = String::from_utf8(file_bytes).expect("File contains invalid utf characters");
    let mut lines = contents.lines();
    let date = date::Date::today();
    let mut title = "";
    let mut line_count = 1;
    let mut tasks_index = 0;
    let mut tasks = vec![];
    
    for line in lines {
        // Get todo title from markdown title
        if line.starts_with("# ") {
            title = &line[2..];
        }

        // Set tasks index
        if line.starts_with("## Tasks:") {
            tasks_index = line_count;
        }

        // get tasks
        if tasks_index != 0 && line_count > tasks_index {
            let word_start = line.find("] ").expect("Not a valid task");
            let task = &line[word_start + 2..];
            tasks.push(Task{title: task.to_string(), checked: false});
        }
        
        line_count = line_count + 1;
    }

    if title != ""
        && tasks_index != 0
        && tasks.len() > 0 {            
            Ok(Todo{
                title: String::from(title),
                date_started: date,
                date_ended: String::from("Ongoing"),
                tasks: tasks,
                pomodoros: Vec::new(),
                total_time_spend: 0,                
            })
        } else {
            Result::Err(Box::new(errors::TodoErr(
                "Unable to parse Todo Markdown file".into(),
            )))
        }       
}
