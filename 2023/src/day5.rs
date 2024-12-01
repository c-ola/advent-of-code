use std::cmp::{max, min};
use std::ops::{Add, Range};

#[derive(Clone, Copy, Debug)]
struct RangeMap<T>
where
    T: PartialOrd + PartialEq + Ord,
{
    source: T,
    dest: T,
    range: T,
}

impl RangeMap<u64> {
    fn source(&self) -> Range<u64> {
        self.source..(self.source + self.range)
    }

    fn dest(&self) -> Range<u64> {
        self.dest..(self.dest + self.range)
    }

    fn offset(&self) -> i64 {
        self.dest as i64 - self.source as i64
    }

    fn get(&self, num: u64) -> Option<u64> {
        if self.has_source(num) {
            let diff = self.dest as i64 - self.source as i64;
            return Some((num as i64 + diff as i64) as u64);
        }
        None
    }

    fn has_source(&self, num: u64) -> bool {
        if num >= self.source && num <= self.source + self.range {
            return true;
        }
        false
    }
}

trait Intersection<T> {
    fn intersection(&self, other: &Self) -> Option<Self>
    where
        Self: Sized;
    fn split(&self, other: &Self) -> Vec<Self>
    where
        Self: Sized;
}

impl<T> Intersection<T> for Range<T>
where
    T: Ord + Copy + Add<Output = T>,
{
    // 10..20.intersection(0..15) -> 10..15
    fn intersection(&self, other: &Range<T>) -> Option<Range<T>> {
        if self.end < other.start || self.start > other.end {
            return None;
        }
        let start = max(self.start, other.start);
        let end = min(self.end, other.end);
        Some(start..end)
    }

    // 10..20.intersection(0..15) -> [0..10, 10..15, 15..20]
    fn split(&self, splitter: &Range<T>) -> Vec<Range<T>> {
        let mut result: Vec<Range<T>> = Vec::new();
        if let Some(intersection) = splitter.intersection(self) {
            result.push(splitter.start..intersection.start);
            result.push(intersection.end..splitter.end);
            result.push(intersection);
        } else {
            result.push(self.start..self.end);
        }
        result
    }
}

fn split_map(accepter: &Range<u64>, map: &RangeMap<u64>) -> Vec<Range<u64>> {
    let mut result: Vec<Range<u64>> = Vec::new();
    let splitter = map.source();
    if let Some(intersection) = splitter.intersection(accepter) {
        result.push(accepter.start..intersection.start);
        result.push(intersection.end..accepter.end);
        let mapped = (intersection.start as i64 + map.offset()) as u64
            ..(intersection.end as i64 + map.offset()) as u64;
        result.push(mapped);
    } else {
        result.push(accepter.start..accepter.end);
    }
    result
}

// similarities between ranges
//
// 0..50 -> 50..100
// next map
// 60..90 -> 80..110 => only accepts range 60..90, rest are passed through
// next maps
// 75..85 -> 10..20 & 100..105 -> 115->130 => accepts 80..85 & 100..105, rest are passed through

// start with first range from seeds say
// 70..100
// pass that range through to each map, checking for

fn get_mapped_value(j: u64, maps: &Vec<Vec<RangeMap<u64>>>) -> u64 {
    let mut key: u64 = j;
    for i in 0..maps.len() {
        for map in 0..maps[i].len() {
            match maps[i][map].get(key) {
                Some(v) => {
                    key = v;
                    break;
                }
                None => (),
            }
        }
    }

    key
}

fn get_layer_map(lines: Vec<String>) -> Vec<Vec<RangeMap<u64>>> {
    let mut maps: Vec<Vec<RangeMap<_>>> = vec![Vec::new(); 7];
    let mut i = 2;
    let mut cur_map = 0;

    while i < lines.len() {
        let line = lines[i].clone();

        if line.contains("map") {
            cur_map += 1;
        } else {
            let numbers: Vec<_> = line.split(" ").collect();
            if numbers.len() != 3 {
                i += 1;
                continue;
            }
            let dest = numbers[0].parse::<u64>().unwrap();
            let source = numbers[1].parse::<u64>().unwrap();
            let range = numbers[2].parse::<u64>().unwrap() - 1;

            let map = RangeMap::<u64> {
                source,
                dest,
                range,
            };
            maps[cur_map - 1].push(map);
        }
        i += 1;
    }
    maps
}

fn part1() {
    let lines: Vec<String> = include_str!("../inputs/day5")
        .lines()
        .map(|s| s.to_string())
        .collect();

    let seeds: Vec<_> = lines[0]
        .split(": ")
        .nth(1)
        .unwrap()
        .split(' ')
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let maps = get_layer_map(lines);

    let mut min = u64::MAX;
    for seed in seeds {
        let key: u64 = get_mapped_value(seed, &maps);

        if key < min {
            min = key;
        }
    }
    println!("Location: {min}");
}

fn part2() {
    let lines: Vec<String> = include_str!("../inputs/day5")
        .lines()
        .map(|s| s.to_string())
        .collect();

    let seed_values: Vec<_> = lines[0]
        .split(": ")
        .nth(1)
        .unwrap()
        .split(' ')
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let mut seed_ranges = Vec::with_capacity(seed_values.len() / 2);

    for i in 0..seed_values.len() / 2 {
        let start = seed_values[i * 2];
        let end = start + seed_values[i * 2 + 1];
        let range = start..end;
        //seed_ranges.push(start..start);
        //seed_ranges.push(end..end);
        seed_ranges.push(range);
    }

    // 79..98
    // 98..100 -> 50->52
    let maps = get_layer_map(lines);

    //let m = RangeMap {source: 100, dest: 0, range: 10};
    //let r = 105..120;
    //let s = split_map(&r, &m);
    //panic!("{s:?}");



    let mut min = u64::MAX;
    use std::collections::HashSet;
    for seed in seed_ranges {
        // split seed ranges with accepting and nonaccepting
        println!("seed: {seed:?}");
        let mut stack: HashSet<Range<u64>> = HashSet::new();
        let mut acc: HashSet<Range<u64>> = HashSet::new();
        let mut unmapped: HashSet<Range<u64>> = HashSet::new();
        let mut comp: HashSet<Range<u64>> = HashSet::new();
        stack.insert(seed);
        for map in &maps {
            for maprange in &*map {
                //println!("{maprange:?}");
                for key in &stack {
                    let mapped = split_map(key, &maprange);
                    //println!("split: {mapped:?}");
                    match mapped.get(2) {
                        Some(range) => {
                            acc.insert(range.clone());
                            let c1 = mapped[0].clone().count();
                            let c2 = mapped[1].clone().count();

                            if c1 == 0 && c2 == 0 {
                                comp.insert(key.clone());
                            }

                            if c1 != 0 {
                                acc.insert(mapped[0].clone());
                            }
                            if c2 != 0 {
                                acc.insert(mapped[1].clone());
                            }

                        }
                        None => {
                            if !comp.contains(&mapped[0]) {
                                unmapped.insert(mapped[0].clone());
                            } 
                        }
                    }
                }
            }
            for key in &unmapped {
                if !comp.contains(key) {
                    acc.insert(key.clone());
                }

            }
            stack = acc.clone();
            acc.clear();
            unmapped.clear();
            comp.clear();
            //println!("{stack:?}");
            //panic!();
        }

        for i in stack {
            if i.start < min {
                //println!("Min loc: {min}");
                min = i.start;
            }
        }

        //println!("{stack:?}");
    }
    println!("Min loc: {min}");
}

fn part23() {
    let lines: Vec<String> = include_str!("../inputs/day5")
        .lines()
        .map(|s| s.to_string())
        .collect();

    let seed_values: Vec<_> = lines[0]
        .split(": ")
        .nth(1)
        .unwrap()
        .split(' ')
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let mut seed_ranges = Vec::with_capacity(seed_values.len() / 2);

    for i in 0..seed_values.len() / 2 {
        let start = seed_values[i * 2];
        let end = start + seed_values[i * 2 + 1];
        let range = start..end;
        //seed_ranges.push(start..start);
        //seed_ranges.push(end..end);
        seed_ranges.push(range);
    }

    // 79..98
    // 98..100 -> 50->52
    let maps = get_layer_map(lines);
    
    use rayon::prelude::*;
    let mut min = u64::MAX;
    for seed in seed_ranges {
        println!("{seed:?}");
        let r = seed.clone();
        let key = r.into_par_iter()
            .map(|j| get_mapped_value(j, &maps))
            .min().unwrap();

        println!("{key}");

        if key < min {
            min = key;
        }
    }

    println!("Min loc: {min}");
}


fn main() {
    part1();
    part2();
    part23();
}
