use crate::lib::parse_input;

pub fn part_1() -> i32 {
    let input = parse_input("inputs/day_20.txt").clone()[0].parse::<usize>().unwrap();
    let mut houses:Vec<u64> = vec![0u64; 1000000];
    let limit = 1000000;
    for elf in 1..limit {
        let presents = (elf * 10) as u64;
        for house in (elf.. houses.len()).step_by(elf) {
            let house = house;
            houses[house] += presents;
            if houses[house] >=  input as u64 {
                return house as i32;
            }
        }
    }
    panic!("No house found")
}

pub fn part_2() -> i32 {
    let input = parse_input("inputs/day_20.txt").clone()[0].parse::<usize>().unwrap();
    let mut houses: Vec<u64> = vec![0u64; 1000000];
    let limit = 1000000;
    for elf in 1..limit {
        for visited in (elf..limit.min(elf * 50)).step_by(elf as usize) {
            houses[visited] += elf as u64 * 11;
            if houses[visited] >= input as u64 && visited < limit {
                return visited as i32;
            }
        }
    }
    panic!("No house found")
}
