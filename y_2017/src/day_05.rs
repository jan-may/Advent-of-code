use crate::lib::parse_input;

pub fn part_1() -> i32 {
    let mut instructions = parse_input("inputs/day_05.txt").iter().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    return count_steps(&mut instructions,false)
}

fn count_steps(instructions: &mut Vec<i32>, part2: bool) -> i32 {
    let mut steps = 0;
    let mut index = 0;
    while index < instructions.len() {
        let jump = instructions[index];
        match part2 && jump >= 3 {
            true => instructions[index] -= 1,
            false => instructions[index] += 1
        }
        index = (index as i32 + jump) as usize;
        steps += 1;
    }
    steps
}

pub fn part_2() -> i32 {
    let mut instructions = parse_input("inputs/day_05.txt").iter().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    return count_steps(&mut instructions,true)
}

// tests
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        let mut instructions = vec![0, 3, 0, 1, -3];
        assert_eq!(count_steps(&mut instructions, false), 5);

    }

    #[test]
    fn test_part_2() {
        let mut instructions = vec![0, 3, 0, 1, -3];
        assert_eq!(count_steps(&mut instructions, true), 10);
    }
}