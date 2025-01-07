mod day01;
mod day02;
mod day03;
mod day04;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args[1].parse().expect("Invalid day number");
    match day {
        1 => {
            day01::part_one();
            println!("part two");
            day01::part_two();
        }
        2 => {
            day02::part_one();
            println!("part two");
            day02::part_two();
        }
        3 => {
            day03::part_one();
            println!("part two");
            day03::part_two();
        }
        4 => {
            day04::part_one();
            println!("part two");
            day04::part_two();
        }
        _ => println!("No solution for the given day."),
    };
}
