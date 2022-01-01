use std::collections::{BinaryHeap, HashSet};

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
    basins.iter().take(3).product()
}

fn main() {
    let input = common::read_file("d9.txt");
    println!("P1: {}", p1(input.trim()));
    println!("P2: {}", p2(input.trim()));
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
