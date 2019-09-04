use std::thread;
use std::time::Duration;

pub fn countdown(count_from: i32) -> Result<(), ()>{
    let handle = thread::spawn(move || {
        for i in (1..=count_from).rev() {
            println!("{}sec", i);
            thread::sleep(Duration::from_millis(60000));
        }
    });

    handle.join().unwrap();
    Ok(())
}
