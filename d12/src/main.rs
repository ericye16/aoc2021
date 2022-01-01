use std::collections::{HashMap, HashSet};

type Cave<'a> = &'a str;

type CaveSystem<'a> = HashMap<Cave<'a>, Vec<Cave<'a>>>;

type CavePath<'a> = Vec<Cave<'a>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CaveType {
    Small,
    Big,
}

fn get_cave_type(cave: &Cave) -> CaveType {
    if cave.chars().all(char::is_uppercase) {
        CaveType::Big
    } else if cave.chars().all(char::is_lowercase) {
        CaveType::Small
    } else {
        panic!("Mixed case cave");
    }
}

fn parse_cave_system(input: &str) -> CaveSystem {
    let mut cave_system: CaveSystem = HashMap::new();
    for line in input.lines() {
        let line = line.trim();
        let cave_connections: Vec<&str> = line.split('-').collect();
        let first_cave = cave_connections[0];
        let second_cave = cave_connections[1];
        cave_system
            .entry(first_cave)
            .or_insert(vec![])
            .push(second_cave);
        // Bidirectional cave systems
        cave_system
            .entry(second_cave)
            .or_insert(vec![])
            .push(first_cave);
    }
    cave_system
}

fn find_paths<'a>(
    cave_system: &'a CaveSystem,
    path: CavePath<'a>,
    used_caves: HashSet<&str>,
) -> Vec<CavePath<'a>> {
    let last_cave = &path[path.len() - 1];
    if *last_cave == "end" {
        // println!("Found complete path: {:?}", path);
        return vec![path];
    }
    if !cave_system.contains_key(last_cave) {
        panic!("Cave system doesn't contain us")
    }
    // println!("Path so far: {:?}, used {:?}", path, used_caves);
    let mut v = vec![];
    for possible_next_cave in cave_system.get(last_cave).unwrap() {
        if used_caves.contains(*possible_next_cave) {
            continue;
        }
        let mut this_path = path.clone();
        this_path.push(possible_next_cave);
        let mut this_used_caves = used_caves.clone();
        if get_cave_type(possible_next_cave) == CaveType::Small {
            this_used_caves.insert(possible_next_cave);
        }
        // println!("Exploring {:?}, used {:?}", this_path, this_used_caves);
        v.extend(find_paths(cave_system, this_path, this_used_caves))
    }
    v
}

fn p1(input: &str) -> i32 {
    let cave_system = parse_cave_system(input);
    // println!("Cave system: {:?}", cave_system);

    let mut used_caves = HashSet::new();
    used_caves.insert("start");
    let path = vec!["start"];
    let paths = find_paths(&cave_system, path, used_caves);
    // println!("Paths: {:?}", paths);
    paths.len() as i32
}

fn find_paths_p2<'a>(
    cave_system: &'a CaveSystem,
    path: CavePath<'a>,
    used_caves: HashSet<&str>,
    used_2_caves: bool,
) -> Vec<CavePath<'a>> {
    let last_cave = path[path.len() - 1];
    if last_cave == "end" {
        // println!("Found complete path: {:?}", path);
        return vec![path];
    }
    if !cave_system.contains_key(last_cave) {
        panic!("Cave system doesn't contain us")
    }
    // println!(
    //     "Path so far: {:?}, used {:?}, used_2_caves: {:}",
    //     path, used_caves, used_2_caves
    // );
    let mut v = vec![];
    for possible_next_cave in cave_system.get(last_cave).unwrap() {
        if *possible_next_cave == "start" {
            continue;
        }
        if used_2_caves && used_caves.contains(*possible_next_cave) {
            continue;
        }
        let mut this_path = path.clone();
        this_path.push(possible_next_cave);
        let mut this_used_caves = used_caves.clone();
        let mut this_used_2_caves = used_2_caves;
        if get_cave_type(possible_next_cave) == CaveType::Small {
            this_used_2_caves |= !this_used_caves.insert(possible_next_cave);
        }
        // println!(
        //     "Exploring {:?}, used {:?}, used_2_caves: {:?}",
        //     this_path, this_used_caves, this_used_2_caves
        // );
        v.extend(find_paths_p2(
            cave_system,
            this_path,
            this_used_caves,
            this_used_2_caves,
        ))
    }
    v
}

fn p2(input: &str) -> i32 {
    let cave_system = parse_cave_system(input);
    // println!("Cave system: {:?}", cave_system);
    let mut used_caves = HashSet::new();
    used_caves.insert("start");
    let path = vec!["start"];
    let paths = find_paths_p2(&cave_system, path, used_caves, false);
    // println!("Paths: {:?}", paths);
    paths.len() as i32
}

fn main() {
    let input = common::read_file("d12.txt");
    println!("P1: {}", p1(input.trim()));
    println!("P2: {}", p2(input.trim()));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "start-A
    start-b
    A-c
    A-b
    b-d
    A-end
    b-end";

    const INPUT_2: &str = "dc-end
    HN-start
    start-kj
    dc-start
    dc-HN
    LN-dc
    HN-end
    kj-sa
    kj-HN
    kj-dc";

    const INPUT_3: &str = "fs-end
    he-DX
    fs-he
    start-DX
    pj-DX
    end-zg
    zg-sl
    zg-pj
    pj-he
    RW-he
    fs-DX
    pj-RW
    zg-RW
    start-pj
    he-WI
    zg-he
    pj-fs
    start-RW";

    #[test]
    fn test_p1() {
        assert_eq!(p1(INPUT), 10);
        assert_eq!(p1(INPUT_2), 19);
        assert_eq!(p1(INPUT_3), 226);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(INPUT), 36);
        assert_eq!(p2(INPUT_2), 103);
        assert_eq!(p2(INPUT_3), 3509);
    }
}
