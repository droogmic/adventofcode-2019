use structopt::StructOpt;

pub use advent2019::get_string;

mod day1;
use day1::main1;

mod day2;
use day2::main2;

use advent2019::main3;
use advent2019::main4;

mod day5;
use day5::main5;

mod day6;
use day6::main6;

#[derive(StructOpt)]
struct Cli {
    puzzle: Option<u8>,
}

fn main() {
    println!("Advent Of Code 2019");
    let args = Cli::from_args();
    match args.puzzle {
        None => {
            main6(get_string(6));
        }
        Some(1) => main1(get_string(1)),
        Some(2) => main2(),
        Some(3) => main3(get_string(3)),
        Some(4) => main4(),
        Some(5) => main5(),
        Some(6) => main6(get_string(6)),
        Some(_) => println!("Choose a day."),
    }
}
