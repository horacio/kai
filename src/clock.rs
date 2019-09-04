use std::thread;
use std::time::Duration;

use console::Term;
use console::style;

pub fn countdown(count_from: i32) -> Result<(), ()>{
    let term = Term::stdout();
    
    let handle = thread::spawn(move || {
        
        for i in (1..=count_from).rev() {
            term.clear_screen().unwrap();
            println!("{}", style("FOCUS").yellow());        
            println!("{}", style("---------------------").yellow());
            println!("{}", style("[COUNTDOWN STARTED]").red());
            
            println!("{}", style(format!("⏰ {}min remaining...", i)).blue());
            thread::sleep(Duration::from_millis(60000));
        }
        println!("{}", style("[COUNTDOWN ENDED]").green());
        println!("{}", style("---------------------").yellow());
    });

    handle.join().unwrap();
    Ok(())
}
