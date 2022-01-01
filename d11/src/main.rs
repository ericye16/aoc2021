type Grid = [[u8; 10]; 10];

fn parse_grid(input: &str) -> Grid {
    let mut grid = [[0; 10]; 10];
    for (r, line) in input.lines().enumerate() {
        let line = line.trim();
        for (c, ch) in line.chars().enumerate() {
            grid[r][c] = ch.to_digit(10).unwrap() as u8;
        }
    }
    grid
}

// Returns number of flashes
fn transition(grid: &mut Grid) -> i32 {
    let mut flashes = 0;
    for r in 0..10 {
        for c in 0..10 {
            let mut to_inc = vec![(r, c)];
            while let Some((r_i, c_i)) = to_inc.pop() {
                if r_i >= 10 || c_i >= 10 {
                    continue;
                }
                grid[r_i][c_i] += 1;
                if grid[r_i][c_i] == 10 {
                    // flash
                    flashes += 1;
                    for dx in -1..=1 {
                        for dy in -1..=1 {
                            if dx == 0 && dy == 0 {
                                continue;
                            }
                            let rt = r_i as i32 + dx;
                            let ct = c_i as i32 + dy;
                            if rt < 0 || ct < 0 {
                                continue;
                            }
                            to_inc.push((rt as usize, ct as usize));
                        }
                    }
                }
            }
        }
    }
    for r in 0..10 {
        for c in 0..10 {
            if grid[r][c] > 9 {
                grid[r][c] = 0;
            }
        }
    }
    flashes
}

fn p1(input: &str) -> i32 {
    let mut grid = parse_grid(input);
    let mut flashes = 0;
    for _ in 0..100 {
        flashes += transition(&mut grid);
    }
    flashes
}

fn p2(input: &str) -> i32 {
    let mut grid = parse_grid(input);
    let mut steps = 0;
    loop {
        steps += 1;
        if transition(&mut grid) == (10 * 10) {
            return steps;
        }
    }
}

fn main() {
    let input = common::read_file("d11.txt");
    println!("P1: {}", p1(input.trim()));
    println!("P2: {}", p2(input.trim()));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "5483143223
    2745854711
    5264556173
    6141336146
    6357385478
    4167524645
    2176841721
    6882881134
    4846848554
    5283751526";

    #[test]
    fn test_p1() {
        assert_eq!(p1(INPUT), 1656);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(INPUT), 195);
    }
}
