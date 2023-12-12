use crate::lib::parse_input;

pub fn part_1() -> i32 {
    let input = parse_input("inputs/day_12.txt");
    let mut count = 0;
    for row in input {
        let parts = row.split(' ').collect::<Vec<&str>>();
        let mut springs = parts[0].chars().collect::<Vec<char>>();
        let mut indices = vec![];
        for (i, c) in springs.iter().enumerate() {
            if *c == '#' {
                indices.push(i);
            }
        }
        let damaged_groups = parts[1].split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        count += find_combinations(&mut springs, &damaged_groups);
    }
    count
}

pub fn part_2() -> i32 {
    0
}


fn generate_and_validate(spring: &mut Vec<char>, groups: &Vec<i32>, index: usize, count: &mut i32) {
    if index == spring.len() {
        if is_valid(spring, groups) {
            *count += 1;
        }
        return;
    }

    if !grouping_still_possible(spring, groups, index) {
        return;
    }

    if spring[index] == '?' {
        // Try with '.'
        spring[index] = '.';
        generate_and_validate(spring, groups, index + 1, count);
        spring[index] = '#';
        generate_and_validate(spring, groups, index + 1, count);

        // Reset to '?'
        spring[index] = '?';
    } else {
        generate_and_validate(spring, groups, index + 1, count);
    }
}

fn find_combinations(springs: &mut Vec<char>, groups: &Vec<i32>) -> i32 {
    let mut count = 0;
    generate_and_validate(springs, groups, 0, &mut count);
    count
}

fn grouping_still_possible(_spring: &[char], _groups: &[i32], _index: usize) -> bool {
    // still needs to be implemented
    true
}

fn is_valid(spring: &[char], groups: &Vec<i32>) -> bool {
    let mut group_counts = Vec::new();
    let mut current_count = 0;

    for &c in spring {
        if c == '#' {
            current_count += 1;
        } else if current_count > 0 {
            group_counts.push(current_count);
            current_count = 0;
        }
    }
    if current_count > 0 {
        group_counts.push(current_count);
    }
    group_counts == *groups
}