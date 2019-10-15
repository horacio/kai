use std::process::{Command, Stdio};

#[derive(Debug)]
pub struct Date {
    pub date: String,
}

impl Date {
    pub fn today() -> String {
        let getdate = Command::new("/bin/date")
            .arg("-I")
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to execute child");

        let output = getdate.wait_with_output().expect("failed to wait on child");

        let mut date = String::from_utf8(output.stdout).expect("Error getting todays date");
        date = String::from(date.trim());

        date
    }
}
