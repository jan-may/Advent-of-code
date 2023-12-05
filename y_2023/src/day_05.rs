use crate::lib::parse_input;
use std::ops::{Sub, Add};
use std::fmt::Debug;
use rayon::prelude::*;

#[derive(Default, Debug, Clone)]
struct Mapping {
    name: String,
    mapping: Vec<Map>
}

#[derive(Debug, Clone)]
struct Map {
    source: (i64, i64),
    destination: (i64, i64)
}

#[derive(Debug, Clone, Default)]
struct Seed {
    value: i64,
    soil: i64,
    fertilizer: i64,
    water: i64,
    light: i64,
    temperature: i64,
    humidity: i64,
    location: i64,
}

pub fn part_1() -> i64 {
    let mut input = parse_input("inputs/day_05.txt");
    let mut seeds = get_seeds(&mut input);

    input.remove(0); // remove 1st line - seeds have been extracted

    let mappings = parse_mapping(&mut input);

    for seed in seeds.iter_mut(){
        calculate_seed_mapping(&mappings, seed, "seed-to-soil map", |seed| seed.value, |seed, value| seed.soil = value);
        calculate_seed_mapping(&mappings, seed, "soil-to-fertilizer map", |seed| seed.soil, |seed, value| seed.fertilizer = value);
        calculate_seed_mapping(&mappings, seed, "fertilizer-to-water map", |seed| seed.fertilizer, |seed, value| seed.water = value);
        calculate_seed_mapping(&mappings, seed, "water-to-light map", |seed| seed.water, |seed, value| seed.light = value);
        calculate_seed_mapping(&mappings, seed, "light-to-temperature map", |seed| seed.light, |seed, value| seed.temperature = value);
        calculate_seed_mapping(&mappings, seed, "temperature-to-humidity map", |seed| seed.temperature, |seed, value| seed.humidity = value);
        calculate_seed_mapping(&mappings, seed, "humidity-to-location map", |seed| seed.humidity, |seed, value| seed.location = value)
    }
    // return smallest location
    seeds.iter().min_by_key(|seed| seed.location).unwrap().location
}

// brute fore - takes a while to run
pub fn part_2() -> i64 {
    let mut input = parse_input("inputs/day_05.txt");
    let seed_values = input[0].split(' ').filter_map(|num| num.parse::<i64>().ok()).collect::<Vec<i64>>();
    let mappings = parse_mapping(&mut input);
    let mut seeds = vec![];
    input.remove(0); // remove 1st line - seeds have been extracted

    for i in 0..seed_values.len() {
        if i % 2 == 0 {
            let result = seed_values[i] + seed_values[i + 1] -1;
            seeds.push((seed_values[i], result));
        }
    }

    // each thread computes its own part of the seeds and returns the smallest location
    // after all threads have finished their work the smallest location is returned
    let locations: Vec<i64> = seeds.par_iter().map(|seed_range| {
        let mut local_smallest_location = i64::MAX;
        for i in seed_range.0..=seed_range.1 {
            let mut seed = Seed { value: i, ..Default::default() };
                calculate_seed_mapping(&mappings, &mut seed, "seed-to-soil map", |seed| seed.value, |seed, value| seed.soil = value);
                calculate_seed_mapping(&mappings, &mut seed, "soil-to-fertilizer map", |seed| seed.soil, |seed, value| seed.fertilizer = value);
                calculate_seed_mapping(&mappings, &mut seed, "fertilizer-to-water map", |seed| seed.fertilizer, |seed, value| seed.water = value);
                calculate_seed_mapping(&mappings, &mut seed, "water-to-light map", |seed| seed.water, |seed, value| seed.light = value);
                calculate_seed_mapping(&mappings, &mut seed, "light-to-temperature map", |seed| seed.light, |seed, value| seed.temperature = value);
                calculate_seed_mapping(&mappings, &mut seed, "temperature-to-humidity map", |seed| seed.temperature, |seed, value| seed.humidity = value);
                calculate_seed_mapping(&mappings, &mut seed, "humidity-to-location map", |seed| seed.humidity, |seed, value| seed.location = value);
            if seed.location < local_smallest_location {
                local_smallest_location = seed.location;
            }
        }
        local_smallest_location
    }).collect();

    *locations.iter().min().unwrap_or(&i64::MAX)
}


fn calculate_seed_mapping<T, G, S>(mappings: &[Mapping], seed: &mut Seed, mapping_name: &str, get_source: G, mut set_destination: S)
    where T: Sub<Output=T> + Add<Output=T> + Copy + PartialOrd + From<i64> + Into<i64> + Debug,
          G: Fn(&Seed) -> T,
          S: FnMut(&mut Seed, T) {

    let mapping = mappings.iter().find(|m| m.name == mapping_name).unwrap();
    let source_value = get_source(seed);

    if let Some(map) = mapping.mapping.iter().find(|map| map.source.0 <= source_value.into() && map.source.1 >= source_value.into()) {
        let diff = source_value - T::from(map.source.0);
        let new_value = T::from(map.destination.0) + diff;
        set_destination(seed, new_value);
    } else {
        set_destination(seed, source_value);
    }
}

fn get_seeds(input: &mut [String]) -> Vec<Seed> {
    let seed_values = input[0].split(' ').filter_map(|num| num.parse::<i64>().ok()).collect::<Vec<i64>>();
    let mut seeds = vec![];
    for value in seed_values {
        seeds.push(Seed { value, ..Default::default() });
    }
    seeds
}

fn parse_mapping(input: &mut Vec<String>) -> Vec<Mapping> {
    // cheat to push last mapping
    input.push(": ".to_string());
    let mut mappings = vec![];
    let mut mapping = Mapping::default();
    for line in input {
        if line.is_empty() {
            continue;
        }
        if line.contains(':') {
            if !mapping.name.is_empty() {
                mappings.push(mapping.clone());
            }
            mapping = Mapping::default();
            mapping.name = line.replace(':', "");
        } else {
            let mut parts = line.split(' ');
            let dest_range_start = parts.next().unwrap().parse::<i64>().unwrap();
            let src_range_start = parts.next().unwrap().parse::<i64>().unwrap();
            let range_length = parts.next().unwrap().parse::<i64>().unwrap();
            let dest = (dest_range_start, dest_range_start + range_length - 1);
            let src = (src_range_start, src_range_start + range_length - 1);
            mapping.mapping.push(Map { source: src, destination: dest });
        }
    }
    mappings
}