use crate::lib::parse_input;
use std::collections::HashMap;

pub fn part_1() -> usize {
    let input = parse_and_prepare_input("inputs/day_12.txt", 1, "");
    input.into_iter()
        .map(|(vents, nums)| possible_ways(&mut HashMap::new(), &vents, None, &nums))
        .sum()
}

pub fn part_2() -> usize {
    let input = parse_and_prepare_input("inputs/day_12.txt", 5, "?");
    input.into_iter()
        .map(|(vents, nums)| possible_ways(&mut HashMap::new(), &vents, None, &nums.repeat(5)))
        .sum()
}

fn parse_and_prepare_input(file_path: &str, repeat_factor: usize, separator: &str) -> Vec<(Vec<u8>, Vec<usize>)> {
    parse_input(file_path)
        .into_iter()
        .map(|line| {
            let (vents, rest) = line.split_once(' ').expect("Invalid input format");
            let nums = rest.split(',')
                .map(|w| w.parse::<usize>().expect("Invalid number"))
                .collect::<Vec<_>>();
            let new_vents = if repeat_factor > 1 {
                (0..repeat_factor).map(|_| vents).collect::<Vec<_>>().join(separator)
            } else {
                vents.to_string()
            };

            (new_vents.into_bytes(), nums)
        })
        .collect()
}

// stolen from @https://github.com/AxlLind
// couldn't figure out how to use the cache properly :(
fn possible_ways(
    cache: &mut HashMap<(usize, usize, usize), usize>,
    s: &[u8],
    within: Option<usize>,
    remaining: &[usize],
) -> usize {
    if s.is_empty() {
        return match (within, remaining.len()) {
            (None, 0) => 1,
            (Some(x), 1) if x == remaining[0] => 1,
            _ => 0,
        };
    }
    if within.is_some() && remaining.is_empty() {
        return 0;
    }

    let key = (s.len(), within.unwrap_or(0), remaining.len());
    if let Some(&x) = cache.get(&key) {
        return x;
    }

    let ways = match (s[0], within) {
        (b'.', Some(x)) if x != remaining[0] => 0,
        (b'.', Some(_)) => possible_ways(cache, &s[1..], None, &remaining[1..]),
        (b'.', None) => possible_ways(cache, &s[1..], None, remaining),
        (b'#', Some(_)) => possible_ways(cache, &s[1..], within.map(|x| x + 1), remaining),
        (b'#', None) => possible_ways(cache, &s[1..], Some(1), remaining),
        (b'?', Some(x)) => {
            let mut ans = possible_ways(cache, &s[1..], within.map(|x| x + 1), remaining);
            if x == remaining[0] {
                ans += possible_ways(cache, &s[1..], None, &remaining[1..])
            }
            ans
        }
        (b'?', None) => {
            possible_ways(cache, &s[1..], Some(1), remaining) + possible_ways(cache, &s[1..], None, remaining)
        }
        _ => unreachable!(),
    };
    cache.insert(key, ways);
    ways
}