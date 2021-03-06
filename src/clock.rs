use console::style;
use console::Term;

use crate::controls::{ctrl_pomo, Ctrl};
use indicatif::{ProgressBar, ProgressStyle};
use std::thread;
use std::time::Duration;

pub fn countdown(count_from: u64, todo_name: String) -> Result<(), ()> {
    let term = Term::stdout();
    let bar = ProgressBar::new(count_from);

    bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .progress_chars("##-"),
    );

    term.clear_screen();
    let handle = thread::spawn(move || {
        println!(
            "{} {}min",
            style("[CLOCK started]").blue().dim(),
            count_from
        );

        for i in (1..=count_from).rev() {
            bar.inc(1); // Increment loader
            thread::sleep(Duration::from_secs(1));
        }
        println!(
            "{}",
            style("+--------------------------------------------------------+").blue()
        );
    });

    handle.join().unwrap();

    term.clear_screen();
    println!("{} {}min", style("[CLOCK ended]").blue().dim(), count_from);
    println!(
        "{}",
        style("+--------------------------------------------------------+").blue()
    );
    println!(
        " Type <{}> to save the pomodoro session",
        style("s").blue().bold()
    );
    println!(
        " Type <{}> key to save and check of a task from the todo",
        style("c").blue().bold()
    );
    println!(
        " Type <{}> key to cancel and quit",
        style("q").blue().bold()
    );
    println!(
        "{}",
        style("+--------------------------------------------------------+").blue()
    );

    ctrl_pomo(todo_name, count_from);

    Ok(())
}
