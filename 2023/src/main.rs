use clap::Parser;
use std::path::Path;

mod day1;
mod utils;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'd', long = "day")]
    day: u8,

    #[arg(short = 'p', long = "part", default_value_t = 1)]
    part: u8,

    #[arg(short = 't', long = "test", default_value_t = false)]
    test: bool,

    #[arg(short = 'n', long = "new", default_value_t = false)]
    new: bool,
}

fn main() {
    let args = Args::parse();

    if args.new {
        let result = utils::new_aoc_day(&args.day);
        match result {
            Ok(_) => println!("Successfully created new AoC day {}", args.day),
            Err(e) => println!("Failed to create new AoC day {}", e),
        }
        return;
    }

    let test_input_path = format!("./data/day{}/test.txt", args.day);
    let data_input_path = format!("./data/day{}/data.txt", args.day);

    let input_path = match args.test {
        true => Path::new(&test_input_path),
        false => Path::new(&data_input_path),
    };

    match (args.day, args.part) {
        (1, 1) => {
            let result = day1::part1(input_path);
            println!("Result {}", result);
        }
        (1, 2) => {
            let result = day1::part2(input_path);
            println!("Result {}", result);
        }
        _ => {
            println!(
                "Too early for this day {} and part {}!",
                args.day, args.part
            );
        }
    }
}
