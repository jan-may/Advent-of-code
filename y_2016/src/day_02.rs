use crate::lib::parse_input;

pub fn part_1() -> i32 {
    let input = parse_input("inputs/day_01.txt");
    const NUMPAD: [[&str; 3]; 3] = [
        ["1", "2", "3"],
        ["4", "5", "6"],
        ["7", "8", "9"],
    ];
    const START: (usize, usize) = (1, 1);
    let mut code = String::new();
    let mut position = START;
    for line in &input {
        for val in line.split("") {
            match val {
                "U" => if position.0 > 0 { position.0 -= 1 },
                "D" => if position.0 < 2 { position.0 += 1 },
                "L" => if position.1 > 0 { position.1 -= 1 },
                "R" => if position.1 < 2 { position.1 += 1 },
                _ => ()
            }
        }
        code.push(NUMPAD[position.0][position.1].to_string().parse::<char>().unwrap());
    }
    code.parse::<i32>().unwrap()
}

pub fn part_2() -> String {
    let input = parse_input("inputs/day_01.txt");
    const NUMPAD: [[&str; 5]; 5] = [
        ["", "", "1", "", ""],
        ["", "2", "3", "4", ""],
        ["5", "6", "7", "8", "9"],
        ["", "A", "B", "C", ""],
        ["", "", "D", "", ""],
    ];

    const START: (usize, usize) = (2, 0);

    let mut code = String::new();
    let mut position = START;
    for line in &input {
        for val in line.split("") {
            match val {
                "U" => if position.0 > 0 && !NUMPAD[position.0 - 1][position.1].is_empty() { position.0 -= 1 },
                "D" => if position.0 < 4 && !NUMPAD[position.0 + 1][position.1].is_empty() { position.0 += 1 },
                "L" => if position.1 > 0 && !NUMPAD[position.0][position.1 - 1].is_empty() { position.1 -= 1 },
                "R" => if position.1 < 4 && !NUMPAD[position.0][position.1 + 1].is_empty() { position.1 += 1 },
                _ => ()
            }
        }
        code.push(NUMPAD[position.0][position.1].to_string().parse::<char>().unwrap());
    }
    code
}



