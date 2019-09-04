use std::env;
use std::process;

use iwah::controls::Ctrl;

fn main() {
    let args: Vec<String> = env::args().collect();
    let cmd = Ctrl::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let app = iwah::App {};

    if let Err(e) = app.run(cmd) {
        eprintln!("Application error {}", e);
        process::exit(1);
    }
}
