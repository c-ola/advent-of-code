// AOC day 1 Trebuchet
// https://adventofcode.com/2023/day/1

use std::fs::File;
use std::io::{self, BufRead};

const STRNUMS: [(&str, char); 9] = [
    ("one", '1'),
    ("two", '2'),
    ("three", '3'),
    ("four", '4'),
    ("five", '5'),
    ("six", '6'), 
    ("seven", '7'), 
    ("eight", '8'), 
    ("nine", '9'),
];

fn part1(){
    let file = File::open("./inputs/day1").unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut sum = 0;

    for line in lines {
        let line = line.unwrap();

        let v: Vec<u64> = line
            .matches(char::is_numeric)
            .map(|s| s.parse().unwrap())
            .collect();

        let first = v.first().unwrap();
        let second = v.last().unwrap();
        sum += first * 10 + second;
    }
    println!("p2 sum: {sum}")
}

fn part2(){
    let file = File::open("./inputs/day1").unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut sum = 0;

    for line in lines {
        let mut line = line.unwrap();

        STRNUMS.iter().for_each(|s| {
            let mut rep: String = s.0.to_string();
            rep.insert(1, s.1);
            line = line.replace(s.0, &rep);
        });

        let v: Vec<u64> = line
            .matches(char::is_numeric)
            .map(|s| s.parse().unwrap())
            .collect();

        let first = v.first().unwrap();
        let second = v.last().unwrap();
        sum += first * 10 + second;
    }
    println!("p2 sum: {sum}")
}

pub fn main() {
    println!("Day 1");
    part1();
    part2();
}
