use crate::lib::parse_input;

pub fn part_1() -> usize {
    let input = parse_input("inputs/day_09.txt")[0].clone();
    let mut decompressed = String::new();
    let mut chars = input.chars();
    while let Some(c) = chars.next() {
        if c == '(' {
            let mut marker = String::new();
            while let Some(c) = chars.next() {
                if c == ')' {
                    break;
                }
                marker.push(c);
            }
            let mut marker = marker.split('x');
            let length = marker.next().unwrap().parse::<usize>().unwrap();
            let times = marker.next().unwrap().parse::<usize>().unwrap();
            let mut temp = String::new();
            for _ in 0..length {
                temp.push(chars.next().unwrap());
            }
            for _ in 0..times {
                decompressed.push_str(&temp);
            }
        } else {
            decompressed.push(c);
        }
    }
    decompressed.len()
}

pub fn part_2() -> usize {
    let input = "X(8x2)(3x3)ABCY".to_string();
    let mut decompressed = String::new();
    let mut chars = input.chars().rev();
    while let Some(c) = chars.next() {
        if c == ')' {
            let mut marker = String::new();
            while let Some(c) = chars.next() {
                if c == '(' {
                    break;
                }
                marker.push(c);
            }
            let mut marker = marker.split('x');
            let length = marker.next().unwrap().parse::<usize>().unwrap();
            let times = marker.next().unwrap().parse::<usize>().unwrap();
            let mut temp = String::new();
            for _ in 0..length {
                temp.push(chars.next().unwrap());
            }
            for _ in 0..times {
                decompressed.push_str(&temp);
            }
        } else {
            decompressed.push(c);
        }
    }
    decompressed.len()
}