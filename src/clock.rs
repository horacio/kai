use std::thread;
use std::time::Duration;

use console::style;
use console::Term;

use crate::controls::Ctrl;

pub fn countdown(count_from: i32, todo: &str) -> Result<(), ()> {
    let term = Term::stdout();

    let handle = thread::spawn(move || {
        for i in (1..=count_from).rev() {
            term.clear_screen().unwrap();
            println!("{}", style("FOCUS TIME").yellow().bold());
            println!("{}", style("++---------------++").yellow().bold());
            println!("{}", style("[CLOCK STARTED]").red().dim());

            println!(
                "{}",
                style(format!("{} min remaining...", i)).blue().italic()
            );
            thread::sleep(Duration::from_millis(60));
        }
        println!("{}", style("[COUNTDOWN ENDED]").green().dim());
        println!("{}", style("++---------------++").yellow().bold());
    });

    handle.join().unwrap();

    // Menu after clock ends
    let term = Term::stdout();

    term.clear_screen().unwrap();

    println!(
        "\n[{} {}] stopped",
        style("sesh").yellow().bold(),
        style("clock").yellow().dim()
    );
    
    println!(
        "{}",
        style("++-----------------------------------------------------++")
            .yellow()
            .bold()
    );

    println!("Type the {} key to save repeat session,", style("r").bold());

    println!(
        "Type the {} key to check of a task from the todo,",
        style("c").bold()
    );
    
    println!("Type the {} key to cancel and quit", style("q").bold());

    println!(
        "{}",
        style("++-----------------------------------------------------++")
            .yellow()
            .bold()
    );

    Ctrl::ctrl_pomo(todo);
    
    Ok(())
}
