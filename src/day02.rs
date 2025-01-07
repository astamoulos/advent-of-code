use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn part_one() {
    let mut valid_lines = 0;
    if let Ok(lines) = read_lines("input/day2.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.map_while(Result::ok) {
            let sequence: Vec<i32> = line
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect();

            if !(sequence.windows(2).all(|w| w[1] > w[0])
                || sequence.windows(2).all(|w| w[1] < w[0]))
            {
                continue;
            }
            if sequence.windows(2).all(|w| i32::abs(w[1] - w[0]) <= 3) {
                valid_lines += 1;
            }
        }
    }
    println!("{}", valid_lines);
}

pub fn part_two() {
    let mut valid_lines = 0;
    if let Ok(lines) = read_lines("input/day2.txt") {
        for line in lines.map_while(Result::ok) {
            let sequence: Vec<i32> = line
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect();

            if (sequence.windows(2).all(|w| w[1] > w[0])
                || sequence.windows(2).all(|w| w[1] < w[0]))
                && sequence.windows(2).all(|w| i32::abs(w[1] - w[0]) <= 3)
            {
                valid_lines += 1;
                continue;
            }

            // Check if it becomes safe by removing one level
            let mut found_safe = false;
            for i in 0..sequence.len() {
                let mut modified_sequence = sequence.clone();
                modified_sequence.remove(i);

                if (modified_sequence.windows(2).all(|w| w[1] > w[0])
                    || modified_sequence.windows(2).all(|w| w[1] < w[0]))
                    && modified_sequence
                        .windows(2)
                        .all(|w| i32::abs(w[1] - w[0]) <= 3)
                {
                    found_safe = true;
                    break;
                }
            }

            if found_safe {
                valid_lines += 1;
            }
        }
    }
    println!("{}", valid_lines);
}
