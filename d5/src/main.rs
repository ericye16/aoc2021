use std::cmp::max;

use anyhow::Result;
use text_io::scan;

/*
--- Day 5: Hydrothermal Venture ---

You come across a field of hydrothermal vents on the ocean floor! These vents constantly produce large, opaque clouds, so it would be best to avoid them if possible.

They tend to form in lines; the submarine helpfully produces a list of nearby lines of vents (your puzzle input) for you to review. For example:

0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
Each line of vents is given as a line segment in the format x1,y1 -> x2,y2 where x1,y1 are the coordinates of one end the line segment and x2,y2 are the coordinates of the other end. These line segments include the points at both ends. In other words:

An entry like 1,1 -> 1,3 covers points 1,1, 1,2, and 1,3.
An entry like 9,7 -> 7,7 covers points 9,7, 8,7, and 7,7.
For now, only consider horizontal and vertical lines: lines where either x1 = x2 or y1 = y2.

So, the horizontal and vertical lines from the above list would produce the following diagram:

.......1..
..1....1..
..1....1..
.......1..
.112111211
..........
..........
..........
..........
222111....
In this diagram, the top left corner is 0,0 and the bottom right corner is 9,9. Each position is shown as the number of lines which cover that point or . if no line covers that point. The top-left pair of 1s, for example, comes from 2,2 -> 2,1; the very bottom row is formed by the overlapping lines 0,9 -> 5,9 and 0,9 -> 2,9.

To avoid the most dangerous areas, you need to determine the number of points where at least two lines overlap. In the above example, this is anywhere in the diagram with a 2 or larger - a total of 5 points.

Consider only horizontal and vertical lines. At how many points do at least two lines overlap?
 */

#[derive(Debug, PartialEq, Eq)]
struct Vent {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

fn parse_line(line: &str) -> Vent {
    let (x1, y1, x2, y2);
    scan!(line.bytes() => "{},{} -> {},{}", x1, y1, x2, y2);
    Vent { x1, x2, y1, y2 }
}

fn read_all_lines(input: &str) -> Vec<Vent> {
    input.lines().map(|x| x.trim()).map(parse_line).collect()
}

fn get_max_dims(vents: &[Vent]) -> (i32, i32) {
    (
        vents
            .iter()
            .fold(0, |acc, vent| max(acc, max(vent.x1, vent.x2))),
        vents
            .iter()
            .fold(0, |acc, vent| max(acc, max(vent.y1, vent.y2))),
    )
}

fn p1(input: &str) -> i32 {
    let vents = read_all_lines(input);
    let lims = get_max_dims(&vents);
    /* Create the seabed */
    let mut sea = vec![vec![]; (lims.0 + 1) as usize];
    sea.iter_mut()
        .for_each(|v| *v = vec![0; (lims.1 + 1) as usize]);
    let mut count = 0;
    for vent in vents {
        if vent.x1 == vent.x2 || vent.y1 == vent.y2 {
            let maxx = std::cmp::max(vent.x1, vent.x2);
            let maxy = std::cmp::max(vent.y1, vent.y2);
            let mut x = std::cmp::min(vent.x1, vent.x2);
            let mut y = std::cmp::min(vent.y1, vent.y2);
            while x <= maxx && y <= maxy {
                if sea[x as usize][y as usize] == 1 {
                    count += 1
                }
                sea[x as usize][y as usize] += 1;
                x += if vent.x1 != vent.x2 { 1 } else { 0 };
                y += if vent.y1 != vent.y2 { 1 } else { 0 };
            }
        }
    }
    count
}

/*
--- Part Two ---

Unfortunately, considering only horizontal and vertical lines doesn't give you the full picture; you need to also consider diagonal lines.

Because of the limits of the hydrothermal vent mapping system, the lines in your list will only ever be horizontal, vertical, or a diagonal line at exactly 45 degrees. In other words:

An entry like 1,1 -> 3,3 covers points 1,1, 2,2, and 3,3.
An entry like 9,7 -> 7,9 covers points 9,7, 8,8, and 7,9.
Considering all lines from the above example would now produce the following diagram:

1.1....11.
.111...2..
..2.1.111.
...1.2.2..
.112313211
...1.2....
..1...1...
.1.....1..
1.......1.
222111....
You still need to determine the number of points where at least two lines overlap. In the above example, this is still anywhere in the diagram with a 2 or larger - now a total of 12 points.

Consider all of the lines. At how many points do at least two lines overlap?

Your puzzle answer was 23864.
*/

fn p2(input: &str) -> i32 {
    let vents = read_all_lines(input);
    let lims = get_max_dims(&vents);
    /* Create the seabed */
    let mut sea = vec![vec![]; (lims.0 + 1) as usize];
    sea.iter_mut()
        .for_each(|v| *v = vec![0; (lims.1 + 1) as usize]);
    let mut count = 0;
    for vent in vents {
        let maxx = std::cmp::max(vent.x1, vent.x2);
        let maxy = std::cmp::max(vent.y1, vent.y2);
        let minx = std::cmp::min(vent.x1, vent.x2);
        let miny = std::cmp::min(vent.y1, vent.y2);
        let mut x = vent.x1;
        let mut y = vent.y1;
        while x <= maxx && x >= minx && y <= maxy && y >= miny {
            if sea[x as usize][y as usize] == 1 {
                count += 1
            }
            sea[x as usize][y as usize] += 1;
            x += if vent.x1 < vent.x2 {
                1
            } else if vent.x1 == vent.x2 {
                0
            } else {
                -1
            };
            y += if vent.y1 < vent.y2 {
                1
            } else if vent.y1 == vent.y2 {
                0
            } else {
                -1
            };
        }
    }
    count
}

fn main() -> Result<()> {
    let input = common::read_file("d5.txt")?;
    println!("P1: {}", p1(&input));
    println!("P2: {}", p2(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("1,2 -> 3,4"),
            Vent {
                x1: 1,
                y1: 2,
                x2: 3,
                y2: 4
            }
        );
    }

    #[test]
    fn test_p1() {
        let input = "0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2";
        assert_eq!(p1(input), 5);
    }

    #[test]
    fn test_p2() {
        let input = "0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2";
        assert_eq!(p2(input), 12);
    }
}
