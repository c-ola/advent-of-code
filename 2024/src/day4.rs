use std::{collections::HashSet, fs::read_to_string};

fn check_xmas(col: i32, row: i32, data: Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    let vectors = vec![(1, 0), (0, 1), (1, 1)];
    let combos = vec![(1, 1), (-1, 1), (-1, -1), (1, -1)];
    let mut dirs_checked: HashSet<(i32, i32)> = HashSet::new();
    for dir in &vectors {
        for combo in &combos {
            let vx = dir.0 * combo.0;
            let vy = dir.1 * combo.1;
            let full_dir = (vx, vy);
            if dirs_checked.contains(&full_dir)
                || col + (vx * 3) < 0
                || col + vx * 3 >= data.len() as i32
                || row + vy * 3 < 0
                || row + vy * 3 >= data.len() as i32
            {
                continue;
            }
            dirs_checked.insert((vx, vy));
            let s = (0..3)
                .map(|i| {
                    let x = col as i32 + (i + 1) * vx;
                    let y = row as i32 + (i + 1) * vy;
                    data[y as usize][x as usize]
                })
                .collect::<String>();
            if s == "MAS" {
                count += 1
            }
        }
    }
    count
}

fn check_x_mas(col: i32, row: i32, data: Vec<Vec<char>>) -> bool {
    if col - 1 < 0 || col + 1 >= data.len() as i32 || row - 1 < 0 || row + 1 >= data.len() as i32 {
        return false;
    }

    let pattern = vec![(0, 0), (-1, -1), (1, 1), (-1, 1), (1, -1)];

    let diag = pattern
        .iter()
        .map(|loc| {
            let x = col - loc.0;
            let y = row - loc.1;
            data[y as usize][x as usize]
        })
        .collect::<Vec<_>>();
    if diag[0] == 'A'
        && diag[1] != diag[2]
        && diag[3] != diag[4]
        && !diag.contains(&'X')
        && !diag[1..].contains(&'A')
    {
        true
    } else {
        false
    }
}

fn part1() {
    let data = read_to_string("inputs/day4.txt").unwrap();
    let data = data
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut count = 0;
    for row in 0..data.len() {
        for col in 0..data.len() {
            if data[row][col] == 'X' {
                count += check_xmas(col as i32, row as i32, data.clone());
            }
        }
    }
    println!("{count}");
}

fn part2() {
    let data = read_to_string("inputs/day4.txt").unwrap();
    let data = data
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut count = 0;
    for row in 0..data.len() {
        for col in 0..data.len() {
            count += check_x_mas(col as i32, row as i32, data.clone()) as usize;
        }
    }
    println!("{count}");
}

fn main() {
    part1();
    part2();
}
