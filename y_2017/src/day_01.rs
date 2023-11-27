use crate::lib::parse_input;

pub fn part_1() -> i32 {
    let input = parse_input("inputs/day_01.txt")[0].clone();
    calculate_sum(&input.to_string(),1)
}

pub fn part_2() -> i32 {
    let input = parse_input("inputs/day_01.txt")[0].clone();
    calculate_sum(&input.to_string(), input.len() as i32 / 2)
}

fn calculate_sum(input: &String, distance: i32) -> i32 {
    let mut sum = 0;
    let mut prev = input.chars().nth(0).unwrap().to_digit(10).unwrap();
    let mut next = input.chars().nth(0 + distance as usize).unwrap().to_digit(10).unwrap();
    for i in 1..=input.len() {
        if prev == next {
            sum += prev as i32;
        }
        prev = input.chars().nth(i).unwrap_or('0').to_digit(10).unwrap_or(0);
        next = input.chars().nth((i + distance as usize) % input.len()).unwrap().to_digit(10).unwrap();
    }
    sum
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(calculate_sum(&"1122".to_string(), 1), 3);
        assert_eq!(calculate_sum(&"1111".to_string(), 1 ), 4);
        assert_eq!(calculate_sum(&"1234".to_string(), 1), 0);
        assert_eq!(calculate_sum(&"91212129".to_string(), 1), 9);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(calculate_sum(&"1212".to_string(), 2), 6);
        assert_eq!(calculate_sum(&"1221".to_string(), 2), 0);
        assert_eq!(calculate_sum(&"123425".to_string(), 3), 4);
        assert_eq!(calculate_sum(&"123123".to_string(), 3), 12);
        assert_eq!(calculate_sum(&"12131415".to_string(), 4), 4);
    }
}