use crate::lib::parse_input;
use std::collections::HashSet;

pub fn part_1() -> i32 {
    let (replacements, molecule) = transform_input();
    let mut molecules: HashSet<String> = HashSet::new();
    for (from, to) in replacements {
        for m in molecule.match_indices(&from) {
            let mut new_molecule = molecule.clone();
            new_molecule.replace_range(m.0..m.0 + from.len(), &to);
            molecules.insert(new_molecule);
        }
    }
    molecules.len() as i32
}

pub fn part_2() -> i32 {
    let (replacements, mut molecule) = transform_input();
    let mut steps = 0;

    loop {
        let mut found = false;
        for (from, to) in replacements.iter() {
            if let Some(m) = molecule.find(to) {
                molecule.replace_range(m..m + to.len(), &from);
                steps += 1;
                found = true;
            }
        }
        if !found {
            break;
        }
    }
    steps
}

fn transform_input() -> (Vec<(String, String)>, String) {
    let input = parse_input("inputs/day_19.txt");
    let mut replacements: Vec<(String, String)> = Vec::new();
    let mut molecule = String::new();

    for line in input {
        if line.contains("=>") {
            let mut split = line.split(" => ");
            let from = split.next().unwrap().to_string();
            let to = split.next().unwrap().to_string();
            replacements.push((from, to));
        } else if line.len() > 0 {
            molecule = line;
        }
    }
    (replacements, molecule)
}
