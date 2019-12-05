pub mod day3;
pub use day3::main3;

pub mod day4;
pub use day4::main4;

use std::format;
use std::fs;

pub fn get_string(name: u8) -> String {
    let contents = fs::read_to_string(format!("inputs/{}", name))
        .expect("Something went wrong reading the file");
    return contents;
}