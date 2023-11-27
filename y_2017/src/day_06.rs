use crate::lib::parse_input;

pub fn part_1() -> i32 {
    let memory_banks = parse_memory_banks(&parse_input("inputs/day_06.txt"));
    let (result, _) = calculate_cycles(memory_banks);
    result
}

pub fn part_2() -> i32 {
    let memory_banks = parse_memory_banks(&parse_input("inputs/day_06.txt"));
    let (_, result) = calculate_cycles(memory_banks);
    result
}

fn calculate_cycles(memory_banks: Vec<i32>) -> (i32,i32) {
    let mut seen_configurations: std::collections::HashMap<Vec<i32>, i32> = std::collections::HashMap::new();
    let mut cycles = 0;
    let mut current_configuration = memory_banks.clone();
    while !seen_configurations.contains_key(&current_configuration) {
        seen_configurations.insert(current_configuration.clone(), cycles);
        let mut max = 0;
        let mut max_index = 0;
        for (index, value) in current_configuration.iter().enumerate() {
            if *value > max {
                max = *value;
                max_index = index;
            }
        }
        current_configuration[max_index] = 0;
        let mut index = max_index;
        while max > 0 {
            index = (index + 1) % current_configuration.len();
            current_configuration[index] += 1;
            max -= 1;
        }
        cycles += 1;
    }
    (cycles, cycles - seen_configurations.get(&current_configuration).unwrap())
}

fn parse_memory_banks(input: &[String]) -> Vec<i32> {
    input[0].split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>()
}

// tests
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        let memory_banks = vec![0, 2, 7, 0];
        assert_eq!(calculate_cycles(memory_banks), (5,4));
    }

    #[test]
    fn test_part_2() {
        let memory_banks = vec![0, 2, 7, 0];
        assert_eq!(calculate_cycles(memory_banks), (5,4));
    }
}