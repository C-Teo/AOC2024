use std::fs::File;
use std::io::{self, Read};

pub fn run(input: u32) -> Result<String, io::Error> {
    let filename: String = format!("./src/solutions/day{:02}/input", input);
    println!("Opening file at {}", filename);
    let mut file = File::open(&filename)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    Ok(contents)
}
