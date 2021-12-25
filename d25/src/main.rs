fn step(grid: &mut Vec<Vec<char>>) -> bool {
    let mut moved = false;
    let mut newgrid = grid.clone();
    // rightwards first
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            let newc = (c + 1) % grid[0].len();
            if grid[r][c] == '>' && grid[r][newc] == '.' {
                moved = true;
                newgrid[r][newc] = '>';
                newgrid[r][c] = '.';
            }
        }
    }
    *grid = newgrid;
    newgrid = grid.clone();
    // downwards next
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            let newr = (r + 1) % grid.len();
            if grid[r][c] == 'v' && grid[newr][c] == '.' {
                moved = true;
                newgrid[newr][c] = 'v';
                newgrid[r][c] = '.';
            }
        }
    }
    *grid = newgrid;
    moved
}

fn p1(input: &str) -> i64 {
    let mut grid = common::read_2d(input);
    let mut count = 0;
    while step(&mut grid) {
        count += 1;
    }
    count + 1
}

fn main() {
    let input = common::read_file("d25.txt");
    println!("P1: {}", p1(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_p1() {
        let input = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";
        assert_eq!(p1(input), 58);
    }

    #[test]
    fn test_p2() {}
}
