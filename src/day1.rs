use std::{io, cmp, collections::BinaryHeap};

pub fn part1() {
    let mut line = String::new();
    let mut sum: u64 = 0;
    let mut max: u64 = 0;

    loop {
        line.clear();

        let bytes = io::stdin().read_line(&mut line).expect("canno read input");
        if bytes == 0 {
            println!("{}", max);
            break;
        }

        let trimmed = line.trim();
        // println!("input: {}", trimmed);

        if trimmed.is_empty() {
            // println!("{sum} {max}");
            max = cmp::max(max, sum);
            sum = 0;
            continue;
        } else {
            let num: u64 = trimmed.parse().expect(format!("not an integer: {trimmed}").as_str());
            sum += num;
        }
    }
}

pub fn part2() {
    let mut line = String::new();
    let mut sum: u64 = 0;
    let mut hp: BinaryHeap<cmp::Reverse<u64>> = BinaryHeap::new();

    loop {
        line.clear();

        let bytes = io::stdin().read_line(&mut line).expect("canno read input");
        if bytes == 0 {
            sum = 0;
            for n in hp {
                sum += n.0;
            }
            println!("{sum}");
            break;
        }

        let trimmed = line.trim();
        // println!("input: {}", trimmed);

        if trimmed.is_empty() {
            // println!("{sum} {max}");
            // max = cmp::max(max, sum);
            if hp.len() >= 3 {
                let top = hp.peek();
                match top {
                    Some(n) => {
                        if n > &cmp::Reverse(sum) {
                            hp.pop();
                            hp.push(cmp::Reverse(sum));
                        }
                    },
                    None => {}
                }
            } else {
                hp.push(cmp::Reverse(sum));
            }
            sum = 0;
            continue;
        } else {
            let num: u64 = trimmed.parse().expect(format!("not an integer: {trimmed}").as_str());
            sum += num;
        }
    }
}
