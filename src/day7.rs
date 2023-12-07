/*
 * Overall happy with this solution,
 * but the type_from_count is a little scuffed
 */

use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug)]
struct Hand {
    hand: String,
    h_type: usize,
    bid: usize,
    uses_joker: bool,
}

impl Hand {
    fn from(hand: &String, bid: usize, uses_joker: bool) -> Hand {
        let h_type = get_type(hand, uses_joker);
        Hand {
            hand: hand.to_string(),
            h_type,
            bid,
            uses_joker,
        }
    }

    fn cmp(&self, other: &Hand) -> Ordering {
        let comp = self.h_type.cmp(&other.h_type);
        if comp != Ordering::Equal {
            return comp;
        }

        let a: Vec<char> = self.hand.clone().chars().collect();
        let b: Vec<char> = other.hand.clone().chars().collect();

        for i in 0..a.len() {
            let a_o = order_label(a[i], self.uses_joker);
            let b_o = order_label(b[i], self.uses_joker);
            let comp = a_o.cmp(&b_o);
            if comp != Ordering::Equal {
                return comp;
            }
        }
        Ordering::Equal
    }
}

fn get_type(hand: &String, uses_joker: bool) -> usize {
    let mut labels: HashMap<char, usize> = HashMap::new();
    let hand = hand.as_bytes();
    let mut jokers = 0;
    for i in 0..hand.len() {
        let c = order_label(hand[i] as char, uses_joker);
        if c == '0' {
            jokers += 1;
            continue;
        }
        if !labels.contains_key(&c) {
            labels.insert(c, 1);
        } else if let Some(val) = labels.get_mut(&c) {
            *val = *val + 1;
        }
    }

    place_jokers(jokers, &mut labels);
    let mut labels_vec = labels
        .iter()
        .map(|x| (*x.1, *x.0))
        .collect::<Vec<(usize, char)>>();
    labels_vec.sort_by(|a, b| b.cmp(a));
    let t = type_from_count(&labels_vec);

    t
}

fn place_jokers(jokers: usize, labels: &mut HashMap<char, usize>) {
    let mut labels_vec = labels
        .iter()
        .map(|x| (*x.1, *x.0))
        .collect::<Vec<(usize, char)>>();
    labels_vec.sort_by(|a, b| {
        if b.0.cmp(&a.0) == Ordering::Equal {
            return b.1.cmp(&a.1);
        }
        b.0.cmp(&a.0)
    });

    if jokers == 5 {
        labels.insert('0', 5);
    } else {
        let val = labels.get_mut(&labels_vec[0].1).unwrap();
        *val += jokers;
    }
}

fn type_from_count(labels: &Vec<(usize, char)>) -> usize {
    let t = if labels[0].0 == 5 || labels[0].0 == 4 {
        labels[0].0 + 1
    } else if labels[0].0 == 3 && labels[1].0 == 2 {
        4
    } else if labels[0].0 == 3 && labels[1].0 == 1 {
        3
    } else if labels[0].0 == 1 {
        0
    } else if labels[0].0 == 2 && labels[1].0 == 1 {
        1
    } else if labels[0].0 == 2 && labels[1].0 == 2 {
        2
    } else {
        7 - labels.len()
    };
    t
}

fn order_label(c: char, uses_joker: bool) -> char {
    return match c as char {
        'T' => 'A',
        'J' => match uses_joker {
            true => '0',
            false => 'B',
        },
        'Q' => 'C',
        'K' => 'D',
        'A' => 'E',
        c => c,
    };
}

fn part1() {
    let lines: Vec<String> = include_str!("../inputs/day7")
        .lines()
        .map(|s| s.to_string())
        .collect();
    let mut hands: Vec<Hand> = lines
        .iter()
        .map(|s| {
            let split: Vec<&str> = s.split(" ").collect();
            let hand = split[0];
            let bid = split[1].parse::<usize>().unwrap();
            Hand::from(&hand.to_string(), bid, false)
        })
    .collect();
    hands.sort_by(|a, b| a.cmp(b));

    let total_winnings = hands
        .iter()
        .enumerate()
        .fold(0, |c, n| c + n.1.bid * (n.0 + 1));

    println!("{total_winnings}");
}

fn part2() {
    let lines: Vec<String> = include_str!("../inputs/day7")
        .lines()
        .map(|s| s.to_string())
        .collect();
    let mut hands: Vec<Hand> = lines
        .iter()
        .map(|s| {
            let split: Vec<&str> = s.split(" ").collect();
            let hand = split[0];
            let bid = split[1].parse::<usize>().unwrap();
            Hand::from(&hand.to_string(), bid, true)
        })
    .collect();
    hands.sort_by(|a, b| a.cmp(b));

    let total_winnings = hands
        .iter()
        .enumerate()
        .fold(0, |c, n| c + n.1.bid * (n.0 + 1));

    println!("{total_winnings}");
}

fn main() {
    part1();
    part2();
}
