use std::collections::HashMap;
use std::fs::read_to_string;

pub fn part_one() {
    let mut col1: Vec<i32> = Vec::new();
    let mut col2: Vec<i32> = Vec::new();
    println!();
    let binding = read_to_string("input/day1.txt").unwrap();
    for line in binding.lines() {
        let Some((a, b)) = line.split_once("   ") else {
            todo!()
        };
        let a = a.parse::<i32>().unwrap();
        let b = b.parse::<i32>().unwrap();
        col1.push(a);
        col2.push(b);
    }

    col1.sort();
    col2.sort();

    let solution: i32 = col1
        .into_iter()
        .zip(col2)
        .map(|(a, b)| i32::abs(a - b))
        .sum();

    println!("{}", solution);
}

pub fn part_two() {
    let mut col1: Vec<i32> = Vec::new();
    let mut map: HashMap<i32, i32> = HashMap::new();

    let binding = read_to_string("input/day1.txt").unwrap();
    for line in binding.lines() {
        let Some((a, b)) = line.split_once("   ") else {
            todo!()
        };
        let a = a.parse::<i32>().unwrap();
        let b = b.parse::<i32>().unwrap();
        col1.push(a);
        *map.entry(b).or_insert(0) += 1;
    }

    let solution: i32 = col1
        .into_iter()
        .map(|a| a * map.get(&a).unwrap_or(&0))
        .sum();

    println!("{}", solution);
}
