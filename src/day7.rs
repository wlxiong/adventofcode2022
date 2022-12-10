use std::{io, cmp, collections::{HashMap, HashSet}, hash::Hash, vec::Drain, path};

fn iter_path_tree(
    folder_size: &mut HashMap<String, u64>,
    subfolders: &mut HashMap<String, Vec<String>>,
    pwd: &mut Vec<String>,
    total_size: &mut u64) -> u64 {

    let mut current_folder_size = folder_size.entry(pwd.join("/")).or_default().to_owned();
    for subfolder in subfolders.entry(pwd.join("/")).or_default().to_owned() {
        pwd.push(subfolder.to_string());
        current_folder_size += iter_path_tree(folder_size, subfolders, pwd, total_size);
        pwd.pop();
    }

    let path = pwd.join("/");
    println!("path: /{path} {current_folder_size}");

    if current_folder_size < 100000 {
        *total_size += current_folder_size;
    }

    folder_size.insert(pwd.join("/"), current_folder_size);

    return current_folder_size;
}

pub fn part1() {
    let mut line = String::new();
    let mut pwd: Vec<String> = Vec::new();
    let mut folder_size: HashMap<String, u64> = HashMap::new();
    let mut subfolders: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        line.clear();

        let bytes = io::stdin().read_line(&mut line).expect("canno read input");
        if bytes == 0 {
            println!("{folder_size:#?}");
            let mut total_folder_size: u64 = 0;

            pwd.clear();
            iter_path_tree(&mut folder_size, &mut subfolders, &mut pwd, &mut total_folder_size);

            println!("{total_folder_size}");
            break;
        }

        let trimmed = line.trim_end_matches('\n');

        if trimmed.len() == 0 {
            break;
        }

        let tokens: Vec<_> = trimmed.split(" ").collect();
        if tokens[0] == "$" {
            let cmd = tokens[1];
            if cmd == "cd" {
                let path = tokens[2];
                if path == "/" {
                    pwd.clear();
                } else if path == ".." {
                    pwd.pop();
                } else {
                    pwd.push(path.to_owned());
                }
            } else if cmd == "ls" {
            } else {
                assert!(false, "unknown command: {cmd}");
            }
        } else {
            let first = tokens[0];
            let path = pwd.join("/");
            if first == "dir" {
                let entry = subfolders.entry(path).or_default();
                entry.push(tokens[1].to_owned());
            } else {
                let size: u64 = first.parse().expect("not an integer");
                let entry = folder_size.entry(path).or_default();
                *entry += size;
            }
        }
    }
}

pub fn part2() {
    let mut line = String::new();
    let mut pwd: Vec<String> = Vec::new();
    let mut folder_size: HashMap<String, u64> = HashMap::new();
    let mut subfolders: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        line.clear();

        let bytes = io::stdin().read_line(&mut line).expect("canno read input");
        if bytes == 0 {
            println!("{folder_size:#?}");
            let mut total_folder_size: u64 = 0;

            pwd.clear();
            iter_path_tree(&mut folder_size, &mut subfolders, &mut pwd, &mut total_folder_size);

            let root_size = folder_size.entry(String::new()).or_default().to_owned() as i64;
            let required_size = ((30000000 as i64) - (70000000 - root_size)) as u64;
            let mut delete_size = root_size as u64;

            for size in folder_size.values() {
                if size >= &required_size && size < &delete_size {
                    delete_size = *size;
                }
            }

            println!("{root_size} {delete_size}");
            break;
        }

        let trimmed = line.trim_end_matches('\n');

        if trimmed.len() == 0 {
            break;
        }

        let tokens: Vec<_> = trimmed.split(" ").collect();
        if tokens[0] == "$" {
            let cmd = tokens[1];
            if cmd == "cd" {
                let path = tokens[2];
                if path == "/" {
                    pwd.clear();
                } else if path == ".." {
                    pwd.pop();
                } else {
                    pwd.push(path.to_owned());
                }
            } else if cmd == "ls" {
            } else {
                assert!(false, "unknown command: {cmd}");
            }
        } else {
            let first = tokens[0];
            let path = pwd.join("/");
            if first == "dir" {
                let entry = subfolders.entry(path).or_default();
                entry.push(tokens[1].to_owned());
            } else {
                let size: u64 = first.parse().expect("not an integer");
                let entry = folder_size.entry(path).or_default();
                *entry += size;
            }
        }
    }
}
