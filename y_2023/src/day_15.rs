use crate::lib::parse_input;

pub fn part_1() -> usize {
    let input = parse_input("inputs/day_15.txt");
    input[0]
        .split(',')
        .map(calc_hash)
        .sum::<usize>()
}

pub fn part_2() -> usize {
    let input = parse_input("inputs/day_15.txt");
    let parts = input[0].split(',').collect::<Vec<&str>>();
    let mut map: Vec<Vec<(&str, usize)>> = vec![vec![]; 256];

    for line in parts {
        let mut operation = '_';
        let mut operation_index = 0;

        for (i, c) in line.chars().enumerate() {
            match c {
                '=' | '-' => {
                    operation = c;
                    operation_index = i;
                }
                _ => {}
            }
        }

        let label = &line[0..operation_index];
        let value = line[operation_index + 1..].parse::<usize>().unwrap_or_default();
        let hash_value = calc_hash(label);

        match operation {
            '=' => {
                // Check if already exists
                let mut found = false;
                for (i, pair) in map[hash_value].iter_mut().enumerate() {
                    if pair.0 == label {
                        map[hash_value][i].1 = value;
                        found = true;
                        break;
                    }
                }
                if !found {
                    map[hash_value].push((label, value));
                }
            }
            '-' => {
                // Delete if exists
                map[hash_value].retain(|pair| pair.0 != label);
            }
            _ => {}
        }
    }

    let mut result = 0;
    for (i,bucket) in map.iter().enumerate() {
        for (bucket_pos, value) in bucket.iter().enumerate() {
            result += (i+1) * (bucket_pos +1 ) * value.1;
        }
    }
    result
}

fn calc_hash(line: &str) -> usize {
    let mut sum: usize = 0;
    for c in line.chars() {
        sum += c as usize;
        sum *= 17;
        sum %= 256;
    }
    sum
}