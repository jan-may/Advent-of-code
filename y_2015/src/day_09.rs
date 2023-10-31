use crate::lib::parse_input;
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug)]
struct Location {
    name: String,
    neighbors: HashMap<String, u32>,
}

pub fn part_1() -> i32 {
    let input = parse_input("inputs/day_09.txt");
    let graph = build_graph(&input);
    let shortest_distance = find_shortest_distance(&graph);
    shortest_distance as i32
}

pub fn part_2() -> i32 {
    let input = parse_input("inputs/day_09.txt");
    let graph = build_graph(&input);
    let longest_distance = find_longest_distance(&graph);
    longest_distance as i32
}

fn build_graph(input: &Vec<String>) -> HashMap<String, Location> {
    let mut graph: HashMap<String, Location> = HashMap::new();

    for line in input {
        let mut split = line.split(" = ");
        let mut locations = split.next().unwrap().split(" to ");

        let location_1 = locations.next().unwrap().to_string();
        let location_2 = locations.next().unwrap().to_string();
        let distance = split.next().unwrap().parse::<u32>().unwrap();

        insert_location(&mut graph, &location_1);
        insert_location(&mut graph, &location_2);

        graph.get_mut(&location_1).unwrap().neighbors.insert(location_2.clone(), distance);
        graph.get_mut(&location_2).unwrap().neighbors.insert(location_1.clone(), distance);
    }

    graph
}

fn insert_location(graph: &mut HashMap<String, Location>, location_name: &str) {
    if !graph.contains_key(location_name) {
        graph.insert(
            location_name.to_string(),
            Location {
                name: location_name.to_string(),
                neighbors: HashMap::new(),
            },
        );
    }
}

fn find_shortest_distance(graph: &HashMap<String, Location>) -> u32 {
    let mut shortest_distance = u32::MAX;
    for permutation in graph.keys().permutations(graph.len()) {
        let distance = calculate_total_distance(&permutation, graph);

        if distance < shortest_distance {
            shortest_distance = distance;
        }
    }
    shortest_distance
}

fn find_longest_distance(graph: &HashMap<String, Location>) -> u32 {
    let mut longest_distance = 0;
    for permutation in graph.keys().permutations(graph.len()) {
        let distance = calculate_total_distance(&permutation, graph);

        if distance > longest_distance {
            longest_distance = distance;
        }
    }
    longest_distance
}

fn calculate_total_distance(permutation: &Vec<&String>, graph: &HashMap<String, Location>) -> u32 {
    let mut distance = 0;
    for i in 0..permutation.len() - 1 {
        distance += graph[permutation[i]].neighbors[permutation[i + 1]];
    }
    distance
}
