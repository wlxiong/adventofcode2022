use std::{io, cmp, collections::{HashMap, HashSet, VecDeque}, hash::Hash, vec::Drain};

pub fn part1() {
    process(4);
}

pub fn part2() {
    process(14);
}

fn process(num_uniq_chars: u32) {
    let mut line = String::new();

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

        let mut char_count: HashMap<char, u32> = HashMap::new();
        let mut char_queue: VecDeque<char> = VecDeque::new();
        let mut dup_char: Option<char> = None;

        for (i, c) in trimmed.chars().enumerate() {
            // println!("{i}: {c}");

            while char_queue.len() >= num_uniq_chars as usize || (
                !char_queue.is_empty() && dup_char.is_some())
            {
                let first = *char_queue.front().unwrap();
                let elem = char_count.entry(first);
                let count = elem.or_insert(0);
                *count -= 1;
                if *count == 0 { char_count.remove(&first); }
                char_queue.pop_front();

                // println!("pop {char_queue:?}: {char_count:?}");

                if first == dup_char.unwrap() {
                    dup_char = None;
                    break;
                }
            }

            // println!("{char_queue:?}: {char_count:?}");

            let elem = char_count.entry(c);
            let count = elem.or_insert(0);
            *count += 1;
            let val = *count;
            char_queue.push_back(c);

            // println!("{char_queue:?}: {char_count:?}");

            if val > 1 {
                dup_char = Some(c);
                continue;
            }

            if char_count.len() == num_uniq_chars as usize {
                println!("{}", i+1);
                break;
            }
        }
    }
}
