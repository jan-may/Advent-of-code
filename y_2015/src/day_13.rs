use std::collections::HashMap;
use crate::lib::parse_input;
use itertools::Itertools;

#[derive(Debug)]
struct Neighbor {
    name: String,
    happiness: i32,
}

pub fn part_1() -> i32 {
    let input = get_input();
    let map = create_mapping(&input);
    let people: Vec<String> = map.keys().map(|s| s.to_string()).collect();
    return calculate_all_possible_permutations(map, people);
}

fn calculate_all_possible_permutations(map: HashMap<String, HashMap<String, i32>>, people: Vec<String>) -> i32{
    let mut max = 0;
    for permutation in people.iter().permutations(people.len()) {
        let mut total = 0;
        for i in 0..permutation.len() {
            // only need to check one direction
            let curr = permutation[i];
            let next = permutation[(i + 1) % permutation.len()];
            total += map.get(curr).unwrap().get(next).unwrap_or(&0);
            total += map.get(next).unwrap().get(curr).unwrap_or(&0);
        }
        max = std::cmp::max(max, total);
    }
    max
}

fn create_mapping(input: &Vec<String>) -> HashMap<String, HashMap<String, i32>> {
    let mut map: HashMap<String, HashMap<String, i32>> = HashMap::new();
    for line in input {
        let mut split = line.split_whitespace();
        let name = split.next().unwrap().to_string();
        let happiness = split.next().unwrap().parse::<i32>().unwrap();
        let neighbor = split.next().unwrap().to_string();
        let neighbor_map = map.entry(name).or_insert(HashMap::new());
        neighbor_map.insert(neighbor, happiness);
    }
    map
}


pub fn part_2() -> i32 {
    let input = get_input();
    let mut map = create_mapping(&input);
    let people: Vec<String> = map.keys().map(|s| s.to_string()).collect();
    let mut me = HashMap::new();
    for person in people.iter() {
        me.insert(person.to_string(), 0);
    }
    map.insert("me".to_string(), me);
    let people: Vec<String> = map.keys().map(|s| s.to_string()).collect();
    return calculate_all_possible_permutations(map, people);
}

pub fn get_input() -> Vec<String> {
    return parse_input("inputs/day_13.txt")
        .iter()
        .map(|s| {
            s.to_string()
                .replace("would gain ", "")
                .replace("would lose ", "-")
                .replace(" happiness units by sitting next to ", " ")
                .replace(".", "")
        })
        .collect_vec();
}
