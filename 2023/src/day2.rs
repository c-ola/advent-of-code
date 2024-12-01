use std::fs::File;
use std::io::{self, BufRead};
use std::str::Chars;

const RED: u32 = 12;
const GREEN: u32 = 13;
const BLUE: u32 = 14;

trait MovePast {
    fn move_past(&mut self, c: char) -> bool;
    fn get_max_colors(&mut self) -> (u32, u32, u32);
    fn check_line(&mut self) -> bool;
    fn get_number(&mut self, c: char) -> u32;
}

impl MovePast for Chars<'_> {
    fn move_past(&mut self, c: char) -> bool {
        while let Some(f) = self.next() {
            if f == c {
                return true;
            }
        }
        false
    }

    fn get_number(&mut self, c: char) -> u32 {
        let mut num = c.to_digit(10).unwrap();
        let second = self.next().unwrap();
        if second.is_numeric() {
            num = num * 10 + second.to_digit(10).unwrap();
            self.next();
        }
        num
    }

    fn get_max_colors(&mut self) -> (u32, u32, u32) {
        let mut rgb = (0, 0, 0);

        while let Some(c) = self.next() {
            if c.is_numeric() {
                let num = self.get_number(c);
                match self.next() {
                    Some('r') => {
                        if num > rgb.0 {
                            rgb.0 = num
                        }
                    }
                    Some('g') => {
                        if num > rgb.1 {
                            rgb.1 = num
                        }
                    }
                    Some('b') => {
                        if num > rgb.2 {
                            rgb.2 = num
                        }
                    }
                    None | Some(_) => panic!("no color found"),
                }

                loop {
                    match self.next() {
                        Some(';') => break,
                        None | Some(',') => break,
                        Some('\n') => break,
                        _ => (),
                    }
                }
            }
        }
        rgb
    }

    fn check_line(&mut self) -> bool {
        let mut rgb = (0, 0, 0);

        while let Some(c) = self.next() {
            if c.is_numeric() {

                let num = self.get_number(c);

                match self.next() {
                    Some('r') => rgb.0 = num,
                    Some('g') => rgb.1 = num,
                    Some('b') => rgb.2 = num,
                    _ => (),
                }

                loop {
                    match self.next() {
                        Some(';') | None => {
                            if rgb.0 > RED || rgb.1 > GREEN || rgb.2 > BLUE {
                                return false;
                            }
                            rgb = (0, 0, 0);
                            break;
                        }
                        Some(',') => break,
                        _ => (),
                    }
                }
            }
        }
        true
    }
}

pub fn part1() {
    let file = File::open("inputs/day2").unwrap();
    let mut lines = io::BufReader::new(file).lines();
    let mut valid_games = 0;
    let mut game = 1;

    while let Some(line) = lines.next() {
        let line = line.unwrap();
        let mut chars = line.chars();

        chars.move_past(':');
        if chars.check_line() {
            valid_games += game;
        }
        game += 1;
    }

    println!("valid games: {valid_games}");
}

pub fn part2() {
    let file = File::open("inputs/day2").unwrap();
    let mut lines = io::BufReader::new(file).lines();
    let mut cubed = 0;

    while let Some(line) = lines.next() {
        let line = line.unwrap();
        let mut chars = line.chars();

        chars.move_past(':');
        let rgb = chars.get_max_colors();
        cubed += rgb.0 * rgb.1 * rgb.2;
    }
    println!("cubed: {cubed}");
}

pub fn main() {
    println!("Day 2");
    part1();
    part2();
}
