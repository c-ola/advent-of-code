fn part1() {
    let lines: Vec<String> = include_str!("../inputs/day6")
        .lines()
        .map(|s| s.to_string())
        .collect();

    let line = lines[0].split(':').nth(1).unwrap();
    let times = line
        .split_whitespace()
        .filter_map(|x| x.parse::<u64>().ok())
        .collect::<Vec<_>>();

    let line = lines[1].split(':').nth(1).unwrap();
    let dists = line
        .split_whitespace()
        .filter_map(|x| x.parse::<u64>().ok())
        .collect::<Vec<_>>();

    let mut wins: Vec<u64> = Vec::new();
    for i in 0..times.len() {
        let (time, dist) = (times[i], dists[i]);
        let roots = roots(-1., time as f64, -1. * dist as f64);
        let diff = (roots.1.ceil() - roots.0.floor()) as u64 - 1;
        wins.push(diff);
    }

    let product: u64 = wins.iter().product();
    println!("Win product: {product}");
}

fn part2() {
    let lines: Vec<String> = include_str!("../inputs/day6")
        .lines()
        .map(|s| s.to_string())
        .collect();

    // yuck but not fine
    let chars = lines[0].split(':').nth(1).unwrap().chars();
    let times = chars.filter_map(|x| x.to_digit(10)).map(|x| x as u64);
    let time = times
        .rev()
        .enumerate()
        .fold(0, |c, n| c as u64 + n.1 * 10u64.pow(n.0 as u32));

    let line = lines[1].split(':').nth(1).unwrap();
    let dists = line.chars().filter_map(|x| x.to_digit(10)).map(|x| x as u64);
    let dist = dists
        .rev()
        .enumerate()
        .fold(0, |c, n| c as u64 + n.1 * 10u64.pow(n.0 as u32));

    let roots = roots(-1., time as f64, -1. * dist as f64);
    let wins = (roots.1.ceil() - roots.0.floor()) as u64 - 1;

    println!("Wins: {wins}");
}

// speed in x seconds = time0 * acceleration
// dist = speed * seconds = (race_time - time0) * (time0) * acceleration
// 0 = -t^2 + r*t - d
// a = -1, b = r, c = -d
// possible_times = x1.flooor - x0.ceil() - 1
fn roots(a: f64, b: f64, c: f64) -> (f64, f64) {
    let discriminant = b * b - 4. * a * c;
    let sqrt = discriminant.sqrt();
    ((-1. * b + sqrt) / (2. * a), (-1. * b - sqrt) / (2. * a))
}

fn main() {
    part1();
    part2();
}
