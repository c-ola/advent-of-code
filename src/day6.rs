fn part1() {
    let lines: Vec<String> = include_str!("../inputs/day6")
        .lines()
        .map(|s| s.to_string())
        .collect();

    let times = lines[0]
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split_whitespace()
        .filter_map(|x| x.parse::<u64>().ok())
        .collect::<Vec<_>>();

    let distances = lines[1]
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split_whitespace()
        .filter_map(|x| x.parse::<u64>().ok())
        .collect::<Vec<_>>();

    println!("times: {times:?}");
    println!("distances: {distances:?}");
    let mut wins: Vec<u64> = Vec::new();
    for i in 0..times.len() {
        let (time, record_dist) = (times[i], distances[i]);
        let mut ways_to_win = 0;
        for hold_time in 0..time {
            let speed = hold_time;
            if speed * (time - hold_time) > record_dist {
                ways_to_win += 1;
            }
        }
        wins.push(ways_to_win);
    }
    let product: u64 = wins.iter().product();
    println!("Win product: {product}");
}

fn part2() {
    let lines: Vec<String> = include_str!("../inputs/day6")
        .lines()
        .map(|s| s.to_string())
        .collect();

    let times = lines[0]
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .chars()
        .filter_map(|x| x.to_digit(10))
        .map(|x| x as u64)
        .rev();

    let time = times.enumerate().fold(0, |c, n| {
        c as u64 + n.1 * 10u64.pow(n.0 as u32)
    });

    let distances = lines[1]
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .chars()
        .filter_map(|x| x.to_digit(10))
        .map(|x| x as u64)
        .rev();

    let dist = distances.enumerate().fold(0, |c, n| {
        c as u64 + n.1 * 10u64.pow(n.0 as u32)
    });

    println!("Time: {time}");
    println!("Dist: {dist}");
    
    // speed in x seconds = time0 * acceleration
    // dist in seconds with speed = speed * seconds = (race_time - time0) * (time0) * acceleration
    // 0 = -t^2 + r*t - d
    // a = -1
    // b = r
    // c = -d
    // winning_times = x1 - x0
    let roots = roots(-1., time as f64, -1. * dist as f64);
    let wins = (roots.1 - roots.0) as u64;

    println!("Win product: {wins}");
}


fn roots(a: f64, b: f64, c: f64) -> (f64, f64) {
    let discriminant = b * b - 4. * a * c;
    let sqrt = discriminant.sqrt();
    (
        (-1. * b + sqrt) / (2. * a),
        (-1. * b - sqrt) / (2. * a),
    )
}

fn main() {
    part1();
    part2();
}
