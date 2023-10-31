use crate::lib::parse_input;

pub fn part_1() -> i32 {
    let input = parse_input("inputs/day_08.txt");
    return input.iter().map(|s|  s.len() as i32 - count_string_chars(s)).sum();
}

pub fn part_2() -> i32 {
    let input = parse_input("inputs/day_08.txt");
    return input.iter().map(|s| count_encoded_chars(s) - s.len() as i32).sum();
}

fn count_string_chars(s: &str) -> i32 {
    let mut count = 0;
    // Skip the first and last character
    let mut chars = s.chars().skip(1).take(s.len() - 2);

    while let Some(c) = chars.next() {
        if c == '\\' {
            if let Some(next_char) = chars.next() {
                match next_char {
                    'x' => {
                        // Skip the next two characters as they represent a hex escape
                        count += 1;
                        chars.next();
                        chars.next();
                    }
                    '\\' | '"' => {
                        // These are escaped characters, count as one
                        count += 1;
                    }
                    _ => panic!("Invalid escape sequence"),
                }
            }
        } else {
            count += 1;
        }
    }
    count
}

fn count_encoded_chars(s: &str) -> i32 {
    let mut count = 0;
    for c in s.chars() {
        if c == '"' || c == '\\' {
            count += 1;
        }
        count += 1;
    }
    count + 2
}


