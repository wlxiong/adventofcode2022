use std::{io, cmp, collections::{HashMap, HashSet}, hash::Hash};

pub fn part1() {
    let lowercases = ('a'..='z').into_iter();
    let uppercases = ('A'..='Z').into_iter();
    let priority_map: HashMap<char,_> = lowercases.chain(uppercases).enumerate().map(|(i, c)| (c, i+1)).collect();
    let mut total: usize = 0;
    let mut line = String::new();

    loop {
        line.clear();

        let bytes = io::stdin().read_line(&mut line).expect("canno read input");
        if bytes == 0 {
            println!("{}", total);
            break;
        }

        // learn rust: string slice + hash set
        let first_rucksack: HashSet<char> = line[..(bytes/2)].chars().collect();
        let second_rucksack: HashSet<char> = line[(bytes/2)..].chars().collect();
        let intersection = first_rucksack.intersection(&second_rucksack);

        total += intersection.map(|c| match priority_map.get(c) {
            Some(p) => *p,
            None => 0,
        }).sum::<usize>();
    }
}

pub fn part2() {
    let lowercases = ('a'..='z').into_iter();
    let uppercases = ('A'..='Z').into_iter();
    let priority_map: HashMap<char,_> = lowercases.chain(uppercases).enumerate().map(|(i, c)| (c, i+1)).collect();
    let mut total: usize = 0;
    let mut line = String::new();
    // learn rust: initialize a vector of collections
    let mut group_rucksacks: Vec<_> = (0..3).map(|_| HashSet::<char>::new()).collect();
    let mut line_num = 0;

    loop {
        line.clear();

        let bytes = io::stdin().read_line(&mut line).expect("canno read input");
        if bytes == 0 {
            println!("{}", total);
            break;
        }

        line_num %= 3;
        // learn rust: chars()
        group_rucksacks[line_num] = line.trim().chars().collect();
        line_num += 1;

        if line_num == 3 {
            let mut intersection = HashSet::<&char>::new();

            for item in &group_rucksacks[0] {
                let mut found = 0;
                for rucksack in &group_rucksacks[1..] {
                    if !rucksack.contains(&item) {
                        break;
                    }
                    found += 1;
                }
                if found == 2 {
                    intersection.insert(item);
                }
            }

            total += intersection.into_iter().map(|c| match priority_map.get(c) {
                Some(p) => *p,
                None => 0,
            }).sum::<usize>();
        }
    }
}
