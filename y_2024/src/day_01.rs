use crate::lib::parse_input;

fn parse_and_split_input(file_path: &str) -> (Vec<i32>, Vec<i32>) {
    let input = parse_input(file_path);
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for line in input {
        let mut split = line.split_whitespace();
        list1.push(split.next().unwrap().parse::<i32>().unwrap());
        list2.push(split.next().unwrap().parse::<i32>().unwrap());
    }

    (list1, list2)
}

pub fn part_1() -> i32 {
    let (mut list1, mut list2) = parse_and_split_input("inputs/day_01.txt");
    let mut sum = 0;

    list1.sort();
    list2.sort();

    // calculate the sum of the absoulte! differences between each element in the lists
    for i in 0..list1.len() {
        sum += (list1[i] - list2[i]).abs();
    }
    sum
}

pub fn part_2() -> i32 {
    let (list1, list2) = parse_and_split_input("inputs/day_01.txt");
    let mut sum = 0;
    let mut hash_map = std::collections::HashMap::new();
    for i in &list1 {
        hash_map.insert(i, 0);
    }

    // iterate through list2 and check if the value is in the hashmap if yes increment the value
    for i in list2 {
        if hash_map.contains_key(&i) {
            let count = hash_map.get_mut(&i).unwrap();
            *count += 1;
        }
    }

    // calulate similarity between the two lists
    for i in &list1 {
        if hash_map.contains_key(&i) {
            let count = hash_map.get(&i).unwrap();
            sum += i * count;
        }
    }
    sum
}
