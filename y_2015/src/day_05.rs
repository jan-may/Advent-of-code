use std::collections::HashSet;
use crate::lib::parse_input;

pub fn part_1() -> i32 {
    // Implement the solution for part 1
    let data = parse_input("inputs/day_05.txt");
    let mut count = 0;
    for word in data {
        if has_three_vowels(&word) && has_double_letter(&word) && !has_bad_strings(&word) {
            count += 1;
        }
    }
    count
}

pub fn part_2() -> i32 {
    let data = parse_input("inputs/day_05.txt");
    let mut count = 0;
    for word in data {
        if has_double_pair(&word) && has_sandwich(&word) {
            count += 1;
        }
    }
    count
}

fn has_three_vowels(s: &str) -> bool {
    s.chars().filter(|c| "aeiou".contains(*c)).count() >= 3
}

fn has_double_letter(s: &str) -> bool {
    s.chars().collect::<Vec<char>>().windows(2).any(|pair| pair[0] == pair[1])
}

fn has_bad_strings(s: &str) -> bool {
    let bad_strings: HashSet<&str> = ["ab", "cd", "pq", "xy"].iter().cloned().collect();
    (0..s.len() - 1).any(|i| bad_strings.contains(&s[i..=i+1]))
}

fn has_double_pair(s: &str) -> bool {
    (0..s.len() - 2).any(|i| s[i+2..].contains(&s[i..=i+1]))
}

fn has_sandwich(s: &str) -> bool {
    s.chars().collect::<Vec<char>>().windows(3).any(|slice| slice[0] == slice[2])
}



