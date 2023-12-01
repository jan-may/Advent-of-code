use crate::lib::parse_input;

const NUMS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn part_1() -> i32 {
    let input = parse_input("inputs/day_01.txt");
    let mut result = 0;

    for line in &input {
        let digits: Vec<char> = line.chars().filter(|c| c.is_ascii_digit()).collect();
        let first_digit = digits[0].to_digit(10).unwrap_or(0);
        let last_digit = digits[digits.len() - 1].to_digit(10).unwrap_or(0);
        result += first_digit as i32 * 10 + last_digit as i32;
    }

    result
}


pub fn part_2() -> i32 {
    let input = parse_input("inputs/day_01.txt");
    let mut result = 0;
    for line in &input {
        calc_part2(&mut result, line);
    }
    result
}

fn calc_part2(result: &mut i32, line: &str) {
    let mut nums: Vec<(i32, i32)> = Vec::new();
    // add numeric characters to nums vector
    for (i, char) in line.chars().enumerate() {
        if let Some(digit) = char.to_digit(10) {
            nums.push((digit as i32, i as i32));
        }
    }
    // check if the String contains any numbers from NUMS array and add them + index of end to nums vector
    // find all occurrences of num in line and add them to nums
    for num in &NUMS {
        if line.contains(num) {
            let mut index = 0;
            while let Some(i) = line[index..].find(num) {
                index += i + num.len();
                nums.push((
                    NUMS.iter().position(|&r| r == *num).unwrap() as i32 + 1,
                    index as i32 - 1,
                ));
            }
        }
    }
    // sort the vector by index
    nums.sort_by_key(|&(_, index)| index);
    if let (Some(&(first_num, _)), Some(&(last_num, _))) = (nums.first(), nums.last()) {
        *result += first_num * 10 + last_num;
    }
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {}

    #[test]
    fn test_part_2() {
        let mut result = 0;
        let lines = vec![
            String::from("two1nine"),
            String::from("eightwothree"),
            String::from("abcone2threexyz"),
            String::from("xtwone3four"),
            String::from("4nineeightseven2"),
            String::from("zoneight234"),
            String::from("7pqrstsixteen"),
        ];
        for line in &lines {
            calc_part2(&mut result, line);
        }
        assert_eq!(result, 281);
    }
}
