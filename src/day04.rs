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
    let mut input: Vec<Vec<char>> = Vec::new();
    let mut appeared = 0;

    if let Ok(lines) = read_lines("input/day4.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            let char_vec: Vec<char> = line.chars().collect();
            input.push(char_vec);
        }
    }

    let height = input.len();
    let width = input[0].len();
    let matches = |slice: &[char]| slice == ['X', 'M', 'A', 'S'] || slice == ['S', 'A', 'M', 'X'];

    for i in 0..=height - 1 {
        for j in 0..width {
            if j + 3 < width && matches(&input[i][j..j + 4]) {
                appeared += 1;
            }
            if i + 3 < height {
                let vertical: Vec<char> = (0..4).map(|k| input[i + k][j]).collect();
                if matches(&vertical) {
                    appeared += 1;
                }
            }
            if i + 3 < height && j + 3 < width {
                let diagonal: Vec<char> = (0..4).map(|k| input[i + k][j + k]).collect();
                if matches(&diagonal) {
                    appeared += 1;
                }
            }
            if i >= 3 && j + 3 < width {
                let diagonal: Vec<char> = (0..4).map(|k| input[i - k][j + k]).collect();

                if matches(&diagonal) {
                    appeared += 1;
                }
            }
        }
    }
    println!("{}", appeared);
}

pub fn part_two() {
    let mut input: Vec<Vec<char>> = Vec::new();
    let mut appeared = 0;

    if let Ok(lines) = read_lines("input/day4.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            let char_vec: Vec<char> = line.chars().collect();
            input.push(char_vec);
        }
    }

    let height = input.len();
    let width = input[0].len();
    let matches = |slice: &[char]| slice == ['M', 'A', 'S'] || slice == ['S', 'A', 'M'];

    for i in 1..(height - 1) {
        for j in 1..(width - 1) {
            let right_diagonal: Vec<char> = (0..3).map(|k| input[i + k - 1][j + k - 1]).collect();
            let left_diagonal: Vec<char> = (0..3).map(|k| input[i + 1 - k][j + k - 1]).collect();

            if matches(&right_diagonal) && matches(&left_diagonal) {
                appeared += 1;
            }
        }
    }
    println!("{}", appeared);
}
