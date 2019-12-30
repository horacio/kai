use std::fs;

pub fn parse(filename: &str){
    let file = fs::read(filename).expect("file not found");
    println!("{:?}", file);
}
