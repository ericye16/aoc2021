use std::collections::HashSet;

use text_io::scan;

fn parse_pair(input: &str) -> Option<(i32, i32)> {
    let input: Vec<&str> = input.split(',').collect();
    if input.len() != 2 {
        return None;
    }
    Some((input[0].parse().unwrap(), input[1].parse().unwrap()))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Fold {
    AlongX(i32),
    AlongY(i32),
}

fn parse_fold(input: Option<&str>) -> Option<Fold> {
    input?;
    let input = input.unwrap();
    let i: i32;
    let along: String;
    let mut words = input.split(' ');
    if words.next() != Some("fold") {
        return None;
    }
    scan!(input.bytes() => "fold along {}={}", along, i);
    if along == "x" {
        Some(Fold::AlongX(i))
    } else if along == "y" {
        Some(Fold::AlongY(i))
    } else {
        panic!("Can't parse fold")
    }
}

fn p1(input: &str) -> i32 {
    let mut pairs: HashSet<(i32, i32)> = HashSet::new();
    let mut lines = input.lines().map(str::trim);
    while let Some(pair) = parse_pair(lines.next().unwrap()) {
        pairs.insert(pair);
    }
    let fold = parse_fold(lines.next()).unwrap();
    // fold
    let mut new_pairs = HashSet::new();
    for (x, y) in pairs {
        match fold {
            Fold::AlongX(i) => {
                if x > i {
                    new_pairs.insert((i - (x - i), y));
                } else {
                    new_pairs.insert((x, y));
                }
            }
            Fold::AlongY(i) => {
                if y > i {
                    new_pairs.insert((x, i - (y - i)));
                } else {
                    new_pairs.insert((x, y));
                }
            }
        }
    }
    new_pairs.len() as i32
}

fn plot_pairs(pairs: &HashSet<(i32, i32)>) {
    let minx = pairs.iter().fold(
        i32::MAX,
        |acc, item| if item.0 < acc { item.0 } else { acc },
    );
    let miny = pairs.iter().fold(
        i32::MAX,
        |acc, item| if item.1 < acc { item.1 } else { acc },
    );
    let maxx = pairs.iter().fold(
        i32::MIN,
        |acc, item| if item.0 > acc { item.0 } else { acc },
    );
    let maxy = pairs.iter().fold(
        i32::MIN,
        |acc, item| if item.1 > acc { item.1 } else { acc },
    );
    let mut y = miny;
    while y <= maxy {
        let mut x = minx;
        let mut line = vec!['.'; (maxx - minx + 1).try_into().unwrap()];
        while x <= maxx {
            if pairs.contains(&(x, y)) {
                line[(x - minx) as usize] = '#';
            }
            x += 1;
        }
        println!("{}", line.into_iter().collect::<String>());
        y += 1;
    }
}

fn p2(input: &str) -> i32 {
    let mut pairs: HashSet<(i32, i32)> = HashSet::new();
    let mut lines = input.lines().map(str::trim);
    while let Some(pair) = parse_pair(lines.next().unwrap()) {
        pairs.insert(pair);
    }
    while let Some(fold) = parse_fold(lines.next()) {
        // fold
        let mut new_pairs = HashSet::new();
        for (x, y) in pairs {
            match fold {
                Fold::AlongX(i) => {
                    if x > i {
                        new_pairs.insert((i - (x - i), y));
                    } else {
                        new_pairs.insert((x, y));
                    }
                }
                Fold::AlongY(i) => {
                    if y > i {
                        new_pairs.insert((x, i - (y - i)));
                    } else {
                        new_pairs.insert((x, y));
                    }
                }
            }
        }
        pairs = new_pairs;
    }
    plot_pairs(&pairs);
    0
}

fn main() {
    let input = common::read_file("d13.txt");
    println!("P1: {}", p1(input.trim()));
    println!("P2: {}", p2(input.trim()));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "6,10
    0,14
    9,10
    0,3
    10,4
    4,11
    6,0
    6,12
    4,1
    0,13
    10,12
    3,4
    3,0
    8,4
    1,10
    2,14
    8,10
    9,0
    
    fold along y=7
    fold along x=5";

    #[test]
    fn test_p1() {
        assert_eq!(p1(INPUT), 17);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(INPUT), 0);
    }
}
