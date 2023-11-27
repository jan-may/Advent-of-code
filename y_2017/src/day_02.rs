use crate::lib::parse_input;

pub fn part_1() -> i32 {
    let input = parse_input("inputs/day_02.txt");
    return calculate_checksum(input)
}

pub fn part_2() -> i32 {
    let input = parse_input("inputs/day_02.txt");
    return calculate_divisible_checksum(input)
}

fn calculate_checksum(input: Vec<String>) -> i32 {
    let mut checksum = 0;
    for line in input {
        let data = line.split_whitespace().into_iter().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let min = data.iter().min().unwrap();
        let max = data.iter().max().unwrap();
        let diff = max - min;
        checksum += diff;
    }
    checksum
}

fn calculate_divisible_checksum(input: Vec<String>) -> i32 {
    let mut checksum = 0;
    for line in input {
        let data = line.split_whitespace().into_iter().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        'outer: for i in 0..data.len() {
            for j in 0..data.len() {
                if i == j { continue; }
                let max = data[i].max(data[j]);
                let min = data[i].min(data[j]);
                if max % min != 0 { continue; }
                checksum += max / min;
                break 'outer;
            }
        }
    }
    checksum
}


// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = vec![
            String::from("5 1 9 5"),
            String::from("7 5 3"),
            String::from("2 4 6 8")
        ];
        assert_eq!(calculate_checksum(input), 18);
    }

    #[test]
    fn test_part_2() {
        let input = vec![
            String::from("5 9 2 8"),
            String::from("9 4 7 3"),
            String::from("3 8 6 5")
        ];
        assert_eq!(calculate_divisible_checksum(input), 9);

    }
}