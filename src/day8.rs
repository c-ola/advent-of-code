use std::collections::HashMap;

fn part1() {
    let lines: Vec<String> = include_str!("../inputs/day8")
        .lines()
        .map(|s| s.to_string())
        .collect();

    let instructions = lines[0].chars().collect::<Vec<char>>();
    let mut paths: HashMap<String, (String, String)> = HashMap::new();

    for i in 2..lines.len() {
        let line = lines[i].split(' ').collect::<Vec<&str>>();
        let root = line[0].to_string();
        let left = line[2].trim_matches(|c| c == '(' || c == ',').to_string();
        let right = line[3].trim_matches(|c| c == ')').to_string();
        paths.insert(root, (left, right));
    }
    // traverse
    let mut steps = 0;
    let mut node = "AAA".to_string();
    for i in instructions.iter().cycle() {
        let cur = node.clone();
        if cur.ends_with("Z") {
            break;
        }
        steps += 1;
        if *i == 'R' {
            node = paths.get(&cur).unwrap().1.clone();
        } else if *i == 'L' {
            node = paths.get(&cur).unwrap().0.clone();
        }
    }

    println!("Normal: {steps} steps");
}

pub fn gcd(mut n: usize, mut m: usize) -> usize {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd(a, b)
}

fn part2() {
    let lines: Vec<String> = include_str!("../inputs/day8")
        .lines()
        .map(|s| s.to_string())
        .collect();

    let instructions = lines[0].chars().collect::<Vec<char>>();
    let mut paths: HashMap<String, (String, String)> = HashMap::new();

    for i in 2..lines.len() {
        let line = lines[i].split(' ').collect::<Vec<&str>>();
        let root = line[0].to_string();
        let left = line[2].trim_matches(|c| c == '(' || c == ',').to_string();
        let right = line[3].trim_matches(|c| c == ')').to_string();
        paths.insert(root, (left, right));
    }
    // traverse
    let mut nodes: Vec<String> = paths
        .keys()
        .filter(|n| n.ends_with("A"))
        .map(|n| n.clone())
        .collect();

    // find steps for each one, then modulo to find when they line up
    let mut steps = vec![0; nodes.len()];
    for n in 0..nodes.len() {
        let mut step = 0;
        for i in instructions.iter().cycle() {
            let cur = nodes[n].clone();
            if cur.ends_with("Z") {
                break;
            }
            step += 1;
            if *i == 'R' {
                nodes[n] = paths.get(&cur).unwrap().1.clone();
            } else if *i == 'L' {
                nodes[n] = paths.get(&cur).unwrap().0.clone();
            }
        }
        steps[n] = step;
    }

    println!("{steps:?}");
    let steps: usize = lcm(&steps);
    println!("Ghost went for a short walk of {steps} steps");
}

fn main() {
    part1();
    part2();
}
