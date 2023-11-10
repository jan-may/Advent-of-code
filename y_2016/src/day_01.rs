use std::collections::HashSet;
use crate::lib::parse_input;

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}

enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn(&mut self, turn: &str) {
        match turn {
            "R" => *self = match *self {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
            },
            "L" => *self = match *self {
                Direction::North => Direction::West,
                Direction::East => Direction::North,
                Direction::South => Direction::East,
                Direction::West => Direction::South,
            },
            _ => panic!("Invalid turn"),
        }
    }

    fn move_forward(&self, position: &mut Point, steps: i32) {
        match *self {
            Direction::North => position.y += steps,
            Direction::East => position.x += steps,
            Direction::South => position.y -= steps,
            Direction::West => position.x -= steps,
        }
    }
}

pub fn part_1() -> i32 {
    let input = parse_input("inputs/day_01.txt");
    let mut position = Point { x: 0, y: 0 };
    let mut direction = Direction::North;

    for line in input.iter() {
        for val in line.split(", ") {
            let turn = val.split_at(1).0;
            let steps = val.split_at(1).1.parse::<i32>().unwrap();
            direction.turn(turn);
            direction.move_forward(&mut position, steps);
        }
    }

    position.x.abs() + position.y.abs()
}

pub fn part_2() -> i32 {
    let input = parse_input("inputs/day_01.txt");
    let mut position = Point { x: 0, y: 0 };
    let mut direction = Direction::North;
    let mut visited = HashSet::new();

    for line in input.iter() {
        for val in line.split(", ") {
            let turn = val.split_at(1).0;
            let steps = val.split_at(1).1.parse::<i32>().unwrap();
            direction.turn(turn);
            for _ in 0..steps {
                direction.move_forward(&mut position, 1);
                if visited.contains(&position) {
                    return position.x.abs() + position.y.abs();
                }
                visited.insert(position);
            }
        }
    }

    0 // Default return value if no revisited location is found.
}
