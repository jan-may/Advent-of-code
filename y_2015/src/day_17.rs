use itertools::Itertools;
use crate::lib::parse_input;

pub fn part_1() -> i32 {
    let containers = parse_input("inputs/day_17.txt").iter().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let mut count = 0;
    for i in 1..containers.len() {
        for combination in containers.clone().into_iter().combinations(i) {
            if combination.iter().sum::<i32>() == 150 {
                count += 1;
            }
        }
    }
    count
}

pub fn part_2() -> i32 {
    let containers = parse_input("inputs/day_17.txt").iter().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let mut count = 0;

    let mut min = 0;
    for i in 1..containers.len() {
        for combination in containers.clone().into_iter().combinations(i) {
            if combination.iter().sum::<i32>() == 150 {
                if min == 0 {
                    min = combination.len();
                }
                if combination.len() == min {
                    count += 1;
                }
            }
        }
        if count > 0 {
            break;
        }
    }
    count
}