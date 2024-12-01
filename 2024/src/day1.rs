use std::fs::read_to_string;

pub trait SplitOdd<T> {
    fn split_odd(self) -> (Vec<T>, Vec<T>);
}

impl<T: Sized, I: IntoIterator<Item = T>> SplitOdd<T> for I {
    fn split_odd(self) -> (Vec<T>, Vec<T>) {
        let mut a: Vec<T> = Vec::new();
        let mut b: Vec<T> = Vec::new();
        let mut i = 0;
        for item in self {
            if i % 2 == 0 {
                a.push(item);
            } else {
                b.push(item);
            }
            i += 1
        }
        (a, b)
    }
}

fn part1() {
    let data = read_to_string("inputs/day1.txt").unwrap();
    let data = data.split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
    let (mut left, mut right) = data.into_iter().split_odd();
    //println!("{left:?}, {right:?}");
    left.sort();
    right.sort();
    let mut sum = 0;
    for i in 0..left.len() {
        sum += left[i].abs_diff(right[i]);
    }
    println!("{sum}");
}

fn part2() {
    let data = read_to_string("inputs/day1.txt").unwrap();
    let data = data.split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
    let (left, right) = data.into_iter().split_odd();

    let mut sum = 0;
    for i in 0..left.len() {
        let count = right.iter().filter(|x| **x == left[i]).collect::<Vec<_>>().len();
        sum += left[i] * count;
    }
    println!("{sum}");
}

fn main() {
    part1();
    part2();
}
