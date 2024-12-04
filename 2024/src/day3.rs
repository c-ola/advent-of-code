use std::{fs::read_to_string, io::Cursor};

fn parse_int(i: &mut usize, data: Vec<char>) -> Option<u32> {
    let mut num: Option<u32> = None; 
    let mut j = 3;
    while j > 0 && *i < data.len() - j {
        let x = &data[*i..*i+j].iter().collect::<String>();
        let y: Result<u32, _> = x.parse::<u32>();
        num = y.map_or(None, |v| Some(v));
        if num.is_some() {
            break
        }
        j -= 1;
    }
    *i += j;
    num
}

fn part1() {
    let data = read_to_string("inputs/day3.txt").unwrap().chars().collect::<Vec<_>>();
    let mut product = 0;
    let mut i = 0;
    while i < data.len() {
        if i < data.len() - 4 && &data.as_slice()[i..i+4] == ['m', 'u', 'l', '('] {
            i += 4;
            if let Some(x) = parse_int(&mut i, data.clone()) {
                if data[i] != ',' {
                    i += 1;
                    continue
                }
                i += 1;
                if let Some(y) = parse_int(&mut i, data.clone()) {
                    if data[i] == ')' { 
                        product += y * x;
                    }
                }
            }
        } else {
            i += 1
        }
    }
    println!("{product}")
}

fn _part2() {
    let data = read_to_string("inputs/day3.txt").unwrap().chars().collect::<Vec<_>>();
    let mut product = 0;
    let mut i = 0;
    let mut enabled = true;
    while i < data.len() {
        if i < data.len() - 7 && &data[i..i+7] == ['d', 'o', 'n', '\'', 't', '(', ')'] {
            enabled = false;
            i += 7;
        }
        if i < data.len() - 4 && &data[i..i+4] == ['d', 'o', '(', ')'] {
            enabled = true;
            i += 4;
        }
        if enabled && i < data.len() - 4 && &data.as_slice()[i..i+4] == ['m', 'u', 'l', '('] {
            i += 4;
            if let Some(x) = parse_int(&mut i, data.clone()) {
                if data[i] != ',' {
                    i += 1;
                    continue
                }
                i += 1;
                if let Some(y) = parse_int(&mut i, data.clone()) {
                    if data[i] == ')' { 
                        product += y * x;
                    }
                }
            }
        } else {
            i += 1
        }
    }
    println!("{product}")

}

fn main() {
    part1();
    _part2();
}
