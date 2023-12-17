//!main.rs

use advent_of_code_2023::run;

fn main() {
    if let Err(err) = run() {
        println!("Error occured: {}", err);

        // look for source
        match err.source() {
            Some(source) => println!("Source of error: {:?}", source),
            None => (),
        }
    }
}
