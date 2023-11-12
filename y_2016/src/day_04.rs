use std::collections::{BinaryHeap, HashMap};
use crate::lib::parse_input;

pub fn part_1() -> i32 {
    let input = parse_input("inputs/day_04.txt");
    let mut total = 0;
    for line in &input{
        let Ok((name, id, checksum)) = parse_room(line) else {panic!("Invalid room format")};
        let mut vec: HashMap<char, (u8, u8)> = HashMap::new();
        for (i,c) in name.chars().enumerate() {
            if c != '-' {
                let count = vec.entry(c).or_insert((0, i as u8));
                count.0 += 1;
            }
        }
        // order by count, then by smallest index
        let mut heap: BinaryHeap<(u8, u8, char)> = BinaryHeap::new();
        let mut vec = vec.into_iter().collect::<Vec<(char, (u8, u8))>>();
        vec.sort_by(|a, b| {
            match a.1.0.cmp(&b.1.0) {
                std::cmp::Ordering::Equal => a.1.1.cmp(&b.1.1),
                other => other,
            }
        });
        for (c, (count, _)) in vec {
            heap.push((count, 255 - c as u8, c));
        }
        let mut checksum_calc = String::new();
        for _ in 0..5 {
            checksum_calc.push(heap.pop().unwrap().2);
        }
        if checksum == checksum_calc {
            total += id;
        }
    }
    total
}

pub fn part_2() -> i32 {
    let input = parse_input("inputs/day_04.txt") ;
    for line in &input{
        let Ok((name, id, _)) = parse_room(line) else {panic!("Invalid room format")};
        let mut name = name.chars().collect::<Vec<char>>();
        for c in &mut name {
            match *c {
                ' ' => *c = ' ',
                _ => {
                    let mut new_c = *c as u8 + (id % 26) as u8;
                    new_c = if new_c > 122 { new_c - 26 } else { new_c };
                    *c = new_c as char;
                }
            }
        }
        let name = name.into_iter().collect::<String>();
        if name.contains("north") {
            return id;
        }
    }
    0
}

fn parse_room(room: &str) -> Result<(String, i32, String), &'static str> {
    let checksum_start = room.rfind('[').ok_or("Invalid room format: no '[' found")?;
    let (room_code, checksum) = room.split_at(checksum_start);
    let checksum = &checksum[1..checksum.len() - 1];

    let id_start = room_code.rfind('-').ok_or("Invalid room format: no '-' found")?;
    let (name, id) = room_code.split_at(id_start);
    let id = id[1..].parse::<i32>().map_err(|_| "Invalid ID format")?.abs();

    Ok((name.to_string(), id, checksum.to_string()))
}
