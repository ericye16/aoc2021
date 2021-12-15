use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
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

    pub(crate) fn random_access(grid: &Vec<Vec<i32>>, rng: &mut StepRng, size: usize) -> i32 {
        let mut s = 0;
        let die = Uniform::from(0..size);
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

pub(crate) mod vec2d_tests {
    use std::{cmp::Reverse, collections::BinaryHeap};

    use rand::{distributions::Uniform, prelude::Distribution, rngs::mock::StepRng};

    pub(crate) fn sum_all(grid: &vec2d::Vec2D<i32>) -> i32 {
        grid.iter().sum()
    }

    pub(crate) fn random_access(grid: &vec2d::Vec2D<i32>, rng: &mut StepRng, size: usize) -> i32 {
        let mut s = 0;
        let die = Uniform::from(0..size);
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

pub fn create_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("create");
    for side_length in [100, 400].iter() {
        group.bench_with_input(
            BenchmarkId::new("Control", side_length),
            side_length,
            |b, side_length| b.iter(|| control::create_grid(*side_length)),
        );
        group.bench_with_input(
            BenchmarkId::new("Vec2D", side_length),
            side_length,
            |b, side_length| b.iter(|| Vec2D::new(*side_length, *side_length, 1i32)),
        );
    }
}

pub fn sum_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("sum");
    for side_length in [100, 400].iter() {
        let control_grid = control::create_grid(*side_length);
        group.bench_with_input(
            BenchmarkId::new("Control", side_length),
            side_length,
            |b, _| b.iter(|| control::sum_all(&control_grid)),
        );
        let vec2d_grid = vec2d::Vec2D::new(*side_length, *side_length, 1i32);
        group.bench_with_input(
            BenchmarkId::new("Vec2D", side_length),
            side_length,
            |b, _| b.iter(|| vec2d_tests::sum_all(&vec2d_grid)),
        );
    }
}

pub fn random_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("random");
    for side_length in [100, 400].iter() {
        let control_grid = control::create_grid(*side_length);
        let mut rng = StepRng::new(4, 1);
        group.bench_with_input(
            BenchmarkId::new("Control", side_length),
            side_length,
            |b, size| b.iter(|| control::random_access(&control_grid, &mut rng, *size)),
        );
        let vec2d_grid = vec2d::Vec2D::new(*side_length, *side_length, 1i32);
        let mut rng = StepRng::new(4, 1);
        group.bench_with_input(
            BenchmarkId::new("Vec2D", side_length),
            side_length,
            |b, size| b.iter(|| vec2d_tests::random_access(&vec2d_grid, &mut rng, *size)),
        );
    }
}

pub fn dijkstra_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("dijkstra");
    for side_length in [100, 400].iter() {
        let control_grid = control::create_grid(*side_length);
        group.bench_with_input(
            BenchmarkId::new("Control", side_length),
            side_length,
            |b, _| b.iter(|| control::dijkstra(&control_grid)),
        );
        let vec2d_grid = vec2d::Vec2D::new(*side_length, *side_length, 1i32);
        group.bench_with_input(
            BenchmarkId::new("Vec2D", side_length),
            side_length,
            |b, size| b.iter(|| vec2d_tests::dijkstra(&vec2d_grid, *size, *size)),
        );
    }
}

criterion_group!(
    benches,
    create_benchmark,
    sum_benchmark,
    random_benchmark,
    dijkstra_benchmark,
);
criterion_main!(benches);
