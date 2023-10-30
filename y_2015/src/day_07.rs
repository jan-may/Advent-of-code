use std::collections::HashMap;
use std::fs;

pub fn part_1() -> u16 {
    let input = parse_input("inputs/day_07.txt");
    let mut cache: HashMap<String, u16> = HashMap::new();
    evaluate_signal(&input, "a", &mut cache)
}

pub fn part_2(new_value: u16) -> u16 {
    let input = parse_input("inputs/day_07.txt");
    let mut cache: HashMap<String, u16> = HashMap::new();
    cache.insert("b".to_string(), new_value);
    evaluate_signal(&input, "a", &mut cache)
}

fn parse_input(filename: &str) -> HashMap<String, String> {
    let mut commands = HashMap::new();
    if let Ok(contents) = fs::read_to_string(filename) {
        for line in contents.lines() {
            let parts: Vec<&str> = line.split(" -> ").collect();
            if parts.len() == 2 {
                let signal = parts[0].to_string();
                let wire = parts[1].to_string();
                commands.insert(wire, signal);
            }
        }
    }
    commands
}


fn evaluate_signal(commands: &HashMap<String, String>, wire: &str, cache: &mut HashMap<String, u16>) -> u16 {

    if let Ok(signal) = wire.parse::<u16>() {
        return signal;
    }

    if let Some(&cached_signal) = cache.get(wire) {
        return cached_signal;
    }

    let command = commands.get(wire).unwrap();
    let result = evaluate_command(commands, command, cache);
    cache.insert(wire.to_string(), result);
    result
}

fn evaluate_command(commands: &HashMap<String, String>, command: &str, cache: &mut HashMap<String, u16>) -> u16 {
    let parts: Vec<&str> = command.split_whitespace().collect();
    match parts.len() {
        1 => evaluate_signal(commands, parts[0], cache),
        2 => !evaluate_signal(commands, parts[1], cache),
        3 => {
            let signal1 = evaluate_signal(commands, parts[0], cache);
            let signal2 = evaluate_signal(commands, parts[2], cache);
            match parts[1] {
                "AND" => signal1 & signal2,
                "OR" => signal1 | signal2,
                "LSHIFT" => signal1 << signal2,
                "RSHIFT" => signal1 >> signal2,
                _ => panic!("Unknown operation"),
            }
        }
        _ => panic!("Unknown command format"),
    }
}

