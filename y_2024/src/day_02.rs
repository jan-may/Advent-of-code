use crate::lib::parse_input;

fn parse_and_split_input(file_path: &str) -> Vec<Vec<i32>> {
    let input = parse_input(file_path);
    input
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn check_all_decreasing(input: &Vec<i32>) -> bool {
    for i in 0..input.len() - 1 {
        if input[i] > input[i + 1] {
            return false;
        }
        let diff: i32 = input[i + 1] - input[i];
        if diff > 3 || diff == 0 {
            return false;
        }
    }
    true
}

fn check_all_increasing(input: &Vec<i32>) -> bool {
    for i in 0..input.len() - 1 {
        if input[i] < input[i + 1] {
            return false;
        }
        let diff: i32 = input[i] - input[i + 1];
        if diff > 3 || diff == 0 {
            return false;
        }
    }
    true
}

pub fn part_1() -> i32 {
    let input = parse_and_split_input("inputs/day_02.txt");
    let mut sum = 0;
    for line in input {
        if check_all_decreasing(&line) || check_all_increasing(&line) {
            sum += 1;
        }
    }
    sum
}

pub fn part_2() -> i32 {
    let input = parse_and_split_input("inputs/day_02.txt");
    let mut sum = 0;
    for line in input {
        if check_all_decreasing(&line) || check_all_increasing(&line) {
            sum += 1;
        } else {
            // check if removing any element will make the list valid (buteforce approach, considering the list is small)
            for i in 0..line.len() {
                let mut new_line = line.clone();
                new_line.remove(i);
                if check_all_decreasing(&new_line) || check_all_increasing(&new_line) {
                    sum += 1;
                    break;
                }
            }
        }
    }
    sum
}
