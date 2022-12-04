use std::{io, cmp, collections::{HashMap, HashSet}, hash::Hash};

fn range_contains(r1: &Vec<u32>, r2: &Vec<u32>) -> bool {
    return r1[0] <= r2[0] && r1[1] >= r2[1];
}

fn range_overlaps(r1: &Vec<u32>, r2: &Vec<u32>) -> bool {
    return (r1[0] <= r2[0] && r2[0] <= r1[1]) ||
    (r1[0] <= r2[1] && r2[1] <= r1[1]);
}

pub fn part1() {
    let mut total: usize = 0;
    let mut line = String::new();

    loop {
        line.clear();

        let bytes = io::stdin().read_line(&mut line).expect("canno read input");
        if bytes == 0 {
            println!("{}", total);
            break;
        }

        let pairs: Vec<_> = line.trim().splitn(2, ',').collect();
        let range1: Vec<u32> = pairs[0].splitn(2, '-').map(|x| x.parse().expect("not int")).collect();
        let range2: Vec<u32> = pairs[1].splitn(2, '-').map(|x| x.parse().expect("not int")).collect();

        if range_contains(&range1, &range2) || range_contains(&range2, &range1) {
            total += 1;
        }
    }
}

pub fn part2() {
    let mut total: usize = 0;
    let mut line = String::new();

    loop {
        line.clear();

        let bytes = io::stdin().read_line(&mut line).expect("canno read input");
        if bytes == 0 {
            println!("{}", total);
            break;
        }

        let pairs: Vec<_> = line.trim().splitn(2, ',').collect();
        let range1: Vec<u32> = pairs[0].splitn(2, '-').map(|x| x.parse().expect("not int")).collect();
        let range2: Vec<u32> = pairs[1].splitn(2, '-').map(|x| x.parse().expect("not int")).collect();

        if range_overlaps(&range1, &range2) || range_overlaps(&range2, &range1) {
            total += 1;
        }
    }
}
