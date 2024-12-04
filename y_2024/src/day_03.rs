use crate::lib::parse_input;
use regex::Regex;

const MUL_PATTERN: &str = r"mul\((\d{1,3}),(\d{1,3})\)";
const INSTRUCTION_PATTERN: &str = r"(do\(\)|don't\(\))";
const INPUT_FILE: &str = "inputs/day_03.txt";

pub fn part_1() -> i32 {
    let input = parse_input(INPUT_FILE);
    let re = Regex::new(MUL_PATTERN).unwrap();

    let mut sum = 0;
    for line in input {
        for caps in re.captures_iter(&line) {
            let num1: i32 = caps[1].parse().unwrap();
            let num2: i32 = caps[2].parse().unwrap();
            sum += num1 * num2;
        }
    }
    sum
}

pub fn part_2() -> i32 {
    let input: String = parse_input(INPUT_FILE).join("");
    let re_mul = Regex::new(MUL_PATTERN).unwrap();
    let re_instructions = Regex::new(INSTRUCTION_PATTERN).unwrap();

    let mut sum = 0;
    let mut is_enabled = true; // mul is enabled by default
    let mut current_start = 0;

    // Iterate through all matches of do() and don't()
    for caps in re_instructions.find_iter(&input) {
        let instruction = caps.as_str();
        let end = caps.start();

        if is_enabled {
            // Process all valid mul instructions in the current region
            let region = &input[current_start..end];
            for mul_caps in re_mul.captures_iter(region) {
                let num1: i32 = mul_caps[1].parse().unwrap();
                let num2: i32 = mul_caps[2].parse().unwrap();
                sum += num1 * num2;
            }
        }

        match instruction {
            "do()" => is_enabled = true,
            "don't()" => is_enabled = false,
            _ => (),
        }

        // Update the start of the next region
        current_start = caps.end();
    }

    // Process the remaining region
    if is_enabled {
        let region = &input[current_start..];
        for mul_caps in re_mul.captures_iter(region) {
            let num1: i32 = mul_caps[1].parse().unwrap();
            let num2: i32 = mul_caps[2].parse().unwrap();
            sum += num1 * num2;
        }
    }
    sum
}
