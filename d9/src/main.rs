use std::collections::{BinaryHeap, HashSet};

/*
--- Day 9: Smoke Basin ---

These caves seem to be lava tubes. Parts are even still volcanically active; small hydrothermal vents release smoke into the caves that slowly settles like rain.

If you can model how the smoke flows through the caves, you might be able to avoid it and be that much safer. The submarine generates a heightmap of the floor of the nearby caves for you (your puzzle input).

Smoke flows to the lowest point of the area it's in. For example, consider the following heightmap:

2199943210
3987894921
9856789892
8767896789
9899965678
Each number corresponds to the height of a particular location, where 9 is the highest and 0 is the lowest a location can be.

Your first goal is to find the low points - the locations that are lower than any of its adjacent locations. Most locations have four adjacent locations (up, down, left, and right); locations on the edge or corner of the map have three or two adjacent locations, respectively. (Diagonal locations do not count as adjacent.)

In the above example, there are four low points, all highlighted: two are in the first row (a 1 and a 0), one is in the third row (a 5), and one is in the bottom row (also a 5). All other locations on the heightmap have some lower adjacent location, and so are not low points.

The risk level of a low point is 1 plus its height. In the above example, the risk levels of the low points are 2, 1, 6, and 6. The sum of the risk levels of all low points in the heightmap is therefore 15.

Find all of the low points on your heightmap. What is the sum of the risk levels of all low points on your heightmap?
 */

#[derive(Debug, PartialEq, Eq)]
struct HeightMap {
    map: Vec<Vec<u8>>,
    rows: usize,
    cols: usize,
}

fn read_row(line: &str) -> Vec<u8> {
    line.chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect()
}

fn read_input(input: &str) -> HeightMap {
    let map: Vec<Vec<u8>> = input.lines().map(|s| s.trim()).map(read_row).collect();
    let rows = map.len();
    let cols = map[0].len();
    HeightMap { map, rows, cols }
}

fn p1(input: &str) -> i32 {
    let heightmap = read_input(input);
    let mut risk_level = 0;
    for row in 0..heightmap.rows {
        for col in 0..heightmap.cols {
            let val = heightmap.map[row][col];
            let top_lowest = row == 0 || val < heightmap.map[row - 1][col];
            let bottom_lowest = row == heightmap.rows - 1 || val < heightmap.map[row + 1][col];
            let left_lowest = col == 0 || val < heightmap.map[row][col - 1];
            let right_lowest = col == heightmap.cols - 1 || val < heightmap.map[row][col + 1];
            let lowest = top_lowest && bottom_lowest && left_lowest && right_lowest;
            if lowest {
                risk_level += val as i32 + 1;
            }
        }
    }
    risk_level
}

/*
--- Part Two ---

Next, you need to find the largest basins so you know what areas are most important to avoid.

A basin is all locations that eventually flow downward to a single low point. Therefore, every low point has a basin, although some basins are very small. Locations of height 9 do not count as being in any basin, and all other locations will always be part of exactly one basin.

The size of a basin is the number of locations within the basin, including the low point. The example above has four basins.

The top-left basin, size 3:

2199943210
3987894921
9856789892
8767896789
9899965678
The top-right basin, size 9:

2199943210
3987894921
9856789892
8767896789
9899965678
The middle basin, size 14:

2199943210
3987894921
9856789892
8767896789
9899965678
The bottom-right basin, size 9:

2199943210
3987894921
9856789892
8767896789
9899965678
Find the three largest basins and multiply their sizes together. In the above example, this is 9 * 14 * 9 = 1134.

What do you get if you multiply together the sizes of the three largest basins?
*/

fn p2(input: &str) -> i32 {
    let heightmap = read_input(input);
    let mut basins = BinaryHeap::<i32>::new();
    for row in 0..heightmap.rows {
        for col in 0..heightmap.cols {
            let val = heightmap.map[row][col];
            let top_lowest = row == 0 || val < heightmap.map[row - 1][col];
            let bottom_lowest = row == heightmap.rows - 1 || val < heightmap.map[row + 1][col];
            let left_lowest = col == 0 || val < heightmap.map[row][col - 1];
            let right_lowest = col == heightmap.cols - 1 || val < heightmap.map[row][col + 1];
            let lowest = top_lowest && bottom_lowest && left_lowest && right_lowest;
            if lowest {
                // DFS
                let mut to_visit = vec![(row, col)];
                let mut visited = HashSet::<(usize, usize)>::new();
                let mut sink_size = 0;
                while let Some((vrow, vcol)) = to_visit.pop() {
                    let vval = heightmap.map[vrow][vcol];
                    if vval == 9 || visited.contains(&(vrow, vcol)) {
                        continue;
                    }
                    sink_size += 1;
                    visited.insert((vrow, vcol));
                    if vrow != 0 && heightmap.map[vrow - 1][vcol] >= vval {
                        to_visit.push((vrow - 1, vcol));
                    }
                    if vrow != heightmap.rows - 1 && heightmap.map[vrow + 1][vcol] >= vval {
                        to_visit.push((vrow + 1, vcol));
                    }
                    if vcol != 0 && heightmap.map[vrow][vcol - 1] >= vval {
                        to_visit.push((vrow, vcol - 1));
                    }
                    if vcol != heightmap.cols - 1 && heightmap.map[vrow][vcol + 1] >= vval {
                        to_visit.push((vrow, vcol + 1));
                    }
                }
                basins.push(sink_size);
            }
        }
    }
    basins.iter().take(3).fold(1, |acc, x| acc * x)
}

fn main() {
    let input = common::read_file("d9.txt");
    println!("P1: {}", p1(&input.trim()));
    println!("P2: {}", p2(&input.trim()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = "2199943210
        3987894921
        9856789892
        8767896789
        9899965678";
        assert_eq!(p1(input), 15);
    }

    #[test]
    fn test_p2() {
        let input = "2199943210
        3987894921
        9856789892
        8767896789
        9899965678";
        assert_eq!(p2(input), 1134);
    }
}
