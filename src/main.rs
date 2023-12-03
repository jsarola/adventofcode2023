mod day1;
mod day2;
mod day3;

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
        "day1" => say_day1(),
        "day2" => say_day2(),
        "day3" => say_day3(),
        _ => {
            println!("Unknown function: {}", function_name);
            println!("Available functions: say_hello, say_goodbye");
        }
    }
}

fn say_day1() {
    day1::exec_day1();
}

fn say_day2() {
    day2::exec_day2();
}

fn say_day3() {
    day3::exec_day3();
}

fn say_goodbye() {
    println!("Goodbye, world!");
}
