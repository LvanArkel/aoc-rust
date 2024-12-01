use std::{collections::HashMap, iter::zip};

use aocio;
use itertools::{sorted, Itertools};

type Parsed = (Vec<u32>, Vec<u32>);
type Answer = u32;

const DEBUG: bool = false;

fn test1(expected: Answer, actual_raw: Vec<String>) {
    let actual = parse_input(actual_raw);
    let actual_result = part1(&actual);
    println!("Expecting {}, Actual: {}", expected, actual_result);
}

fn test2(expected: Answer, actual_raw: Vec<String>) {
    let actual = parse_input(actual_raw);
    let actual_result = part2(&actual);
    println!("Expecting {}, Actual: {}", expected, actual_result);
}

fn get_input() -> Vec<String> {
    aocio::read_lines("input.txt")
}

fn parse_input(input: Vec<String>) -> Parsed {
    let mut first = Vec::new();
    let mut second = Vec::new();
    for line in input {
        let splitted: Vec<_> = line.split(r" ").collect();
        first.push(splitted.first().unwrap().parse().unwrap());
        second.push(splitted.last().unwrap().parse().unwrap());
    }
    (first, second)
}

fn part1(input: &Parsed) -> Answer {
    zip(
        sorted(input.0.iter()),
        sorted(input.1.iter()),
    )
    .map(|(a, b)| {
        a.abs_diff(*b)
    })
    .sum()
}

fn part2(input: &Parsed) -> Answer {
    let occurences: HashMap<_, _> = input.1.iter().counts();
    input.0.iter()
    .map(|i| i * *occurences.get(i).unwrap_or(&0) as u32)
    .sum()
}

fn main() {
    let input = get_input();
    let parsed = parse_input(input);
    if DEBUG {
        println!("Executing in DEBUG mode!")
    }
    println!("Tests part 1:");
    test1(11, vec![
        "3   4".to_string(),
        "4   3".to_string(),
        "2   5".to_string(),
        "1   3".to_string(),
        "3   9".to_string(),
        "3   3".to_string(),
    ]);

    if !DEBUG {
        println!("Result part 1: {}", part1(&parsed));
    }

    println!("Tests part 2:");
    test2(31, vec![
        "3   4".to_string(),
        "4   3".to_string(),
        "2   5".to_string(),
        "1   3".to_string(),
        "3   9".to_string(),
        "3   3".to_string(),
    ]);

    if !DEBUG {
        println!("Result part 2: {}", part2(&parsed));
    }
}