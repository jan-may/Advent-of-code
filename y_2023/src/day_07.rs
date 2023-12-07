use std::collections::HashMap;

use crate::lib::parse_input;

const POSSIBLE_REPLACEMENTS: [char; 13] = ['2', '3', '4', '5', '6', '7', '8', '9', 'V', 'W', 'X', 'Y', 'Z'];
const CARD_TYPES : usize = 7;

pub fn part_1() -> i32 {
    let input = parse_input_and_simplify("inputs/day_07.txt",1);
    process_hands(input, get_hand_rank)
}

// another classic Brute Force solution :D
pub fn part_2() -> i32 {
    let input = parse_input_and_simplify("inputs/day_07.txt", 2);
    process_hands(input, get_hand_rank_joker)
}

// Replace face Cards to simplify the ordering of hands
fn parse_input_and_simplify(file_path: &str, part: u8) -> Vec<String> {
    let joker_value = match part {
        1 => "W",
        2 => "0",
        _ => panic!("Invalid part number"),
    };
    parse_input(file_path)
        .iter_mut()
        .map(|x| x.replace('A', "Z").replace('K', "Y").replace('Q', "X").replace('J', joker_value).replace('T', "V"))
        .collect()
}

fn process_hands(input: Vec<String>, rank_fn: fn(&str) -> i32) -> i32 {
    let mut rank_vec: Vec<Vec<(&str, i32)>> = vec![vec![]; CARD_TYPES];
    let mut total = 0;

    for line in &input {
        let mut parts = line.split_whitespace();
        let hand = parts.next().unwrap();
        let bid = parts.next().unwrap().parse::<i32>().unwrap();
        rank_vec[rank_fn(hand) as usize].push((hand, bid));
    }

    for inner_vec in rank_vec.iter_mut() {
        inner_vec.sort_by(|a, b| b.0.cmp(a.0));
    }

    let sorted_hands: Vec<(&str, i32)> = rank_vec.into_iter().flatten().collect();

    for (i, bid) in sorted_hands.iter().enumerate() {
        let rank_points = sorted_hands.len() - i;
        total += bid.1 * rank_points as i32;
    }
    total
}

fn get_hand_rank_joker(hand: &str) -> i32 {
    generate_all_hands(hand, &POSSIBLE_REPLACEMENTS, 0)
        .into_iter()
        .map(|new_hand| get_hand_rank(&new_hand))
        .min()
        .unwrap_or(6) // Return high card rank if no valid hand is formed
}

// Generates all possible hands by replacing each joker with all possible cards
fn generate_all_hands(hand: &str, replacements: &[char], start: usize) -> Vec<String> {
    if start >= hand.len() {
        return vec![hand.to_string()];
    }

    let mut hands = Vec::new();
    if hand.chars().nth(start).unwrap() == '0' { // If it's a joker
        for &replacement in replacements {
            let new_hand = hand[0..start].to_string() + &replacement.to_string() + &hand[start + 1..];
            hands.extend(generate_all_hands(&new_hand, replacements, start + 1));
        }
    } else {
        hands.extend(generate_all_hands(hand, replacements, start + 1));
    }
    hands
}

fn get_hand_rank(hand: &str) -> i32 {
    let mut map = HashMap::new();

    for c in hand.chars() {
        *map.entry(c).or_insert(0) += 1;
    }

    let mut values: Vec<_> = map.iter().collect();
    values.sort_by(|a, b| b.1.cmp(a.1));

    match values.as_slice() {
        [(_c, &5), ..] => 0, // Five of a Kind
        [(_c, &4), ..] => 1, // Four of a Kind
        [(_c, &3), (_d, &2), ..] | [(_d, &2), (_c, &3), ..] => 2, // Full House
        [(_c, &3), ..] => 3, // Three of a Kind
        [(_c, &2), (_d, &2), ..] => 4, // Two Pair
        [(_c, &2), ..] => 5, // One Pair
        _ => 6, // High card or no match
    }
}
