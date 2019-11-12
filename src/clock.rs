use crate::controls::{Ctrl, ctrl_pomo};
use std::thread;
use std::time::Duration;

pub fn countdown(count_from: u64, todo_name: &str) -> Result<(), ()> {
    let handle = thread::spawn(move || {
        println!("[CLOCK started] {}min", count_from);
        for i in (1..=count_from).rev() {
            println!(" {} min remaining...", i);
            thread::sleep(Duration::from_secs(1));
        }
    });

    handle.join().unwrap();

    println!("\n[CLOCK ended]");
    println!("+--------------------------------------------------------+");
    println!(" Type <s> to save the pomodoro session");
    println!(" Type <c> key to save and check of a task from the todo");
    println!(" Type <q> key to cancel and quit");
    println!("+--------------------------------------------------------+");

    ctrl_pomo(todo_name, count_from);

    Ok(())
}
