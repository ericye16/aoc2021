#![feature(test)]
extern crate test;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

type Grid = Vec<Vec<u8>>;

fn parse_input(input: &str) -> Grid {
    input
        .lines()
        .map(str::trim)
        .map(str::chars)
        .map(|c| {
            c.map(|ch| char::to_digit(ch, 10))
                .flatten()
                .map(|i| i as u8)
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>()
}

fn p1(input: &str) -> u32 {
    let input = parse_input(input);
    let w = input[0].len();
    let l = input.len();
    let mut costs = vec![vec![u32::MAX; w]; l];
    let mut queue = BinaryHeap::from([(Reverse(0), 0i32, 0i32)]);
    while let Some((Reverse(cost), x, y)) = queue.pop() {
        if cost >= costs[x as usize][y as usize] {
            continue;
        }
        if x as usize == l - 1 && y as usize == w - 1 {
            // we're here
            // println!("Costs: {:?}", costs);
            return cost;
        }
        costs[x as usize][y as usize] = cost;
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let xx = x + dx;
            let yy = y + dy;
            if xx < 0 || yy < 0 || xx as usize >= l || yy as usize >= w {
                continue;
            }
            let new_cost = cost + input[xx as usize][yy as usize] as u32;
            queue.push((Reverse(new_cost), xx, yy));
        }
    }

    panic!("Somehow didn't finish")
}

fn tile_input(input: &Grid) -> Grid {
    let l1 = input.len();
    let l2 = input[0].len();
    let mut out_grid = vec![vec![0; l2 * 5]; l1 * 5];
    for tx in 0..5 {
        for ty in 0..5 {
            for x in 0..l1 {
                for y in 0..l2 {
                    let mut new_value = input[x][y] + tx as u8 + ty as u8;
                    if new_value > 9 {
                        new_value -= 9;
                    }
                    out_grid[x + l1 * tx][y + l2 * ty] = new_value;
                }
            }
        }
    }
    out_grid
}

fn p2(input: &str) -> u32 {
    let input = parse_input(input);
    let input = tile_input(&input);

    let w = input[0].len();
    let l = input.len();
    let mut costs = vec![vec![u32::MAX; w]; l];
    let mut queue = BinaryHeap::from([(Reverse(0), 0i32, 0i32)]);
    while let Some((Reverse(cost), x, y)) = queue.pop() {
        if cost >= costs[x as usize][y as usize] {
            continue;
        }
        if x as usize == l - 1 && y as usize == w - 1 {
            // we're here
            // println!("Costs: {:?}", costs);
            return cost;
        }
        costs[x as usize][y as usize] = cost;
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let xx = x + dx;
            let yy = y + dy;
            if xx < 0 || yy < 0 || xx as usize >= l || yy as usize >= w {
                continue;
            }
            let new_cost = cost + input[xx as usize][yy as usize] as u32;
            queue.push((Reverse(new_cost), xx, yy));
        }
    }

    panic!("Somehow didn't finish")
}

fn main() {
    println!("Tiled input: {:?}", tile_input(&vec![vec![8]]));
    let input = common::read_file("d15.txt");
    println!("P1: {}", p1(&input.trim()));
    println!("P2: {}", p2(&input.trim()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    const INPUT: &str = "1163751742
    1381373672
    2136511328
    3694931569
    7463417111
    1319128137
    1359912421
    3125421639
    1293138521
    2311944581";

    #[test]
    fn test_p1() {
        assert_eq!(p1(INPUT), 40);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(INPUT), 315);
    }

    #[bench]
    fn bench_p1(b: &mut Bencher) {
        let input = common::read_file("d15.txt");
        b.iter(|| p1(&input))
    }

    #[bench]
    fn bench_p2(b: &mut Bencher) {
        let input = common::read_file("d15.txt");
        b.iter(|| p2(&input))
    }
}
