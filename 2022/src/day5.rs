use itertools::Itertools;


pub struct Game {
    seeds: Vec<u32>,
    maps: Vec<Vec<(u32, u32, u32)>>
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Game {
    let mut parts = input.split("\n\n");

    let seeds = parts.next().unwrap()[7..]
    .split(' ')
    .map(|seed| seed.parse::<u32>().unwrap())
    .collect();

    let fromTos = parts.map(|p| {
        let mut lines = p.lines();
        let (from, to) = lines.next().unwrap().split_once(" map:").unwrap().0.split_once("-to-").unwrap();

        lines.map(|l| {
            let mut nums = l.split(" ");
            let dest = nums.next().unwrap().parse::<u32>().unwrap();
            let source = nums.next().unwrap().parse::<u32>().unwrap();
            let range = nums.next().unwrap().parse::<u32>().unwrap();
            (dest, source, range)
        }).collect()
    }).collect();

    Game { seeds: seeds, maps: fromTos }
}

#[aoc(day5, part1)]
pub fn part1(input: &Game) -> u32 {
    let locations = input.maps.iter().fold(input.seeds.clone(), |seeds, mappings|{
        seeds.into_iter().map(|seed|{
            mappings.iter()
            .find(|&&(_, src, range)| (src..src+range).contains(&seed))
            .map(|(dst, src, _)| dst + seed - src)
            .unwrap_or(seed)
        }).collect()
    });
    *locations.iter().min().unwrap()
}

#[aoc(day5, part2)]
pub fn part2(input: &Game) -> u32 {
    let seeds = input.seeds.clone().into_iter().tuples().map(|(a, len)|  (a, a + len)).collect::<Vec<_>>();
    let locations = input.maps.iter().fold(seeds, |seeds, mappings| seeds.iter().flat_map(|&(start, end)| {
        let mut mapped = Vec::new();
        let mut unmapped = vec![(start, end)];
        for &(dst, src, len) in mappings {
          let mut m = Vec::new();
          for (start, end) in unmapped {
            let a = (start, end.min(src));
            let b = (start.max(src), (src+len).min(end));
            let c = ((src+len).max(start), end);
            if a.0 < a.1 { m.push(a); }
            if b.0 < b.1 { mapped.push((b.0-src+dst, b.1-src+dst)); }
            if c.0 < c.1 { m.push(c); }
          }
          unmapped = m;
        }
        mapped.extend(unmapped);
        mapped
      }).collect());
    locations.iter().map(|r| r.0).min().unwrap()
}

#[cfg(test)]
mod tests{
    use crate::day4::*;
    #[test]
    fn p1(){
        let i = input_generator("seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4");
        let result = part1(&i);
        assert_eq!(result, 35);
    }

//     #[test]
//     fn p2(){
//         let mut i = input_generator("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
// Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
// Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
// Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
// Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
// Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11");
//         let result = part2(&mut i);
//         assert_eq!(result, 30);
//     }
}
