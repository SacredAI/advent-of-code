use std::collections::HashMap;

use itertools::Itertools;

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
            possible_ways(cache, &s[1..], Some(1), remaining)
                + possible_ways(cache, &s[1..], None, remaining)
        }
        _ => unreachable!(),
    };
    cache.insert(key, ways);
    ways
}

pub fn parse(input: &str) -> &str {
    input
}

pub fn part1(input: &str) -> usize {
    let mut cache = HashMap::new();
    input
        .split('\n')
        .map(|l| {
            let (vents, rest) = l.split_once(' ').unwrap();
            let nums = rest
                .split(',')
                .map(|w| w.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            cache.clear();
            possible_ways(&mut cache, vents.as_bytes(), None, &nums)
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    let mut cache = HashMap::new();
    input
        .split('\n')
        .map(|l| {
            let (vents, rest) = l.split_once(' ').unwrap();
            let nums = rest
                .split(',')
                .map(|w| w.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            let new_vents = (0..5).map(|_| vents).join("?");
            let new_nums = (0..5).flat_map(|_| &nums).copied().collect::<Vec<_>>();
            cache.clear();
            possible_ways(&mut cache, new_vents.as_bytes(), None, &new_nums)
        })
        .sum()
}
