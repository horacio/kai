use crate::db::Database;
use crate::todo::Todo;

pub fn basic(todo_name: &str) {
    let json_string = Database::get_todo(todo_name);

    let todo: Todo =
        serde_json::from_str(json_string.as_ref()).expect("Todo is not a valid json value");

    println!("{}", gen_markdown(todo));
}

pub fn gen_markdown(todo: Todo) -> String {
    let mut count = 1;
    let mut output = format!("# {}\n\n", todo.title);

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
