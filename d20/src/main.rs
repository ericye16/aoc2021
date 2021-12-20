#![feature(destructuring_assignment)]

fn parse_algorithm(line: &str) -> Vec<u8> {
    line.chars()
        .map(|c| match c {
            '#' => 1,
            '.' => 0,
            _ => panic!(),
        })
        .collect()
}

fn parse_image<'a>(lines: impl IntoIterator<Item = &'a str>) -> Vec<Vec<u8>> {
    lines
        .into_iter()
        .map(|c| {
            c.chars()
                .map(|c| match c {
                    '#' => 1,
                    '.' => 0,
                    _ => panic!(),
                })
                .collect()
        })
        .collect()
}

fn get_image_area(image: &[Vec<u8>], rc: (i32, i32), outside: u8) -> usize {
    let mut o = 0;
    for r in (rc.0 - 1)..=(rc.0 + 1) {
        for c in (rc.1 - 1)..=(rc.1 + 1) {
            o <<= 1;
            o += if r < 0
                || c < 0
                || r as usize >= image.len()
                || c as usize >= image[r as usize].len()
            {
                outside as usize
            } else {
                image[r as usize][c as usize] as usize
            };
        }
    }
    o
}

fn enhance(image: &[Vec<u8>], algorithm: &[u8], outside: u8) -> (Vec<Vec<u8>>, u8) {
    let mut v = vec![];
    for r in -1..(image.len() as i32 + 1) {
        let mut cols = vec![];
        for c in -1..(image[0].len() as i32 + 1) {
            let pixel_idx = get_image_area(image, (r, c), outside);
            let pixel = algorithm[pixel_idx];
            cols.push(pixel);
        }
        v.push(cols);
    }
    let outside_idx = get_image_area(image, (-3, -3), outside);
    let new_outside = algorithm[outside_idx];
    (v, new_outside)
}

#[allow(dead_code)]
fn print_image(image: &[Vec<u8>]) {
    for row in image {
        let to_print: String = row
            .iter()
            .map(|c| match c {
                0 => '.',
                1 => '#',
                _ => panic!(),
            })
            .collect();
        println!("{}", to_print);
    }
}

fn p1(input: &str) -> i32 {
    let mut lines = input.lines().map(str::trim);
    let algorithm = parse_algorithm(lines.next().unwrap());
    lines.next();
    let mut image = parse_image(lines);
    let mut outside = 0;
    (image, outside) = enhance(&image, &algorithm, outside);
    (image, outside) = enhance(&image, &algorithm, outside);
    assert_eq!(outside, 0);
    let mut s = 0;
    for row in &image {
        for pixel in row {
            s += *pixel as i32;
        }
    }
    s
}

fn p2(input: &str) -> i32 {
    let mut lines = input.lines().map(str::trim);
    let algorithm = parse_algorithm(lines.next().unwrap());
    lines.next();
    let mut image = parse_image(lines);
    let mut outside = 0;
    for _ in 0..50 {
        (image, outside) = enhance(&image, &algorithm, outside);
    }
    assert_eq!(outside, 0);
    let mut s = 0;
    for row in &image {
        for pixel in row {
            s += *pixel as i32;
        }
    }
    s
}

fn main() {
    let input = common::read_file("d20.txt");
    println!("P1: {}", p1(&input));
    println!("P2: {}", p2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#
    
    #..#.
    #....
    ##..#
    ..#..
    ..###";

    #[test]
    fn test_p1() {
        assert_eq!(p1(INPUT), 35);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(INPUT), 3351);
    }
}
