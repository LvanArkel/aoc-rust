use aocio;

type Parsed = Vec<Vec<u32>>;
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
    input.iter().map(|line| {
        line.split(" ").map(|val| val.parse().unwrap()).collect()
    }).collect()
}

fn part1_safe(report: &Vec<u32>) -> bool {
    let increasing = report[1] > report[0];
    for vals in report.windows(2) {
        let a = vals[0] as i32;
        let b = vals[1] as i32;
        let diff = if increasing {
            b - a
        } else {
            a - b
        };
        if diff < 1 || diff > 3 {
            return false;
        }
    }
    true
}

fn part1(input: &Parsed) -> Answer {
    input.iter().filter(|line| part1_safe(line)).count() as u32
}

fn part2_safe(report: &Vec<u32>) -> bool {
    if !part1_safe(report) {
        for i in 0..report.len() {
            let sublist = [&report[0..i], &report[i+1..report.len()]].concat();
            if part1_safe(&sublist) {
                return true;
            }
        }
        return false;
    }
    return true;
}

fn part2(input: &Parsed) -> Answer {
    input.iter().filter(|line| part2_safe(line)).count() as u32
}

fn main() {
    let input = get_input();
    let parsed = parse_input(input);
    if DEBUG {
        println!("Executing in DEBUG mode!")
    }
    println!("Tests part 1:");
    test1(2, vec![
        "7 6 4 2 1".to_string(),
        "1 2 7 8 9".to_string(),
        "9 7 6 2 1".to_string(),
        "1 3 2 4 5".to_string(),
        "8 6 4 4 1".to_string(),
        "1 3 6 7 9".to_string(),
    ]);

    if !DEBUG {
        println!("Result part 1: {}", part1(&parsed));
    }

    println!("Tests part 2:");
    test2(4, vec![
        "7 6 4 2 1".to_string(),
        "1 2 7 8 9".to_string(),
        "9 7 6 2 1".to_string(),
        "1 3 2 4 5".to_string(),
        "8 6 4 4 1".to_string(),
        "1 3 6 7 9".to_string(),
    ]);

    if !DEBUG {
        println!("Result part 2: {}", part2(&parsed));
    }
}