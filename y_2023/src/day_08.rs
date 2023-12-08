use std::collections::HashMap;
use crate::lib::parse_input;

#[derive(Debug)]
struct Node {
    name: String,
    left: String,
    right: String,
}

pub fn part_1() -> i32 {
    let input = refactor_input();
    let directions = &input[0];
    let network = generate_network(&input);
    let steps = calculate_path(directions, &network, network.get("AAA").unwrap(), "part_1");
    steps
}

pub fn part_2() -> i64 {
    let input = refactor_input();
    let directions = &input[0];
    let network = generate_network(&input);
    let nodes = get_all_a_starting_nodes(&network);
    let mut shortest_paths = vec![];

    for node in nodes {
        let current_steps = calculate_path(directions, &network, node, "part_2");
        shortest_paths.push(current_steps as i64);
    }

    lcm_of_array(shortest_paths)
}

fn calculate_path(directions: &str, network: &HashMap<String, Node>, node: &Node, part: &str) -> i32 {
    let mut current = node;
    let mut current_steps = 0;
    'outer: loop {
        for direction in directions.chars() {
            match direction {
                'L' => current = network.get(&current.left).unwrap(),
                'R' => current = network.get(&current.right).unwrap(),
                _ => panic!("Invalid direction"),
            }
            current_steps += 1;
            match part {
                "part_1" => if current.name == "ZZZ" {break 'outer;},
                "part_2" => if current.name.ends_with('Z') {break 'outer;},
                _ => panic!("Invalid part"),
            }
        }
    }
    current_steps
}

fn get_all_a_starting_nodes(network: &HashMap<String, Node>) -> Vec<&Node> {
    let mut current_nodes = vec![];

    // find all nodes that end with A and push them to current_nodes
    for node in network.iter() {
        if node.1.name.ends_with('A') {
            current_nodes.push(node.1);
        }
    }
    current_nodes
}

fn generate_network(input: &[String]) -> HashMap<String, Node> {
    let mut network = HashMap::new();
    for line in input.iter().skip(2) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        network.insert(parts[0].to_string(), Node {
            name: parts[0].to_string(),
            left: parts[1].to_string(),
            right: parts[2].to_string(),
        });
    }
    network
}

fn refactor_input() -> Vec<String> {
    let mut input = parse_input("inputs/day_08.txt");
    let input = input.iter_mut().map(|x| x.replace(" = ", " ").replace(['(', ')', ','], "")).collect::<Vec<String>>();
    input
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    a / gcd(a, b) * b
}

fn lcm_of_array(arr: Vec<i64>) -> i64 {
    arr.iter().fold(1, |acc, &num| lcm(acc, num))
}