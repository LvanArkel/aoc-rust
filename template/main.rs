use aocio;

type Parsed = Vec<String>;
type Answer = u32;

const DEBUG: bool = true;

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
    input
}

fn part1(input: &Parsed) -> Answer {
    todo!()
}

fn part2(input: &Parsed) -> Answer {
    todo!()
}

fn main() {
    let input = get_input();
    let parsed = parse_input(input);
    if DEBUG {
        println!("Executing in DEBUG mode!")
    }
    println!("Tests part 1:");

    if !DEBUG {
        println!("Result part 1: {}", part1(&parsed));
    }

    println!("Tests part 2:");

    if !DEBUG {
        println!("Result part 2: {}", part2(&parsed));
    }
}