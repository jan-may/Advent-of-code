use crate::lib::parse_input;
use md5;

pub fn part_1() -> String {
    let input = parse_input("inputs/day_05.txt")[0].clone();
    let mut password = String::new();
    let mut i = 0;
    while password.len() < 8 {
        let hash = format!("{:x}", md5::compute(format!("{}{}", input, i)));
        if hash.starts_with("00000") {
            password.push(hash.chars().nth(5).unwrap());
        }
        i += 1;
    }
    password
}

pub fn part_2() -> String {
    let input = parse_input("inputs/day_05.txt")[0].clone();
    let mut password = vec![' '; 8];
    let mut i = 0;
    while password.contains(&' ') {
        let hash = format!("{:x}", md5::compute(format!("{}{}", input, i)));
        if hash.starts_with("00000") {
            if let Some(pos) = hash.chars().nth(5).unwrap().to_digit(10) {
                if pos < 8 && password[pos as usize] == ' ' {
                    password[pos as usize] = hash.chars().nth(6).unwrap();
                }
            }
        }
        i += 1;
    }
    password.into_iter().collect()
}