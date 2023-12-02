// AOC day 1 Trebuchet
// https://adventofcode.com/2023/day/1

use std::fs::File;
use std::io::{self, BufRead};

const NUMBERS_ALPHA: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
const NUMBERS: [char; 9] = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];

fn part1(){
    let file = File::open("src/input_day1").unwrap();
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
    let file = File::open("src/input_day1").unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut sum = 0;

    for line in lines {
        let mut line = line.unwrap();

        NUMBERS_ALPHA.iter().enumerate().for_each(|(i, s)| {
            let mut rep: String = s.to_string();
            rep.insert(1, NUMBERS[i]);
            line = line.replace(s, &rep);
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

pub fn day1() {
    println!("Day 1");
    part1();
    part2();
}
