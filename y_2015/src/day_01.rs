use crate::lib::parse_input;

pub fn part_1() -> i32 {
    // Implement the solution for part 1
    let text = parse_input("inputs/day_01.txt");
    let mut floor = 0;
    for c in text[0].chars() {
        if c == '(' {
            floor += 1;
        } else if c == ')' {
            floor -= 1;
        }
    }
    floor
}

pub fn part_2() -> i32 {
    // Implement the solution for part 2
    let text = parse_input("inputs/day_01.txt");
    let mut floor = 0;
    for (i, c) in text[0].chars().enumerate() {
        if c == '(' {
            floor += 1;
        } else if c == ')' {
            floor -= 1;
        }
        if floor == -1 {
            return (i + 1) as i32;
        }
    }
    floor
}
