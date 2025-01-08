use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part_one() {
    let file = File::open("input/day5.txt").unwrap();
    let reader = BufReader::new(file);
    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut line_break = 0;
    let mut solution = 0;
    for line in reader.lines() {
        let line = line.unwrap();

        if line.is_empty() {
            line_break = 1;
            continue;
        }

        if line_break == 0 {
            let (a, b) = line.split_once('|').unwrap();
            rules
                .entry(a.parse::<i32>().unwrap())
                .or_insert(Vec::new())
                .push(b.parse::<i32>().unwrap());
        } else {
            let arr: Vec<i32> = line.split(',').map(|s| s.parse::<i32>().unwrap()).collect();
            let mut error_found = false;

            'outer: for i in 0..arr.len() {
                let curr = arr[i];
                for j in i + 1..arr.len() {
                    let next = arr[j];
                    //println!("{:?}", rules.get(&next));
                    if let Some(rule) = rules.get(&next) {
                        if rule.contains(&curr) {
                            error_found = true;
                            break 'outer;
                        }
                    }
                }
            }

            if !error_found {
                solution += arr[arr.len() / 2];
            }
        }
    }

    println!("{:?}", solution);
}

pub fn part_two() {
    let file = File::open("input/day5.txt").unwrap();
    let reader = BufReader::new(file);
    let mut rules: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut line_break = 0;
    let mut solution = 0;
    for line in reader.lines() {
        let line = line.unwrap();

        if line.is_empty() {
            line_break = 1;
            continue;
        }

        if line_break == 0 {
            let (a, b) = line.split_once('|').unwrap();
            rules
                .entry(b.parse::<i32>().unwrap())
                .or_default()
                .insert(a.parse::<i32>().unwrap());
        } else {
            let mut arr: Vec<i32> = line.split(',').map(|s| s.parse::<i32>().unwrap()).collect();

            if !arr.is_sorted_by(|a, b| rules[b].contains(a)) {
                arr.sort_by(|a, b| rules[b].contains(a).cmp(&true));
                solution += arr[arr.len() / 2];
            }
        }
    }

    println!("{:?}", solution);
}
