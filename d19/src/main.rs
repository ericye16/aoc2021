use std::collections::HashSet;

use lazy_static::lazy_static;
use text_io::try_scan;

fn p1(input: &str) -> i32 {
    todo!()
}

fn p2(input: &str) -> i32 {
    todo!()
}

type Coord = ndarray::Array1<i32>;

/*
#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
struct Coords(i32, i32, i32);

impl Display for Coords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{})", self.0, self.1, self.2)
    }
}

impl Sub for Coords {
    type Output = Coords;

    fn sub(self, rhs: Self) -> Self::Output {
        Coords(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Add for Coords {
    type Output = Coords;

    fn add(self, rhs: Self) -> Self::Output {
        Coords(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}
*/

fn maybe_parse_scanner(line: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let id: usize;
    try_scan!(line.bytes() => "--- scanner {} ---", id);
    Ok(id)
}

fn parse_coords(line: &str) -> ndarray::Array1<i32> {
    let coords: Vec<&str> = line.split(',').collect();
    assert!(coords.len() >= 2);
    let x = coords[0].parse().unwrap();
    let y = coords[1].parse().unwrap();
    let z = if coords.len() == 3 {
        coords[2].parse().unwrap()
    } else {
        0
    };
    ndarray::array![x, y, z]
}

fn parse_input(input: &str) -> Vec<HashSet<Coord>> {
    let mut v = vec![];
    let lines = input.lines().map(str::trim);
    let mut scanner_id = 0;
    for line in lines {
        if let Ok(s) = maybe_parse_scanner(line) {
            v.push(HashSet::new());
            scanner_id = s;
        } else if line.len() > 0 {
            let coords = parse_coords(line);
            v[scanner_id].insert(coords);
        }
    }
    v
}

fn count_matching(
    reference: &HashSet<Coord>,
    to_align: &HashSet<Coord>,
    correction: &Coord,
) -> i32 {
    let mut c = 0;
    for target in to_align {
        let corrected = target + correction;
        if reference.contains(&corrected) {
            c += 1;
        }
    }
    c
}

fn intsin(i: i32) -> i32 {
    match i % 4 {
        0 => 0,
        1 => 1,
        2 => 0,
        3 => -1,
        _ => unreachable!(),
    }
}

fn intcos(i: i32) -> i32 {
    match i % 4 {
        0 => 1,
        1 => 0,
        2 => -1,
        3 => 0,
        _ => unreachable!(),
    }
}

lazy_static! {
    static ref ROTATIONS: Vec<ndarray::Array2<i32>> = generate_rotations();
}

fn generate_rotations() -> Vec<ndarray::Array2<i32>> {
    // Four rotations about the x axis (only roll)
    let rotations: Vec<ndarray::Array2<i32>> = (0..4)
        .map(|roll| {
            ndarray::array![
                [1, 0, 0],
                [0, intcos(roll), -intsin(roll)],
                [0, intsin(roll), intcos(roll)],
            ]
        })
        .collect();

    let mut v = vec![];

    // +/- x
    for yaw in [0, 2] {
        let m = ndarray::array![
            [intcos(yaw), -intsin(yaw), 0],
            [intsin(yaw), intcos(yaw), 0],
            [0, 0, 1]
        ];
        for rotation in &rotations {
            v.push(m.dot(rotation));
        }
    }

    // +/- y
    for yaw in [1, 3] {
        let m = ndarray::array![
            [intcos(yaw), -intsin(yaw), 0],
            [intsin(yaw), intcos(yaw), 0],
            [0, 0, 1]
        ];
        for rotation in &rotations {
            v.push(m.dot(rotation));
        }
    }

    // +/- z
    for pitch in [1, 3] {
        let m = ndarray::array![
            [intcos(pitch), 0, intsin(pitch)],
            [0, 1, 0],
            [-intsin(pitch), 0, intcos(pitch)]
        ];
        for rotation in &rotations {
            v.push(m.dot(rotation));
        }
    }
    v
}

fn try_align(
    reference: &HashSet<Coord>,
    to_align: &HashSet<Coord>,
    threshold: i32,
) -> Option<Coord> {
    for ref_beacon in reference {
        for target_beacon in to_align {
            let correction = ref_beacon - target_beacon;
            let num_matching = count_matching(reference, to_align, &correction);
            println!("Correction: {}, matching: {}", correction, num_matching);
            if num_matching >= threshold {
                return Some(correction);
            }
        }
    }
    None
}

fn main() {
    // let input = common::read_file("d19.txt");
    // println!("P1: {}", p1(input.trim()));
    // println!("P2: {}", p2(input.trim()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2d() {
        let input = parse_input(
            "--- scanner 0 ---
        0,2
        4,1
        3,3
        
        --- scanner 1 ---
        -1,-1
        -5,0
        -2,1",
        );
        let correction = try_align(&input[0], &input[1], 3);
        assert_eq!(correction.unwrap(), ndarray::array![5, 2, 0]);
    }

    #[test]
    fn test_rotations() {
        let v1 = "-1,-1,1
        -2,-2,2
        -3,-3,3
        -2,-3,1
        5,6,-4
        8,0,7"
            .lines()
            .map(str::trim)
            .map(parse_coords)
            .collect::<Vec<ndarray::Array1<i32>>>();

        let v2 = "1,-1,1
        2,-2,2
        3,-3,3
        2,-1,3
        -5,4,-6
        -8,-7,0"
            .lines()
            .map(str::trim)
            .map(parse_coords)
            .collect::<Vec<ndarray::Array1<i32>>>();

        let v3 = "-1,-1,-1
        -2,-2,-2
        -3,-3,-3
        -1,-3,-2
        4,6,5
        -7,0,8"
            .lines()
            .map(str::trim)
            .map(parse_coords)
            .collect::<Vec<ndarray::Array1<i32>>>();

        let v4 = "1,1,-1
        2,2,-2
        3,3,-3
        1,3,-2
        -4,-6,5
        7,0,8"
            .lines()
            .map(str::trim)
            .map(parse_coords)
            .collect::<Vec<ndarray::Array1<i32>>>();

        let v5 = "1,1,1
        2,2,2
        3,3,3
        3,1,2
        -6,-4,-5
        0,7,-8"
            .lines()
            .map(str::trim)
            .map(parse_coords)
            .collect::<Vec<ndarray::Array1<i32>>>();

        for i in 0..v1.len() {
            let r = &v1[i];
            let rotated_r = ROTATIONS
                .iter()
                .map(|rm| rm.dot(r))
                .collect::<HashSet<ndarray::Array1<i32>>>();
            println!("r: {}, rotated_r: {:?}", r, rotated_r);
            assert!(rotated_r.contains(&v1[i]));
            assert!(rotated_r.contains(&v2[i]));
            assert!(rotated_r.contains(&v3[i]));
            assert!(rotated_r.contains(&v4[i]));
            assert!(rotated_r.contains(&v5[i]));
        }
    }

    #[test]
    fn test_p1() {
        assert_eq!(
            p1("-892,524,684
            -876,649,763
            -838,591,734
            -789,900,-551
            -739,-1745,668
            -706,-3180,-659
            -697,-3072,-689
            -689,845,-530
            -687,-1600,576
            -661,-816,-575
            -654,-3158,-753
            -635,-1737,486
            -631,-672,1502
            -624,-1620,1868
            -620,-3212,371
            -618,-824,-621
            -612,-1695,1788
            -601,-1648,-643
            -584,868,-557
            -537,-823,-458
            -532,-1715,1894
            -518,-1681,-600
            -499,-1607,-770
            -485,-357,347
            -470,-3283,303
            -456,-621,1527
            -447,-329,318
            -430,-3130,366
            -413,-627,1469
            -345,-311,381
            -36,-1284,1171
            -27,-1108,-65
            7,-33,-71
            12,-2351,-103
            26,-1119,1091
            346,-2985,342
            366,-3059,397
            377,-2827,367
            390,-675,-793
            396,-1931,-563
            404,-588,-901
            408,-1815,803
            423,-701,434
            432,-2009,850
            443,580,662
            455,729,728
            456,-540,1869
            459,-707,401
            465,-695,1988
            474,580,667
            496,-1584,1900
            497,-1838,-617
            527,-524,1933
            528,-643,409
            534,-1912,768
            544,-627,-890
            553,345,-567
            564,392,-477
            568,-2007,-577
            605,-1665,1952
            612,-1593,1893
            630,319,-379
            686,-3108,-505
            776,-3184,-501
            846,-3110,-434
            1135,-1161,1235
            1243,-1093,1063
            1660,-552,429
            1693,-557,386
            1735,-437,1738
            1749,-1800,1813
            1772,-405,1572
            1776,-675,371
            1779,-442,1789
            1780,-1548,337
            1786,-1538,337
            1847,-1591,415
            1889,-1729,1762
            1994,-1805,1792"),
            4140
        );
    }

    #[test]
    fn test_p2() {
        assert_eq!(
            p2("[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
        [[[5,[2,8]],4],[5,[[9,9],0]]]
        [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
        [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
        [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
        [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
        [[[[5,4],[7,7]],8],[[8,3],8]]
        [[9,3],[[9,9],[6,[4,9]]]]
        [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
        [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"),
            3993
        );
    }
}
