use aocio;
use fancy_regex::Regex;

type Parsed = String;
type Answer = u32;

const DEBUG: bool = false;

fn test1(expected: Answer, actual_raw: Parsed) {
    let actual = parse_input(actual_raw);
    let actual_result = part1(&actual);
    println!("Expecting {}, Actual: {}", expected, actual_result);
}

fn test2(expected: Answer, actual_raw: String) {
    let actual = parse_input(actual_raw);
    let actual_result = part2(&actual);
    println!("Expecting {}, Actual: {}", expected, actual_result);
}

fn get_input() -> String {
    aocio::read_input("input.txt")
}

fn parse_input(input: String) -> Parsed {
    input
}

fn part1(input: &Parsed) -> Answer {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(input).map(|cap| {
        let cap = cap.unwrap();
        let first = cap.get(1).unwrap().as_str().parse::<u32>().unwrap();
        let second = cap.get(2).unwrap().as_str().parse::<u32>().unwrap();
        first * second
    }).sum::<Answer>()
}

fn part2(input: &Parsed) -> Answer {
    let re = Regex::new(r"(?:mul\((\d+),(\d+)\)|do\(\)|don't\(\))").unwrap();
    let mut enabled = true;
    re.captures_iter(input).map(|cap| {
        let cap = cap.unwrap();
        if (cap.get(0).unwrap().as_str().contains("do")) {
            enabled = cap.get(0).unwrap().as_str() == "do()";
            return 0;
        }
        if !enabled { return 0; }
        let first = cap.get(1).unwrap().as_str().parse::<u32>().unwrap();
        let second = cap.get(2).unwrap().as_str().parse::<u32>().unwrap();
        first * second
    }).sum::<Answer>()
}

fn main() {
    let input = get_input();
    let parsed = parse_input(input);
    if DEBUG {
        println!("Executing in DEBUG mode!")
    }
    println!("Tests part 1:");
    test1(161, "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string());

    if !DEBUG {
        println!("Result part 1: {}", part1(&parsed));
    }

    println!("Tests part 2:");
    test2(48, "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_string());

    if !DEBUG {
        println!("Result part 2: {}", part2(&parsed));
    }
}