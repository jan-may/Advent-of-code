use crate::lib::parse_input;
use itertools::Itertools;
use std::collections::HashMap;

const INPUT_FILE: &str = "inputs/day_05.txt";

fn split_input() -> (Vec<String>, Vec<Vec<i32>>) {
    // Read the input and split into parts by empty lines
    let sections: Vec<Vec<String>> = parse_input(INPUT_FILE)
        .split(|line| line.is_empty())
        .map(|part| part.iter().map(ToOwned::to_owned).collect())
        .collect();

    let top_part = sections.first().cloned().unwrap_or_default();
    let bottom_part = sections
        .get(1)
        .cloned()
        .unwrap_or_default()
        .iter()
        .map(|line| {
            line.split(',')
                .filter_map(|num| num.parse::<i32>().ok()) // Parse safely and ignore errors
                .collect()
        })
        .collect();

    (top_part, bottom_part)
}

pub fn part_1() -> i32 {
    let (top_part, bottom_part) = split_input();
    let mut sum = 0;
    // hashmap of all the rule keys and their associated values
    let mut key_smaller_than_value_hashmap: HashMap<i32, Vec<i32>> = HashMap::new();
    for line in top_part {
        let (rule_key, rule_value) = line
            .split('|')
            .map(|s| s.parse::<i32>().unwrap())
            .collect_tuple()
            .expect("Invalid input format");
        key_smaller_than_value_hashmap
            .entry(rule_key)
            .or_default()
            .push(rule_value);
    }

    for line in bottom_part {
        let mut valid = true;
        for (index, value) in line.iter().enumerate() {
            // check if the value is in the hashmap
            if key_smaller_than_value_hashmap.contains_key(value) {
                // get the keys associated with the value
                let values = key_smaller_than_value_hashmap.get(value).unwrap();
                // check if all the values come after the key in the current line
                for val in values {
                    if line.contains(val) {
                        if line[index + 1..].contains(val) {
                            continue;
                        } else {
                            valid = false;
                            break;
                        }
                    }
                }
            }
        }
        if valid {
            // get middle value of the line
            let middle = line[line.len() / 2];
            sum += middle;
        }
    }
    sum
}

pub fn part_2() -> i32 {
    // brute force solution not suitable for large inputs...
    0
}
