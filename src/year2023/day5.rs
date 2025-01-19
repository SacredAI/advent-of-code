use itertools::Itertools;

pub struct Game {
    seeds: Vec<u32>,
    maps: Vec<Vec<(u32, u32, u32)>>,
}

pub fn parse(input: &str) -> Game {
    let mut parts = input.split("\n\n");

    let seeds = parts.next().unwrap()[7..]
        .split(' ')
        .map(|seed| seed.parse::<u32>().unwrap())
        .collect();

    let from_tos = parts
        .map(|p| {
            let lines = p.lines();

            lines
                .map(|l| {
                    let mut nums = l.split(' ');
                    let dest = nums.next().unwrap().parse::<u32>().unwrap();
                    let source = nums.next().unwrap().parse::<u32>().unwrap();
                    let range = nums.next().unwrap().parse::<u32>().unwrap();
                    (dest, source, range)
                })
                .collect()
        })
        .collect();

    Game {
        seeds,
        maps: from_tos,
    }
}

pub fn part1(input: &Game) -> u32 {
    let locations = input
        .maps
        .iter()
        .fold(input.seeds.clone(), |seeds, mappings| {
            seeds
                .into_iter()
                .map(|seed| {
                    mappings
                        .iter()
                        .find(|&&(_, src, range)| (src..src + range).contains(&seed))
                        .map(|(dst, src, _)| dst + seed - src)
                        .unwrap_or(seed)
                })
                .collect()
        });
    *locations.iter().min().unwrap()
}

pub fn part2(input: &Game) -> u32 {
    let seeds = input
        .seeds
        .clone()
        .into_iter()
        .tuples()
        .map(|(a, len)| (a, a + len))
        .collect::<Vec<_>>();
    let locations = input.maps.iter().fold(seeds, |seeds, mappings| {
        seeds
            .iter()
            .flat_map(|&(start, end)| {
                let mut mapped = Vec::new();
                let mut unmapped = vec![(start, end)];
                for &(dst, src, len) in mappings {
                    let mut m = Vec::new();
                    for (start, end) in unmapped {
                        let a = (start, end.min(src));
                        let b = (start.max(src), (src + len).min(end));
                        let c = ((src + len).max(start), end);
                        if a.0 < a.1 {
                            m.push(a);
                        }
                        if b.0 < b.1 {
                            mapped.push((b.0 - src + dst, b.1 - src + dst));
                        }
                        if c.0 < c.1 {
                            m.push(c);
                        }
                    }
                    unmapped = m;
                }
                mapped.extend(unmapped);
                mapped
            })
            .collect()
    });
    locations.iter().map(|r| r.0).min().unwrap()
}
