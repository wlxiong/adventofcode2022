use std::{io, cmp, collections::{HashMap, HashSet}, hash::Hash, vec::Drain};

pub fn part1() {
    let mut line = String::new();
    let mut stacks : Vec<Vec<char>> = Vec::new();

    loop {
        line.clear();

        let bytes = io::stdin().read_line(&mut line).expect("canno read input");
        if bytes == 0 {
            break;
        }

        let trimmed = line.trim_end_matches('\n');

        if trimmed.len() == 0 {
            break;
        }

        for (i, c) in trimmed.chars().enumerate() {
            if i % 4 == 1 && c != ' ' {
                let col = i / 4;
                // println!("({col}, {c})");
                if col >= stacks.len() {
                    stacks.resize(col+1, Vec::new());
                }
                stacks[col].push(c);
                // println!("{:?}", stacks[col]);
            }
        }
    }

    for s in &mut stacks {
        s.pop();
        s.reverse();
        println!("{s:?}");
    }

    loop {
        line.clear();

        let bytes = io::stdin().read_line(&mut line).expect("canno read input");
        if bytes == 0 {
            break;
        }

        let trimmed = line.trim_end_matches('\n');

        if trimmed.len() == 0 {
            break;
        }

        // move 2 from 8 to 4
        let fields: Vec<usize> = trimmed.splitn(6, ' ').map(|c| c.parse().unwrap_or(0)).collect();
        let (num, src, dst) = (fields[1], fields[3]-1, fields[5]-1);

        for _ in 0..num {
            let x = stacks[src].pop();
            if let Some(m) = x {
                stacks[dst].push(m);
            }
        }
    }

    let mut result = String::new();

    for s in &mut stacks {
        if let Some(m) = s.last() {
            result.push(*m);
        }
    }

    println!("result: {result}");

}

pub fn part2() {
    let mut line = String::new();
    let mut stacks : Vec<Vec<char>> = Vec::new();

    loop {
        line.clear();

        let bytes = io::stdin().read_line(&mut line).expect("canno read input");
        if bytes == 0 {
            break;
        }

        let trimmed = line.trim_end_matches('\n');

        if trimmed.len() == 0 {
            break;
        }

        for (i, c) in trimmed.chars().enumerate() {
            if i % 4 == 1 && c != ' ' {
                let col = i / 4;
                // println!("({col}, {c})");
                if col >= stacks.len() {
                    stacks.resize(col+1, Vec::new());
                }
                stacks[col].push(c);
                // println!("{:?}", stacks[col]);
            }
        }
    }

    for s in &mut stacks {
        s.pop();
        s.reverse();
        println!("{s:?}");
    }

    loop {
        line.clear();

        let bytes = io::stdin().read_line(&mut line).expect("canno read input");
        if bytes == 0 {
            break;
        }

        let trimmed = line.trim_end_matches('\n');

        if trimmed.len() == 0 {
            break;
        }

        // move 2 from 8 to 4
        let fields: Vec<usize> = trimmed.splitn(6, ' ').map(|c| c.parse().unwrap_or(0)).collect();
        let (num, src, dst) = (fields[1], fields[3]-1, fields[5]-1);

        let stack_height = stacks[src].len();
        let tail_elems: Vec<_> = stacks[src].drain((stack_height - num)..).collect();
        stacks[dst].extend(tail_elems);
    }

    let mut result = String::new();

    for s in &mut stacks {
        if let Some(m) = s.last() {
            result.push(*m);
        }
    }

    println!("result: {result}");

}
