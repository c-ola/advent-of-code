use std::fs::read_to_string;

fn part1() {
    let data = read_to_string("inputs/day2.txt").unwrap();
    let reports = data
        .split("\n")
        .map(|x| {
            x.split_whitespace()
                .map(|x| x.parse::<isize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>();

    let mut count_safe = 0;
    for report in &reports {
        if report.len() == 0 {
            continue;
        }
        match validate(report) {
            None => count_safe += 1,
            _ => (),
        }
    }
    println!("{count_safe:?}");
}


fn validate(report: &[isize]) -> Option<(Vec<isize>, isize, isize)> {
    let mut diffs: Vec<isize> = Vec::new();
    let (mut min, mut max) = (report[0] - report[1], report[0] - report[1]);
    for i in 0..report.len() - 1 {
        let diff = report[i] - report[i + 1];
        diffs.push(diff);
        min = isize::min(min, diff);
        max = isize::max(max, diff);
    }
    let min_abs = isize::min(max.abs(), min.abs());
    let max_abs = isize::max(max.abs(), min.abs());
    let is_safe = (max < 0 || min > 0) && max_abs <= 3 && min_abs >= 1;
    return if is_safe {
        None
    } else {
        Some((diffs, min, max))
    }
}

fn _part2() {
    let data = read_to_string("inputs/day2.txt").unwrap();
    let reports = data
        .split("\n")
        .map(|x| {
            x.split_whitespace()
                .map(|x| x.parse::<isize>().unwrap())
                .collect::<Vec<_>>()
        })
    .collect::<Vec<Vec<_>>>();

    let mut count_safe = 0;
    for reportt in &reports {
        if reportt.len() == 0 {
            continue;
        }
        // maybe faster??? idk
        /*match validate(reportt.to_vec()) {
            None => count_safe += 1,
            Some((diffs, min, max)) => {
                for i in 0..diffs.len() - 2 {
                    let sum = diffs[i] + diffs[i + 1];
                    let mut diffs_copy = diffs.clone();
                    diffs_copy[i] = sum;
                    diffs_copy[i + 1] = sum;
                    let min = *diffs_copy.iter().min().unwrap();
                    let max = *diffs_copy.iter().max().unwrap();
                    let max_abs = isize::max(max.abs(), min.abs());
                    let min_abs = isize::min(max.abs(), min.abs());
                    let is_safe = (max < 0 || min > 0) && max_abs <= 3 && min_abs >= 1;
                    if is_safe {
                        count_safe += 1;
                        break
                    }
                }
                let mut tmp = reportt.to_vec().clone();
                tmp.remove(0);
                match validate(tmp) {
                    None => count_safe += 1,
                    _ => {
                        let mut tmp = reportt.to_vec().clone();
                        tmp.remove(reportt.len() - 1);
                        match validate(tmp) {
                            None => count_safe += 1,
                            _ => ()
                        }
                    }
                }

            }

        };*/
        if validate(reportt).is_none() {
            count_safe += 1;
        } else {
            for i in 0..reportt.len() {
                let report: Vec<isize> = reportt
                    .iter()
                    .enumerate()
                    .filter_map(|(j, c)| if j == i { None } else { Some(*c) })
                    .collect();
                if validate(&report).is_none() {
                    count_safe += 1;
                    break;
                }
            }
        }
    }
    println!("{count_safe:?}");
}

fn main() {
    part1();
    _part2();
}
