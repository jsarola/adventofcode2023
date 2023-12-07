use std::fs::File;
use std::io::{BufReader, BufRead, Error};

const PATH: &str = "day6/input-test-part-1.txt";
// const PATH: &str = "day4/input.txt";

pub fn exec_day6() -> Result<(), Error> {
    println!("Hello world day 6");

    let input = File::open(PATH)?;
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        println!("{:?}", line);
    }
    
    Ok(())
}