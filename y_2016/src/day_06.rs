use crate::lib::parse_input;

const CHAR_COUNT: usize = 26;
const PASSWORD_LENGTH: usize = 8;
const ASCII_OFFSET: u8 = 97; // ASCII offset for lowercase letters

pub fn part_1() -> String {
    let input = parse_input("inputs/day_06.txt");
    let mut arr = [[0; CHAR_COUNT]; PASSWORD_LENGTH];
    fill_arr(input, &mut arr);
    let mut password = String::new();
    for item in arr.iter().take(PASSWORD_LENGTH) {
        let mut max = 0;
        let mut max_i = 0;
        for (i,val) in item.iter().enumerate().take(CHAR_COUNT) {
            if *val > max {
                max = *val;
                max_i = i;
            }
        }
        password.push((max_i as u8 + ASCII_OFFSET) as char);
    }
    password
}


pub fn part_2() -> String {
    let input = parse_input("inputs/day_06.txt");
    let mut arr = [[0; CHAR_COUNT]; PASSWORD_LENGTH];
    fill_arr(input, &mut arr);
    let mut password = String::new();
    for item in arr.iter().take(PASSWORD_LENGTH) {
        let mut min = 1000;
        let mut min_index = CHAR_COUNT +1;
        for (i,val) in item.iter().enumerate().take(CHAR_COUNT) {
            if *val < min {
                min = *val;
                min_index = i;
            }
        }
        password.push((min_index as u8 + ASCII_OFFSET) as char);
    }
    password
}

fn fill_arr(input: Vec<String>, arr: &mut [[i32; 26]; 8]) {
    for line in input {
        for (i, c) in line.chars().enumerate() {
            arr[i][c as usize - ASCII_OFFSET as usize] += 1;
        }
    }
}
