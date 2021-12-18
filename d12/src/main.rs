use std::collections::{HashMap, HashSet};

/*
--- Day 12: Passage Pathing ---

With your submarine's subterranean subsystems subsisting suboptimally, the only way you're getting out of this cave anytime soon is by finding a path yourself. Not just a path - the only way to know if you've found the best path is to find all of them.

Fortunately, the sensors are still mostly working, and so you build a rough map of the remaining caves (your puzzle input). For example:

start-A
start-b
A-c
A-b
b-d
A-end
b-end
This is a list of how all of the caves are connected. You start in the cave named start, and your destination is the cave named end. An entry like b-d means that cave b is connected to cave d - that is, you can move between them.

So, the above cave system looks roughly like this:

    start
    /   \
c--A-----b--d
    \   /
     end
Your goal is to find the number of distinct paths that start at start, end at end, and don't visit small caves more than once. There are two types of caves: big caves (written in uppercase, like A) and small caves (written in lowercase, like b). It would be a waste of time to visit any small cave more than once, but big caves are large enough that it might be worth visiting them multiple times. So, all paths you find should visit small caves at most once, and can visit big caves any number of times.

Given these rules, there are 10 paths through this example cave system:

start,A,b,A,c,A,end
start,A,b,A,end
start,A,b,end
start,A,c,A,b,A,end
start,A,c,A,b,end
start,A,c,A,end
start,A,end
start,b,A,c,A,end
start,b,A,end
start,b,end
(Each line in the above list corresponds to a single path; the caves visited by that path are listed in the order they are visited and separated by commas.)

Note that in this cave system, cave d is never visited by any path: to do so, cave b would need to be visited twice (once on the way to cave d and a second time when returning from cave d), and since cave b is small, this is not allowed.

Here is a slightly larger example:

dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc
The 19 paths through it are as follows:

start,HN,dc,HN,end
start,HN,dc,HN,kj,HN,end
start,HN,dc,end
start,HN,dc,kj,HN,end
start,HN,end
start,HN,kj,HN,dc,HN,end
start,HN,kj,HN,dc,end
start,HN,kj,HN,end
start,HN,kj,dc,HN,end
start,HN,kj,dc,end
start,dc,HN,end
start,dc,HN,kj,HN,end
start,dc,end
start,dc,kj,HN,end
start,kj,HN,dc,HN,end
start,kj,HN,dc,end
start,kj,HN,end
start,kj,dc,HN,end
start,kj,dc,end
Finally, this even larger example has 226 paths through it:

fs-end
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
start-RW
How many paths through this cave system are there that visit small caves at most once?
*/

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

/*
--- Part Two ---

After reviewing the available paths, you realize you might have time to visit a single small cave twice. Specifically, big caves can be visited any number of times, a single small cave can be visited at most twice, and the remaining small caves can be visited at most once. However, the caves named start and end can only be visited exactly once each: once you leave the start cave, you may not return to it, and once you reach the end cave, the path must end immediately.

Now, the 36 possible paths through the first example above are:

start,A,b,A,b,A,c,A,end
start,A,b,A,b,A,end
start,A,b,A,b,end
start,A,b,A,c,A,b,A,end
start,A,b,A,c,A,b,end
start,A,b,A,c,A,c,A,end
start,A,b,A,c,A,end
start,A,b,A,end
start,A,b,d,b,A,c,A,end
start,A,b,d,b,A,end
start,A,b,d,b,end
start,A,b,end
start,A,c,A,b,A,b,A,end
start,A,c,A,b,A,b,end
start,A,c,A,b,A,c,A,end
start,A,c,A,b,A,end
start,A,c,A,b,d,b,A,end
start,A,c,A,b,d,b,end
start,A,c,A,b,end
start,A,c,A,c,A,b,A,end
start,A,c,A,c,A,b,end
start,A,c,A,c,A,end
start,A,c,A,end
start,A,end
start,b,A,b,A,c,A,end
start,b,A,b,A,end
start,b,A,b,end
start,b,A,c,A,b,A,end
start,b,A,c,A,b,end
start,b,A,c,A,c,A,end
start,b,A,c,A,end
start,b,A,end
start,b,d,b,A,c,A,end
start,b,d,b,A,end
start,b,d,b,end
start,b,end
The slightly larger example above now has 103 paths through it, and the even larger example now has 3509 paths through it.

Given these new rules, how many paths through this cave system are there?
*/

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
