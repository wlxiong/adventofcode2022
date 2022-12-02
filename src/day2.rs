use std::{io, cmp, collections::HashMap};

pub fn part1() {
    let mut line = String::new();
    let mut total: u64 = 0;
    let rules = HashMap::from([
        (("A", "X"), (1, 3)),
        (("A", "Y"), (2, 6)),
        (("A", "Z"), (3, 0)),
        (("B", "X"), (1, 0)),
        (("B", "Y"), (2, 3)),
        (("B", "Z"), (3, 6)),
        (("C", "X"), (1, 6)),
        (("C", "Y"), (2, 0)),
        (("C", "Z"), (3, 3)),
    ]);

    loop {
        line.clear();

        let bytes = io::stdin().read_line(&mut line).expect("canno read input");
        if bytes == 0 {
            println!("{}", total);
            break;
        }

        let trimmed = line.trim();
        let actions: Vec<_> = trimmed.splitn(2, " ").collect();
        let score = match rules.get(&(actions[0], actions[1])) {
            Some(out) => out.0 + out.1,
            None => 0
        };
        total += score;
    }
}

pub fn part2() {
    let mut line = String::new();
    let mut total: u64 = 0;
    let rules = HashMap::from([
        (("A", "X"), (3, 0)),
        (("A", "Y"), (1, 3)),
        (("A", "Z"), (2, 6)),
        (("B", "X"), (1, 0)),
        (("B", "Y"), (2, 3)),
        (("B", "Z"), (3, 6)),
        (("C", "X"), (2, 0)),
        (("C", "Y"), (3, 3)),
        (("C", "Z"), (1, 6)),
    ]);

    loop {
        line.clear();

        let bytes = io::stdin().read_line(&mut line).expect("canno read input");
        if bytes == 0 {
            println!("{}", total);
            break;
        }

        let trimmed = line.trim();
        let actions: Vec<_> = trimmed.splitn(2, " ").collect();
        let score = match rules.get(&(actions[0], actions[1])) {
            Some(out) => out.0 + out.1,
            None => 0
        };
        total += score;
    }
}
