use std::fs::File;
use std::io::{self, Read};

pub fn run(input: u32) -> Result<String, io::Error> {
    let filename = format!("./src/solutions/day{:02}/input", input);
    let mut file = File::open(&filename)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    Ok(contents)
}