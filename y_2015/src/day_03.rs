use crate::lib::parse_input;
use std::collections::HashSet;

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
struct Location {
    x: i32,
    y: i32,
}

pub fn part_1() -> i32 {
    let input_data = parse_input("inputs/day_03.txt");
    let input_data: Vec<char> = input_data[0].chars().collect();
    let mut location = Location { x: 0, y: 0 };
    let mut visited_locations = HashSet::new();
    visited_locations.insert(location);

    for char in input_data {
        match char {
            '^' => location.y += 1,
            'v' => location.y -= 1,
            '>' => location.x += 1,
            '<' => location.x -= 1,
            _ => (),
        }
        visited_locations.insert(location);
    }
    visited_locations.len() as i32
}

pub fn part_2() -> i32 {
    let input_data = parse_input("inputs/day_03.txt");
    let input_data: Vec<char> = input_data[0].chars().collect();
    let mut santa_location = Location { x: 0, y: 0 };
    let mut robot_location = Location { x: 0, y: 0 };
    let mut visited_locations = HashSet::new();
    visited_locations.insert(santa_location);
    for (i, char) in input_data.iter().enumerate() {
        let location = if i % 2 == 0 {
            &mut santa_location
        } else {
            &mut robot_location
        };
        match char {
            '^' => location.y += 1,
            'v' => location.y -= 1,
            '>' => location.x += 1,
            '<' => location.x -= 1,
            _ => (),
        }
        visited_locations.insert(*location);
    }
    visited_locations.len() as i32
}
