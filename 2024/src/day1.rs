use std::fs::read_to_string;

fn part1() {
    let data = read_to_string("inputs/day1.txt").unwrap();
    let data = data
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let (left, right): (Vec<_>, Vec<_>) =
        data.into_iter().enumerate().partition(|(i, _)| i % 2 == 0);
    let mut right: Vec<usize> = right.into_iter().map(|(_, x)| x).collect();
    right.sort();
    let mut left: Vec<usize> = left.into_iter().map(|(_, x)| x).collect();
    left.sort();

    let mut sum = 0;
    for i in 0..left.len() {
        sum += left[i].abs_diff(right[i]);
    }
    println!("{sum}");
}

fn part2() {
    let data = read_to_string("inputs/day1.txt").unwrap();
    let data = data
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let (left, right): (Vec<_>, Vec<_>) =
        data.into_iter().enumerate().partition(|(i, _)| i % 2 == 0);
    let mut right: Vec<usize> = right.into_iter().map(|(_, x)| x).collect();
    right.sort();
    let left: Vec<usize> = left.into_iter().map(|(_, x)| x).collect();

    let mut sum = 0;
    for i in 0..left.len() {
        let count = right
            .iter()
            .filter(|x| **x == left[i])
            .collect::<Vec<_>>()
            .len();
        sum += left[i] * count;
    }
    println!("{sum}");
}

fn main() {
    part1();
    part2();
}
