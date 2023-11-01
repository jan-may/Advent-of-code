use crate::lib::parse_input;

pub fn part_1() -> String {
    let input = &parse_input("inputs/day_11.txt")[0];
    find_next_valid_password(&input, 1)
}

pub fn part_2() -> String {
    let input = &parse_input("inputs/day_11.txt")[0];
    find_next_valid_password(&input, 2)
}

fn find_next_valid_password(input: &str, nth_password: usize) -> String {
    let mut password = input.to_string();
    for _ in 0..nth_password {
        loop {
            password = increment_string_by_one(&password);
            if valid_password(&password) {
                break;
            }
        }
    }
    password
}

fn has_three_increasing_letters(s: &str) -> bool {
    let mut chars = s.chars();
    let mut prev = chars.next().unwrap();
    let mut count:usize = 1;
    for c in chars {
        if c as u8 == prev as u8 + 1 {
            count += 1;
            if count == 3 {
                return true;
            }
        } else {
            count = 1;
        }
        prev = c;
    }
    false
}

fn has_no_forbidden_letters(s: &str) -> bool {
    !s.contains('i') && !s.contains('o') && !s.contains('l')
}

fn has_two_non_overlapping_pairs(s: &str) -> bool {
    let mut chars = s.chars();
    let mut prev = chars.next().unwrap();
    let mut count = 0;
    for c in chars {
        if c == prev {
            count += 1;
            if count == 2 {
                return true;
            }
            prev = '\0';
        } else {
            prev = c;
        }
    }
    false
}

fn valid_password(s: &str) -> bool {
    has_three_increasing_letters(s) && has_no_forbidden_letters(s) && has_two_non_overlapping_pairs(s)
}

fn increment_string_by_one(s: &str) -> String {
    let chars = s.chars().rev();
    let mut new_string = String::new();
    let mut carry = true;
    for c in chars {
        if carry {
            if c == 'z' {
                new_string.push('a');
            } else {
                new_string.push((c as u8 + 1) as char);
                carry = false;
            }
        } else {
            new_string.push(c);
        }
    }
    new_string.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_increasing_letters() {
        assert_eq!(has_three_increasing_letters("abc"), true);
        assert_eq!(has_three_increasing_letters("abd"), false);
        assert_eq!(has_three_increasing_letters("hijklmmn"), true);
        assert_eq!(has_two_non_overlapping_pairs("abbceffg"), true);
        assert_eq!(has_two_non_overlapping_pairs("abbcegjk"), false);
        assert_eq!(has_no_forbidden_letters("hijklmmn"), false);
        assert_eq!(has_no_forbidden_letters("abbceffg"), true);
        assert_eq!(valid_password("hijklmmn"), false);
        assert_eq!(valid_password("abbceffg"), false);
        assert_eq!(valid_password("abbcegjk"), false);
        assert_eq!(valid_password("abcdefgh"), false);
        assert_eq!(valid_password("abcdffaa"), true);
        assert_eq!(valid_password("ghjaabcc"), true);
    }
}



