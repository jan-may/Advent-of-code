use crate::lib::parse_input;
use std::collections::HashSet;
use std::str::Split;

pub fn part_1() -> i32 {
    let scratchcards = parse_input("inputs/day_04.txt");
    scratchcards
        .iter()
        .map(|card| process_and_count_card(card))
        .filter(|&win_count| win_count > 0)
        .map(|win_count| 2i32.pow(win_count as u32 - 1))
        .sum()
}

pub fn part_2() -> i32 {
    let scratchcards = parse_input("inputs/day_04.txt");
    let mut cards = vec![1; scratchcards.len()];

    for (i, card) in scratchcards.iter().enumerate() {
        let win_numbers_count = process_and_count_card(card);

        for _ in 0..cards[i] {
            for x in i + 1..=win_numbers_count + i {
                cards[x] += 1;
            }
        }
    }
    cards.iter().sum::<i32>()
}

fn process_and_count_card(card: &str) -> usize {
    let card = card[8..].replace("  ", " ").replace(": ", "");
    let mut scratchcard_parts = card.split(" | ");
    let winning_numbers = parse_numbers(&mut scratchcard_parts);
    let my_numbers = parse_numbers(&mut scratchcard_parts).into_iter().collect();
    count_winning_numbers(&winning_numbers, &my_numbers)
}

fn count_winning_numbers(winning_numbers: &[i32], my_numbers: &HashSet<i32>) -> usize {
    winning_numbers
        .iter()
        .filter(|num| my_numbers.contains(num))
        .count()
}

fn parse_numbers(scratchcard_parts: &mut Split<&str>) -> Vec<i32> {
    scratchcard_parts
        .next()
        .unwrap_or("")
        .split_whitespace()
        .filter_map(|num| num.parse::<i32>().ok())
        .collect()
}
