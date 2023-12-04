use std::collections::HashSet;
use std::{fs::File, io::Read};

// This might look a little complicated but it can consistently parse to the next number
fn get_num(index: usize, row: String) -> (u32, usize) {
    let mut offset = 0;
    let mut num = vec![];
    let mut c = row.as_bytes()[offset + index] as char;

    while offset + index < row.len() && !c.is_numeric() {
        offset += 1;
        if offset + index < row.len() {
            c = row.as_bytes()[offset + index] as char;
        }
    }

    let mut num_len = 0;
    while offset + index < row.len() && c.is_numeric() {
        num.push(c.to_digit(10).unwrap());
        offset += 1;
        num_len += 1;
        if offset + index < row.len() {
            c = row.as_bytes()[offset + index] as char;
        }
    }
    (
        num.iter().fold(0, |c, n| {
            num_len -= 1;
            c + n * (10 as u32).pow(num_len as u32)
        }),
        offset,
    )
}

fn part1() {
    let mut file = File::open("./inputs/day4").unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();
    let lines = contents.lines();
    let mut points = 0;

    for line in lines {
        let line = line.to_string();
        let line_bytes = line.as_bytes();
        let mut index = 0;

        let mut map: HashSet<u32> = HashSet::new();
        let mut got_winners = false;
        let mut line_winners = 0;

        while line_bytes[index] as char != ':' { index += 1 };
        while index < line.len() {
            let c = line.as_bytes()[index] as char;

            if c.is_numeric() {
                let (num, offset) = get_num(index, line.clone());
                if got_winners {
                    if map.contains(&num) {
                        line_winners += 1;
                    }
                } else {
                    map.insert(num);
                }

                index += offset;
            } else {
                index += 1;
            }
            if c == '|' {
                got_winners = true;
            }
        }

        if line_winners != 0 {
            points += 2u32.pow(line_winners as u32 - 1);
        }
    }

    println!("Points: {points}");
}

fn part2() {
    let mut file = File::open("./inputs/day4").unwrap();
    let mut contents = String::new();
    
    file.read_to_string(&mut contents).unwrap();
    let lines: Vec<String> = contents.lines().map(|x| x.to_string()).collect();
    
    let mut scratch_cards = vec![1; lines.len()];
    
    for card in 0..lines.len() {
        let line = lines[card].to_string();
        let line_bytes = line.as_bytes();
        let mut index = 0;

        let mut winners: HashSet<u32> = HashSet::new();
        let mut got_winners = false;
        let mut line_winners = 0;

        while line_bytes[index] as char != ':' { index += 1 };
        while index < line.len() {
            let c = line.as_bytes()[index] as char;

            if c.is_numeric() {
                let (num, offset) = get_num(index, line.clone());
                if got_winners {
                    if winners.contains(&num) && line_winners + card < lines.len() {
                        line_winners += 1;
                        scratch_cards[line_winners + card] += 1 * scratch_cards[card];
                    }
                } else {
                    winners.insert(num);
                }

                index += offset;
            } else {
                index += 1;
            }
            if c == '|' {
                got_winners = true;
            }
        }
        
    }

    let total_cards: u32 = scratch_cards.iter().sum();
    println!("Total cards: {total_cards}");
}

fn main() {
    part1();
    part2();
}
