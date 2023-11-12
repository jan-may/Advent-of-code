use crate::lib::parse_input;

pub fn part_1() -> usize {
    let input = parse_input("inputs/day_07.txt");
    let mut total = 0;
    for ip in input{
        let mut abba = false;
        let mut hypernet_abba = false;
        for (i, part) in ip.split(|c| c == '[' || c == ']').enumerate() {
            // every second part is hypernet
            match i % 2 {
                0 => abba = abba || contains_abba(part),
                _ => hypernet_abba = hypernet_abba || contains_abba(part),
            }
        }
        total += match abba && !hypernet_abba {
            true => 1,
            false => 0,
        };
    }
    total
}


pub fn part_2() -> usize {
    let input = parse_input("inputs/day_07.txt");
    let mut total = 0;
    for ip in input{
        let mut abas = Vec::new();
        let mut babs = Vec::new();
        for (i, part) in ip.split(|c| c == '[' || c == ']').enumerate() {
        // every second part is hypernet
            match i % 2 {
                0 => abas.append(&mut get_aba_triple(part)),
                _ => babs.append(&mut get_aba_triple(part)),
            }
        }
        total += match abas.iter().any(|aba| babs.iter().any(|bab| aba[0..2] == bab[1..3] && aba[1..3] == bab[0..2])) {
            true => 1,
            false => 0,
        };
    }
    total
}

fn contains_abba(s: &str) -> bool {
    s.as_bytes().windows(4).any(|w| w[0] == w[3] && w[1] == w[2] && w[0] != w[1])
}

fn get_aba_triple(s: &str) -> Vec<String> {
    s.chars().collect::<Vec<char>>()
        .windows(3)
        .filter(|w| w[0] == w[2] && w[0] != w[1])
        .map(|w| w.iter().collect::<String>())
        .collect()
}
