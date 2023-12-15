fn part1() {
    let lines: Vec<String> = include_str!("../inputs/day9")
        .lines()
        .map(|s| s.to_string())
        .collect();

    let first_histories: Vec<_> = lines
        .iter()
        .map(|s| {
            s.split(" ")
                .map(|c| c.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();
    
    let mut sum = 0;

    for h in &first_histories {
        let fh = h.clone();
        let mut end = false;
        let mut histories: Vec<Vec<i32>> = Vec::new();
        let mut history_index = 0;
        histories.push(fh.clone());
        while !end {
            let mut new_history = Vec::new();
            end = true;
            for i in 0..histories[history_index].clone().len() - 1 {
                let x = histories[history_index][i];
                let y = histories[history_index][i + 1];
                new_history.push(y - x);

                if y - x != 0 {
                    end = false;
                }
            }
            
            history_index += 1;
            histories.push(new_history);
        }

        let mut val = 0;
        while let Some(diff) = histories[history_index].last() {
            val += diff;
            if history_index == 0 {
                break;
            }
            history_index -= 1;
        }
        sum += val;
    }
    println!("Part 1: {sum}");
}

fn part2() {
    let lines: Vec<String> = include_str!("../inputs/day9")
        .lines()
        .map(|s| s.to_string())
        .collect();

    let first_histories: Vec<_> = lines
        .iter()
        .map(|s| {
            s.split(" ")
                .map(|c| c.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();
    
    let mut sum = 0;

    for h in &first_histories {
        let fh = h.clone();
        let mut end = false;
        let mut histories: Vec<Vec<i32>> = Vec::new();
        let mut history_index = 0;
        histories.push(fh.clone());
        while !end {
            let mut new_history = Vec::new();
            end = true;
            for i in 0..histories[history_index].clone().len() - 1 {
                let x = histories[history_index][i];
                let y = histories[history_index][i + 1];
                //println!("{x}, {y}");
                new_history.push(y - x);

                if y - x != 0 {
                    end = false;
                }
            }
            
            histories.push(new_history);
            history_index += 1;
        }
        let mut val = 0;
        while let Some(diff) = histories[history_index].first() {
            val = diff - val;
            //println!("{val}, {diff}");
            if history_index == 0 {
                break;
            }
            history_index -= 1;
        }
        sum += val;
    }
    println!("Part 2: {sum}");
}

fn main() {
    part1();
    part2();
}
