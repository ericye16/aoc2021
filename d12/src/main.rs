use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Cave<'a> {
    Start,
    End,
    Small(&'a str),
    Big(&'a str),
}
type CaveSystem<'a> = HashMap<Cave<'a>, Vec<Cave<'a>>>;

fn to_cave(cave_name: &str) -> Cave {
    match cave_name {
        "start" => Cave::Start,
        "end" => Cave::End,
        cave_name if cave_name.chars().all(char::is_lowercase) => Cave::Small(cave_name),
        _ => Cave::Big(cave_name),
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
            .entry(to_cave(first_cave))
            .or_insert(vec![])
            .push(to_cave(second_cave));
        // Bidirectional cave systems
        cave_system
            .entry(to_cave(second_cave))
            .or_insert(vec![])
            .push(to_cave(first_cave));
    }
    cave_system
}

fn find_paths<'a>(
    cave_system: &'a CaveSystem,
    last_cave: &Cave,
    used_caves: HashSet<&Cave>,
) -> i32 {
    if let Cave::End = last_cave {
        1
    } else {
        debug_assert!(cave_system.contains_key(last_cave));
        cave_system
            .get(last_cave)
            .unwrap()
            .iter()
            .filter(|possible_next_cave| !used_caves.contains(possible_next_cave))
            .map(|possible_next_cave| {
                let mut this_used_caves = used_caves.clone();
                if let Cave::Small(_) = possible_next_cave {
                    this_used_caves.insert(possible_next_cave);
                }
                find_paths(cave_system, possible_next_cave, this_used_caves)
            })
            .sum()
    }
}

fn p1(input: &str) -> i32 {
    let cave_system = parse_cave_system(input);
    let mut used_caves = HashSet::new();
    used_caves.insert(&Cave::Start);
    find_paths(&cave_system, &Cave::Start, used_caves)
}

fn find_paths_p2<'a>(
    cave_system: &'a CaveSystem,
    last_cave: &Cave,
    used_caves: HashSet<&Cave>,
    used_2_caves: bool,
) -> i32 {
    if let Cave::End = last_cave {
        1
    } else {
        debug_assert!(cave_system.contains_key(last_cave));
        cave_system
            .get(last_cave)
            .unwrap()
            .iter()
            .filter(|&possible_next_cave| {
                *possible_next_cave != Cave::Start
                    && !(used_2_caves && used_caves.contains(possible_next_cave))
            })
            .map(|possible_next_cave| {
                let mut this_used_caves = used_caves.clone();
                let mut this_used_2_caves = used_2_caves;
                if let Cave::Small(_) = possible_next_cave {
                    this_used_2_caves |= !this_used_caves.insert(possible_next_cave);
                }
                find_paths_p2(
                    cave_system,
                    possible_next_cave,
                    this_used_caves,
                    this_used_2_caves,
                )
            })
            .sum()
    }
}

fn p2(input: &str) -> i32 {
    let cave_system = parse_cave_system(input);
    let mut used_caves = HashSet::new();
    used_caves.insert(&Cave::Start);
    find_paths_p2(&cave_system, &Cave::Start, used_caves, false)
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
