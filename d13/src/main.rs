use std::collections::HashSet;

use text_io::scan;

/*
--- Day 13: Transparent Origami ---

You reach another volcanically active part of the cave. It would be nice if you could do some kind of thermal imaging so you could tell ahead of time which caves are too hot to safely enter.

Fortunately, the submarine seems to be equipped with a thermal camera! When you activate it, you are greeted with:

Congratulations on your purchase! To activate this infrared thermal imaging
camera system, please enter the code found on page 1 of the manual.
Apparently, the Elves have never used this feature. To your surprise, you manage to find the manual; as you go to open it, page 1 falls out. It's a large sheet of transparent paper! The transparent paper is marked with random dots and includes instructions on how to fold it up (your puzzle input). For example:

6,10
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
fold along x=5
The first section is a list of dots on the transparent paper. 0,0 represents the top-left coordinate. The first value, x, increases to the right. The second value, y, increases downward. So, the coordinate 3,0 is to the right of 0,0, and the coordinate 0,7 is below 0,0. The coordinates in this example form the following pattern, where # is a dot on the paper and . is an empty, unmarked position:

...#..#..#.
....#......
...........
#..........
...#....#.#
...........
...........
...........
...........
...........
.#....#.##.
....#......
......#...#
#..........
#.#........
Then, there is a list of fold instructions. Each instruction indicates a line on the transparent paper and wants you to fold the paper up (for horizontal y=... lines) or left (for vertical x=... lines). In this example, the first fold instruction is fold along y=7, which designates the line formed by all of the positions where y is 7 (marked here with -):

...#..#..#.
....#......
...........
#..........
...#....#.#
...........
...........
-----------
...........
...........
.#....#.##.
....#......
......#...#
#..........
#.#........
Because this is a horizontal line, fold the bottom half up. Some of the dots might end up overlapping after the fold is complete, but dots will never appear exactly on a fold line. The result of doing this fold looks like this:

#.##..#..#.
#...#......
......#...#
#...#......
.#.#..#.###
...........
...........
Now, only 17 dots are visible.

Notice, for example, the two dots in the bottom left corner before the transparent paper is folded; after the fold is complete, those dots appear in the top left corner (at 0,0 and 0,1). Because the paper is transparent, the dot just below them in the result (at 0,3) remains visible, as it can be seen through the transparent paper.

Also notice that some dots can end up overlapping; in this case, the dots merge together and become a single dot.

The second fold instruction is fold along x=5, which indicates this line:

#.##.|#..#.
#...#|.....
.....|#...#
#...#|.....
.#.#.|#.###
.....|.....
.....|.....
Because this is a vertical line, fold left:

#####
#...#
#...#
#...#
#####
.....
.....
The instructions made a square!

The transparent paper is pretty big, so for now, focus on just completing the first fold. After the first fold in the example above, 17 dots are visible - dots that end up overlapping after the fold is completed count as a single dot.

How many dots are visible after completing just the first fold instruction on your transparent paper?*/

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

/*
--- Part Two ---

Finish folding the transparent paper according to the instructions. The manual says the code is always eight capital letters.

What code do you use to activate the infrared thermal imaging camera system?
*/

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
