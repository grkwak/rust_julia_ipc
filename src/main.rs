use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        match line.as_str() {
            "function1" => {
                let result = function1();
                writeln!(io::stdout(), "{}", result).unwrap();
            }
            "function2" => {
                let result = function2();
                writeln!(io::stdout(), "{}", result).unwrap();
            }
            _ => {
                writeln!(io::stderr(), "Unknown function: {}", line).unwrap();
            }
        }
    }
}

fn function1() -> i32 {
    // Your function implementation here
    42
}

fn function2() -> i32 {
    // Your function implementation here
    24
}
