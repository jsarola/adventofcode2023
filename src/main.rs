mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

use std::env;

fn main() {
    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if at least one argument is provided
    if args.len() < 2 {
        println!("Usage: {} <function_name>", args[0]);
        return;
    }

    // Get the function name from the command-line argument
    let function_name = &args[1];

    // Execute the function based on the argument
    match function_name.as_str() {
        "day1" => {
            let _ = day1::exec_day1();
        }
        "day2" => {
            let _ = day2::exec_day2();
        }
        "day3" => {
            let _ = day3::exec_day3();
        }
        "day4" => {
            let _ = day4::exec_day4();
        }
        "day5" => {
            let _ = day5::exec_day5();
        }
        "day6" => {
            let _ = day6::exec_day6();
        }
        "day7" => {
            let _ = day7::exec_day7();
        }
        "day8" => {
            let _ = day8::exec_day8();
        }
        _ => {
            println!("Unknown function: {}", function_name);
            println!("Available functions: day1, day2, ...");
        }
    }
}
