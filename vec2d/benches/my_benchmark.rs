use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::rngs::mock::StepRng;
use vec2d::Vec2D;

pub(crate) mod control {
    use std::{cmp::Reverse, collections::BinaryHeap};

    use rand::{distributions::Uniform, prelude::Distribution, rngs::mock::StepRng};

    pub(crate) fn create_grid(size: usize) -> Vec<Vec<i32>> {
        vec![vec![1; size]; size]
    }

    pub(crate) fn sum_all(grid: &Vec<Vec<i32>>) -> i32 {
        grid.iter().map(|s| s.iter().sum::<i32>()).sum()
    }

    pub(crate) fn sum_all_2(grid: &Vec<Vec<i32>>) -> i32 {
        let mut s = 0;
        for w in grid {
            for g in w {
                s += *g;
            }
        }
        s
    }

    pub(crate) fn random_access(grid: &Vec<Vec<i32>>, rng: &mut StepRng) -> i32 {
        let mut s = 0;
        let die = Uniform::from(0..100);
        for _ in 0..1000 {
            s += grid[die.sample(rng)][die.sample(rng)];
        }
        s
    }

    pub(crate) fn dijkstra(grid: &Vec<Vec<i32>>) -> u32 {
        let w = grid[0].len();
        let l = grid.len();
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
                let new_cost = cost + grid[xx as usize][yy as usize] as u32;
                queue.push((Reverse(new_cost), xx, yy));
            }
        }

        panic!("Somehow didn't finish")
    }
}

pub fn control_benchmark(c: &mut Criterion) {
    c.bench_function("control create 100", |b| {
        b.iter(|| control::create_grid(black_box(100)))
    });
    let grid = control::create_grid(100);
    c.bench_function("control sum 100", |b| b.iter(|| control::sum_all(&grid)));
    c.bench_function("control sum 2 100", |b| {
        b.iter(|| control::sum_all_2(&grid))
    });
    let mut rng = StepRng::new(4, 1);
    c.bench_function("control random access 100x100 1000", |b| {
        b.iter(|| control::random_access(&grid, &mut rng))
    });
    c.bench_function("control dijkstra", |b| b.iter(|| control::dijkstra(&grid)));
}

pub(crate) mod vec2d_tests {
    use std::{cmp::Reverse, collections::BinaryHeap};

    use rand::{distributions::Uniform, prelude::Distribution, rngs::mock::StepRng};

    pub(crate) fn sum_all(grid: &vec2d::Vec2D<i32>) -> i32 {
        grid.iter().sum()
    }

    pub(crate) fn random_access(grid: &vec2d::Vec2D<i32>, rng: &mut StepRng) -> i32 {
        let mut s = 0;
        let die = Uniform::from(0..100);
        for _ in 0..1000 {
            s += grid[(die.sample(rng), die.sample(rng))];
        }
        s
    }

    pub(crate) fn dijkstra(grid: &vec2d::Vec2D<i32>, w: usize, l: usize) -> u32 {
        let mut costs = vec2d::Vec2D::new(l, w, u32::MAX);
        let mut queue = BinaryHeap::from([(Reverse(0), 0i32, 0i32)]);
        while let Some((Reverse(cost), x, y)) = queue.pop() {
            if cost >= costs[(x as usize, y as usize)] {
                continue;
            }
            if x as usize == l - 1 && y as usize == w - 1 {
                // we're here
                // println!("Costs: {:?}", costs);
                return cost;
            }
            costs[(x as usize, y as usize)] = cost;
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let xx = x + dx;
                let yy = y + dy;
                if xx < 0 || yy < 0 || xx as usize >= l || yy as usize >= w {
                    continue;
                }
                let new_cost = cost + grid[(xx as usize, yy as usize)] as u32;
                queue.push((Reverse(new_cost), xx, yy));
            }
        }

        panic!("Somehow didn't finish")
    }
}

pub fn vec2d_benchmark(c: &mut Criterion) {
    c.bench_function("vec2d create 100", |b| {
        b.iter(|| Vec2D::new(black_box(100), black_box(100), 1i32))
    });
    let grid = Vec2D::new(black_box(100), black_box(100), 1);
    c.bench_function("vec2d sum 100", |b| b.iter(|| vec2d_tests::sum_all(&grid)));
    let mut rng = StepRng::new(4, 1);
    c.bench_function("vec2d random access 100x100 1000", |b| {
        b.iter(|| vec2d_tests::random_access(&grid, &mut rng))
    });
    c.bench_function("vec2d dijkstra", |b| {
        b.iter(|| vec2d_tests::dijkstra(&grid, 100, 100))
    });
}

/*
pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}
*/

criterion_group!(benches, control_benchmark, vec2d_benchmark);
criterion_main!(benches);
