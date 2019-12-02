use structopt::StructOpt;
use std::fs;
use std::format;

mod day1;
use day1::main1;

mod day2;
use day2::main2;

#[derive(StructOpt)]
struct Cli {
   puzzle: Option<u8>,
}

fn get_string(name: u8) -> String {
    let contents = fs::read_to_string(
        format!("inputs/{}", name))
        .expect("Something went wrong reading the file");
    return contents;
}

fn main() {
    println!("Advent Of Code 2019");
    let args = Cli::from_args();
    match args.puzzle {
        None => {
            //println!("Last day chosen");
            main2();
        },
        Some(1) => { main1(get_string(1)) },
        Some(2) => { main2() },
        Some(_) => { println!("Choose a day.") },
    }
}
