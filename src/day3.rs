//started 11:18
use std::fs::File;
use std::io::{self, BufRead};
use std::usize;

fn get_num(index: usize, row: &Vec<char>) -> (u32, usize) {
    let mut i = index;
    let mut num = vec![];
    while i < row.len() && row[i].is_numeric() {
        num.push(row[i].to_digit(10).unwrap());
        i += 1;
    }
    i = num.len();
    (
        num.iter().fold(0, |c, n| {
            i -= 1;
            c + n * (10 as u32).pow(i as u32)
        }),
        num.len(),
    )
}

//returns number and start offset from the end
fn get_num_v2(index: usize, row: &Vec<char>) -> (u32, usize) {
    let (mut l, mut r) = (index as isize - 1, index as isize + 1);
    let mut num = vec![row[index].to_digit(10).unwrap()];
    //go left
    while l >= 0 {
        match row[l as usize].to_digit(10) {
            Some(n) => num.insert(0, n),
            None => break,
        }
        l -= 1;
    }
    while r < row.len() as isize {
        match row[r as usize].to_digit(10) {
            Some(n) => num.push(n),
            None => break,
        }
        r += 1;
    }
    l += 1;
    let mut i = num.len();
    (
        num.iter().fold(0, |c, n| {
            i -= 1;
            c + n * (10 as u32).pow(i as u32)
        }),
        r as usize - index - 1,
    )
}

fn check_gear(i: isize, j: isize, board: &Vec<Vec<char>>) -> Option<(u32, usize)> {
    if (i < board.len() as isize && i >= 0) && (j < board[i as usize].len() as isize && j >= 0) {
        let cur_row = &board.get(i as usize).unwrap();
        let start_c = cur_row[j as usize];
        if start_c.is_numeric() {
            return Some(get_num_v2(j as usize, cur_row));
        }
    }
    None
}

fn check_adj(row: isize, col: isize, board: &Vec<Vec<char>>) -> bool {
    for i in row - 1..row + 2 {
        for j in col - 1..col + 2 {
            let (ri, cj) = (i, j);
            if (ri < board.len() as isize && ri >= 0)
                && (cj < board[ri as usize].len() as isize && cj >= 0)
            {
                let c = board[ri as usize][cj as usize];
                if !c.is_numeric() && c != '.' {
                    return true;
                };
            }
        }
    }
    false
}

fn part1() {
    let file = File::open("./inputs/day3").unwrap();
    let lines = io::BufReader::new(file).lines();
    let board: Vec<Vec<char>> = lines.map(|x| x.unwrap().chars().collect()).collect();

    let mut sum = 0;

    for rowi in 0..board.len() {
        let row = board.get(rowi).unwrap();
        let mut coli = 0;

        while coli < row.len() {
            let c = row[coli];
            if c.is_numeric() {
                let (num, offset) = get_num(coli, row);
                for i in 0..offset {
                    if check_adj(rowi as isize, (coli + i) as isize, &board) {
                        sum += num;
                        break;
                    }
                }
                coli += offset;
            }
            coli += 1;
        }
    }
    println!("p1: {sum}");
}

fn part2() {
    //let file = File::open("./inputs/day3").unwrap();
    let file = File::open("./inputs/day3").unwrap();
    let lines = io::BufReader::new(file).lines();
    let board: Vec<Vec<char>> = lines.map(|x| x.unwrap().chars().collect()).collect();

    let mut sum = 0;

    for rowi in 0..board.len() {
        let row = board.get(rowi).unwrap();
        let mut coli = 0;

        while coli < row.len() {
            let c = row[coli];
            if c == '*' {
                let (ri, ci) = (rowi as isize, coli as isize);
                let mut count = 0;
                let mut gear_values = [0, 0];

                for i in ri - 1..ri + 2 {
                    let mut j = ci - 1;
                    while j < ci + 2 {
                        //check bounds
                        if let Some(gear) = check_gear(i, j, &board) {
                            gear_values[count] = gear.0;
                            count += 1;
                            j += gear.1 as isize;
                        } 
                        j += 1;
                    }
                }
                if count == 2 {
                    sum += gear_values[0] * gear_values[1];
                }
            }
            coli += 1;
        }
    }
    println!("p2: {sum}");
}

fn main() {
    part1();
    part2();
}
