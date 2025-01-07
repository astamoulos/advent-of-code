use regex::Regex;
use std::fs;

pub fn part_one() {
    let data = fs::read_to_string("input/day3.txt").expect("Unable to read file");

    let re = Regex::new(r"mul\((?<x>[0-9]{1,3}),(?<y>[0-9]{1,3})\)").unwrap();
    let mult: i32 = re
        .captures_iter(&data)
        .map(|m| {
            let x = m.name("x").unwrap().as_str().parse::<i32>().unwrap();
            let y = m.name("y").unwrap().as_str().parse::<i32>().unwrap();
            x * y
        })
        .sum();

    println!("matches {:?}", mult);
}

pub fn part_two() {
    let data = fs::read_to_string("input/day3.txt").expect("Unable to read file");

    let re = Regex::new(r"don't\(\)|do\(\)|mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();
    let mut enable = 1;
    let mult: Vec<&str> = re.find_iter(&data).map(|m| m.as_str()).collect();
    let mut sol = 0;
    let re1 = Regex::new(r"mul\((?<x>[0-9]{1,3}),(?<y>[0-9]{1,3})\)").unwrap();
    for matched in mult {
        match matched {
            "do()" => enable = 1,
            "don't()" => enable = 0,
            _ if enable == 1 => {
                let Some(caps) = re1.captures(matched) else {
                    println!("no match!");
                    return;
                };
                sol += caps["x"].parse::<i32>().unwrap() * caps["y"].parse::<i32>().unwrap();
            }
            _ => {}
        }
    }
    println!("matches {:?}", sol);
}
