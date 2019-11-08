use crate::db::Database;
use crate::todo::Todo;

pub fn all() {
    for entry in Database::get_all() {
        let todo: Todo =
            serde_json::from_str(entry.as_ref()).expect("Todo is not a valid json value");

        println!("{}", log_all(todo));
    }
}

pub fn basic(todo_name: &str) {
    let json_string = Database::get_todo(todo_name);

    let todo: Todo =
        serde_json::from_str(json_string.as_ref()).expect("Todo is not a valid json value");

    println!("{}", gen_markdown(todo));
}

pub fn gen_markdown(todo: Todo) -> String {
    let mut count = 1;
    let mut output = format!("\n# {}\n\n", todo.title);

    output = format!("{}## Tasks: \n", output);
    for task in todo.tasks {
        if task.checked {
            output = format!("{}{}. [X] {}\n", output, count, task.title);
        } else {
            output = format!("{}{}. [ ] {}\n", output, count, task.title);
        }

        count = count + 1;
    }

    output = format!("{}\n------------------------\n", output);
    output = format!("{}Pomodoros: {}\n", output, todo.pomodoros.len());
    output = format!("{}Date Started: {}\n", output, todo.date_started);
    output = format!("{}Date Ended: {}\n", output, todo.date_ended);
    output = format!("{}Total Time Spend: {}min\n", output, todo.total_time_spend);

    output
}

pub fn log_all(todo: Todo) -> String {
    let output = format!(
        "#{}\n- Tasks: {}\n- Pomodoros: {}\n- Date Started: {}\n- Date Ended: {}\n- Total Time Spend: {}\n",
        todo.title,
        todo.tasks.len(),
        todo.pomodoros.len(),
        todo.date_started,
        todo.date_ended,
        todo.total_time_spend
    );

    output
}
