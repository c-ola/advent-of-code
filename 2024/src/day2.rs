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
        let mut cur: isize = report[0];
        let mut diffs: Vec<isize> = Vec::new();
        for n in &report.as_slice()[1..] {
            let diff: isize = n - cur;
            diffs.push(diff);
            cur = *n;
        }
        let min = *diffs.iter().min().unwrap();
        let max = *diffs.iter().max().unwrap();
        let is_safe = ((min < 0 && max < 0) || (min > 0 && max > 0))
            && isize::max(max.abs(), min.abs()) <= 3
            && isize::min(min.abs(), max.abs()) >= 1;
        if is_safe {
            count_safe += 1;
        }
    }
    println!("{count_safe:?}");
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
        for i in 0..reportt.len() {
            let report: Vec<isize> = reportt
                .iter()
                .enumerate()
                .filter_map(|(j, c)| if j == i { None } else { Some(*c) })
                .collect();
            if report.len() == 0 {
                continue;
            }
            let mut cur: isize = report[0];
            let mut diffs: Vec<isize> = Vec::new();
            for n in &report.as_slice()[1..] {
                let diff: isize = n - cur;
                diffs.push(diff);
                cur = *n;
            }
            let min = *diffs.iter().min().unwrap();
            let max = *diffs.iter().max().unwrap();
            let is_safe = ((min < 0 && max < 0) || (min > 0 && max > 0))
                && isize::max(max.abs(), min.abs()) <= 3
                && isize::min(min.abs(), max.abs()) >= 1;
            if is_safe {
                count_safe += 1;
                break;
            }
        }
        /*let problems = diffs.iter()

          let mut sums = Vec::new();
          for i in 0..diffs.len() - 1 {
          sums.push(diffs[i] + diffs[i + 1]);
          }
          println!("{sums:?}");
          let min = *sums.iter().min().unwrap();
          let max = *sums.iter().max().unwrap();
          let is_safe2 = ((min < 0 && max < 0) || (min > 0 && max > 0)) && isize::max(max.abs(), min.abs()) <= 3 && isize::min(min.abs(), max.abs()) >= 1;

          if is_safe || is_safe2 {
          println!("r{report:?}");
          count_safe += 1;
          }*/
    }
    println!("{count_safe:?}");
}

fn main() {
    //part1();
    _part2();
}
