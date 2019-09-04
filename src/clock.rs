use std::thread;
use std::time::Duration;

pub fn countdown(count_from: i32) -> Result<(), ()>{
    let handle = thread::spawn(move || {
        println!("[⏰ COUNTDOWN STARTED]");
                 
        for i in (1..=count_from).rev() {
            println!("{}min remaining...", i);
            thread::sleep(Duration::from_millis(60000));
        }
        println!("[⏰ COUNTDON ENDED]");
    });

    handle.join().unwrap();
    Ok(())
}
