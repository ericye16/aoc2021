use std::cmp::max;

use anyhow::Result;
use text_io::scan;

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
    let input = common::read_file("d5.txt");
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
