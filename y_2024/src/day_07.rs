use crate::lib::parse_input;

const INPUT_FILE: &str = "inputs/day_07.txt";
const P1_OPERATIONS: [&str; 2] = ["*", "+"];
const P2_OPERATIONS: [&str; 3] = ["*", "+", "||"];

/// Parses the input file into a list of targets and their associated values.
fn get_target_and_values() -> Vec<(i64, Vec<i64>)> {
    parse_input(INPUT_FILE)
        .iter()
        .map(|line| {
            let parts = line.split_whitespace().collect::<Vec<&str>>();
            let target = parts[0].replace(':', "").parse::<i64>().unwrap();
            let values = parts[1..]
                .iter()
                .map(|num| num.parse::<i64>().unwrap())
                .collect();
            (target, values)
        })
        .collect()
}

/// Evaluates a sequence of operations on a list of values from left to right.
fn evaluate(values: &[i64], operations: &[&str]) -> i64 {
    let mut result = values[0];
    for (i, operation) in operations.iter().enumerate() {
        match *operation {
            "*" => result *= values[i + 1],
            "+" => result += values[i + 1],
            "||" => {
                let concatenated = format!("{}{}", result, values[i + 1])
                    .parse::<i64>()
                    .expect("Concatenation failed");
                result = concatenated;
            }
            _ => panic!("Invalid operation"),
        }
    }
    result
}

/// Solves the problem using the specified set of operations.
fn solve(operations: &[&str]) -> i64 {
    let target_and_values = get_target_and_values();
    let mut sum = 0;

    for (target, values) in target_and_values {
        let num_operations = values.len() - 1;
        let all_combinations = (0..operations.len().pow(num_operations as u32)).map(|n| {
            (0..num_operations)
                .map(|i| operations[(n / operations.len().pow(i as u32)) % operations.len()])
                .collect::<Vec<_>>()
        });

        for ops in all_combinations {
            if evaluate(&values, &ops) == target {
                sum += target;
                break;
            }
        }
    }
    sum
}

pub fn part_1() -> i64 {
    solve(&P1_OPERATIONS)
}

pub fn part_2() -> i64 {
    solve(&P2_OPERATIONS)
}
