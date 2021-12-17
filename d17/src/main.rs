#![feature(destructuring_assignment)]

use std::cmp::max;
use text_io::scan;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Probe {
    sx: i32,
    sy: i32,
    vx: i32,
    vy: i32,
}

impl Probe {
    fn step(&mut self) {
        self.sx += self.vx;
        self.sy += self.vy;
        if self.vx > 0 {
            self.vx -= 1;
        } else if self.vx < 0 {
            self.vx += 1;
        }
        self.vy -= 1;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Target {
    minx: i32,
    maxx: i32,
    miny: i32,
    maxy: i32,
}

fn parse_target_area(input: &str) -> Target {
    let minx: i32;
    let maxx: i32;
    let miny: i32;
    let maxy: i32;
    scan!(input.bytes() => "target area: x={}..{}, y={}..{}", minx, maxx, miny, maxy);
    Target {
        minx,
        maxx,
        miny,
        maxy,
    }
}

fn collision_x(probe: &Probe, target: &Target) -> bool {
    probe.sx >= target.minx && probe.sx <= target.maxx
}

fn collision(probe: &Probe, target: &Target) -> bool {
    collision_x(probe, target) && probe.sy >= target.miny && probe.sy <= target.maxy
}

fn would_hit(probe: &mut Probe, target: &Target) -> (bool, i32) {
    let mut highest = probe.sy;
    while !collision(probe, target) && probe.sy >= target.miny {
        probe.step();
        highest = max(highest, probe.sy)
    }
    (collision(probe, target), highest)
}

fn find_min_vx(target: &Target) -> i32 {
    // Try to hit the x first
    let mut hit_x = false;
    let mut vx = 0;
    while !hit_x {
        vx += 1;
        let mut probe = Probe {
            sx: 0,
            sy: 0,
            vx,
            vy: 0,
        };
        while !collision_x(&probe, &target) && probe.vx != 0 {
            probe.step();
        }
        hit_x = collision_x(&probe, &target);
    }
    vx
}

fn find_highest_vy(target: &Target) -> (i32, i32) {
    let mut y = target.miny;
    let mut vy = -target.miny;
    while vy != 0 {
        y += vy;
        vy -= 1;
    }
    (-target.miny - 1, y)
}

fn p1(input: &str) -> i32 {
    let target = parse_target_area(input);
    let highests = find_highest_vy(&target);
    highests.1
}

fn p2(input: &str) -> i32 {
    let target = parse_target_area(input);
    let min_vx = find_min_vx(&target);
    let max_vx = target.maxx;
    let min_vy = target.miny;
    let max_vy = -target.miny - 1;
    println!("min x {}, max x {}", min_vx, max_vx);
    println!("min y {}, max y {}", target.miny, -target.miny - 1);
    let mut count = 0;
    for vx in min_vx..=max_vx {
        for vy in min_vy..=max_vy {
            let mut probe = Probe {
                sx: 0,
                sy: 0,
                vx,
                vy,
            };
            if would_hit(&mut probe, &target).0 {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    let input = common::read_file("d17.txt");
    println!("P1: {}", p1(&input.trim()));
    println!("P2: {}", p2(&input.trim()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let target = parse_target_area("target area: x=20..30, y=-10..-5");
        println!("{:?}", target);
        assert_eq!(
            target,
            Target {
                minx: 20,
                maxx: 30,
                miny: -10,
                maxy: -5
            }
        )
    }

    #[test]
    fn test_p1() {
        assert_eq!(p1("target area: x=20..30, y=-10..-5"), 45);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2("target area: x=20..30, y=-10..-5"), 112);
    }
}
