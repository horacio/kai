use std::thread;
use std::time::Duration;
use crate::controls::Ctrl;

pub fn countdown(count_from: u64, todo_name: &str) -> Result<(), ()> {
    let handle = thread::spawn(move || {
        for i in (1..=count_from).rev() {
            println!("FOCUS TIME");
            println!("++---------------++");
            println!("[CLOCK STARTED]");
            println!("{} min remaining...", i);
            thread::sleep(Duration::from_secs(1));
        }
        println!("[COUNTDOWN ENDED]");
        println!("++---------------++");
    });

    handle.join().unwrap();

    println!("\n[pomocli clock] stopped");
    println!("++-----------------------------------------------------++");
    println!("Type the [s] key to save the pomodoro session,");
    println!("Type the [c] key to check of a task from the todo,");
    println!("Type the [q] key to cancel and quit");
    println!("++-----------------------------------------------------++");

    Ctrl::ctrl_pomo(todo_name, count_from);

    Ok(())
}
