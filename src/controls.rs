pub struct Ctrl {
    pub stage: String,
    pub cmd: String,
}

impl Ctrl {
    pub fn new(args: &[String]) -> Result<Ctrl, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let stage = args[1].clone();
        let cmd = args[2].clone();

        Ok(Ctrl { stage, cmd })
    }
}
