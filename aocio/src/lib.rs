use std::fs;

pub fn read_input(name: &str) -> String {
    fs::read_to_string(name).expect("Invalid file path")
}

pub fn read_lines(name: &str) -> Vec<String> {
    let contents = read_input(name);
    let mut result = Vec::new();
    for line in contents.lines() {
        result.push(String::from(line));
    }
    result
}