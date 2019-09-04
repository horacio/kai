use std::thread;
use std::time::Duration;

use console::style;
use console::Term;

use crate::controls::Ctrl;

pub fn countdown(count_from: i32) -> Result<(), ()> {
    let term = Term::stdout();

    let handle = thread::spawn(move || {
        for i in (1..=count_from).rev() {
            term.clear_screen().unwrap();
            println!("{}", style("FOCUS TIME").yellow().bold());
            println!("{}", style("++---------------++").yellow().bold());
            println!("{}", style("[COUNTDOWN STARTED]").red().dim());

            println!(
                "{}",
                style(format!("⏰ {}min remaining...", i)).blue().italic()
            );
            thread::sleep(Duration::from_millis(60));
        }
        println!("{}", style("[COUNTDOWN ENDED]").green().dim());
        println!("{}", style("++---------------++").yellow().bold());
    });

    handle.join().unwrap();

    Ctrl::ctrl_pomo();
    Ok(())
}
