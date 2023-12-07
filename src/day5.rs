use std::fs::File;
use std::io::{BufReader, BufRead, Error};

const PATH: &str = "day5/input-test-part-1.txt";
// const PATH: &str = "day4/input.txt";

pub fn exec_day5() -> Result<(), Error> {
    println!("Hello world day 5");

    let input = File::open(PATH)?;
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        println!("{:?}", line);
    }

    Ok(())
}