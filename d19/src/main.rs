use rustc_hash::FxHashSet;
use std::collections::VecDeque;

use lazy_static::lazy_static;
use ndarray::*;
use text_io::try_scan;

type Coord = Array1<i32>;
type Rotation = Array2<i32>;

lazy_static! {
    static ref ROTATIONS: Vec<Rotation> = generate_rotations();
}

fn generate_rotations() -> Vec<Array2<i32>> {
    // Four rotations about the x axis (only roll)
    let rotations: Vec<Array2<i32>> = (0..4)
        .map(|roll| {
            array![
                [1, 0, 0],
                [0, intcos(roll), -intsin(roll)],
                [0, intsin(roll), intcos(roll)],
            ]
        })
        .collect();

    let mut v = vec![];

    // +/- x
    for yaw in [0, 2] {
        let m = array![
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
        let m = array![
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
        let m = array![
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

fn maybe_parse_scanner(line: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let id: usize;
    try_scan!(line.bytes() => "--- scanner {} ---", id);
    Ok(id)
}

fn parse_coords(line: &str) -> Array1<i32> {
    let coords: Vec<&str> = line.split(',').collect();
    assert!(coords.len() >= 2);
    let x = coords[0].parse().unwrap();
    let y = coords[1].parse().unwrap();
    let z = if coords.len() == 3 {
        coords[2].parse().unwrap()
    } else {
        0
    };
    array![x, y, z]
}

fn parse_input(input: &str) -> Vec<FxHashSet<Coord>> {
    let mut v = vec![];
    let lines = input.lines().map(str::trim);
    let mut scanner_id = 0;
    for line in lines {
        if let Ok(s) = maybe_parse_scanner(line) {
            v.push(FxHashSet::default());
            scanner_id = s;
        } else if !line.is_empty() {
            let coords = parse_coords(line);
            v[scanner_id].insert(coords);
        }
    }
    v
}

fn count_matching(
    reference: &FxHashSet<Coord>,
    to_align: &FxHashSet<Coord>,
    correction: &Coord,
    rotation: &Rotation,
) -> i32 {
    let mut c = 0;
    for target in to_align {
        let rotated = rotation.dot(target);
        let corrected = rotated + correction;
        if reference.contains(&corrected) {
            c += 1;
        }
    }
    assert!(c >= 1);
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

fn try_align(
    reference: &FxHashSet<Coord>,
    to_align: &FxHashSet<Coord>,
    threshold: i32,
) -> Option<(Coord, Rotation)> {
    let mut attempts = 0;
    for target_beacon in to_align {
        for rotation in ROTATIONS.iter() {
            let rotated_target = rotation.dot(target_beacon);
            for ref_beacon in reference {
                attempts += 1;
                // we want reference = rotation.dot(target) + correction
                // so correction = reference - rotation.dot(target)
                let correction = ref_beacon - &rotated_target;
                let num_matching = count_matching(reference, to_align, &correction, rotation);
                // println!("Correction: {}, matching: {}", correction, num_matching);
                if num_matching >= threshold {
                    println!("Attempts: {}", attempts);
                    return Some((correction, rotation.clone()));
                }
            }
        }
    }
    println!("Attempts: {}", attempts);
    None
}

fn coalesce_all(input: &[FxHashSet<Coord>]) -> (FxHashSet<Coord>, Vec<Coord>) {
    let mut known = input[0].clone();
    let mut unmatched: VecDeque<FxHashSet<Coord>> = input[1..].iter().cloned().collect();
    let mut scanners = vec![array![0, 0, 0]];
    while let Some(to_match) = unmatched.pop_front() {
        if let Some((correction, rotation)) = try_align(&known, &to_match, 12) {
            // add vectors with corrections to known
            for scanned in to_match {
                let corrected = rotation.dot(&scanned) + &correction;
                known.insert(corrected);
            }
            scanners.push(correction);
            println!(
                "There are {} scanners remaining to be solved. {} beacons found so far.",
                unmatched.len(),
                known.len()
            );
        } else {
            unmatched.push_back(to_match);
        }
    }
    (known, scanners)
}

fn p1(solution: &(FxHashSet<Coord>, Vec<Coord>)) -> i32 {
    solution.0.len() as i32
}

fn manhattan_dist(a: &Coord, b: &Coord) -> i32 {
    (a[0] - b[0]).abs() + (a[1] - b[1]).abs() + (a[2] - b[2]).abs()
}

fn p2(solution: &(FxHashSet<Coord>, Vec<Coord>)) -> i32 {
    let scanners = &solution.1;
    let mut m = 0;
    for i in 0..scanners.len() {
        for j in 0..scanners.len() {
            if i == j {
                continue;
            }
            let md = manhattan_dist(&scanners[i], &scanners[j]);
            if md > m {
                m = md;
            }
        }
    }
    m
}
fn main() {
    let input = common::read_file("d19.txt");
    let input = parse_input(&input);
    let solution = coalesce_all(&input);
    println!("P1: {}", p1(&solution));
    println!("P2: {}", p2(&solution));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "--- scanner 0 ---
    404,-588,-901
    528,-643,409
    -838,591,734
    390,-675,-793
    -537,-823,-458
    -485,-357,347
    -345,-311,381
    -661,-816,-575
    -876,649,763
    -618,-824,-621
    553,345,-567
    474,580,667
    -447,-329,318
    -584,868,-557
    544,-627,-890
    564,392,-477
    455,729,728
    -892,524,684
    -689,845,-530
    423,-701,434
    7,-33,-71
    630,319,-379
    443,580,662
    -789,900,-551
    459,-707,401
    
    --- scanner 1 ---
    686,422,578
    605,423,415
    515,917,-361
    -336,658,858
    95,138,22
    -476,619,847
    -340,-569,-846
    567,-361,727
    -460,603,-452
    669,-402,600
    729,430,532
    -500,-761,534
    -322,571,750
    -466,-666,-811
    -429,-592,574
    -355,545,-477
    703,-491,-529
    -328,-685,520
    413,935,-424
    -391,539,-444
    586,-435,557
    -364,-763,-893
    807,-499,-711
    755,-354,-619
    553,889,-390
    
    --- scanner 2 ---
    649,640,665
    682,-795,504
    -784,533,-524
    -644,584,-595
    -588,-843,648
    -30,6,44
    -674,560,763
    500,723,-460
    609,671,-379
    -555,-800,653
    -675,-892,-343
    697,-426,-610
    578,704,681
    493,664,-388
    -671,-858,530
    -667,343,800
    571,-461,-707
    -138,-166,112
    -889,563,-600
    646,-828,498
    640,759,510
    -630,509,768
    -681,-892,-333
    673,-379,-804
    -742,-814,-386
    577,-820,562
    
    --- scanner 3 ---
    -589,542,597
    605,-692,669
    -500,565,-823
    -660,373,557
    -458,-679,-417
    -488,449,543
    -626,468,-788
    338,-750,-386
    528,-832,-391
    562,-778,733
    -938,-730,414
    543,643,-506
    -524,371,-870
    407,773,750
    -104,29,83
    378,-903,-323
    -778,-728,485
    426,699,580
    -438,-605,-362
    -469,-447,-387
    509,732,623
    647,635,-688
    -868,-804,481
    614,-800,639
    595,780,-596
    
    --- scanner 4 ---
    727,592,562
    -293,-554,779
    441,611,-461
    -714,465,-776
    -743,427,-804
    -660,-479,-426
    832,-632,460
    927,-485,-438
    408,393,-506
    466,436,-512
    110,16,151
    -258,-428,682
    -393,719,612
    -211,-452,876
    808,-476,-593
    -575,615,604
    -485,667,467
    -680,325,-822
    -627,-443,-432
    872,-547,-609
    833,512,582
    807,604,487
    839,-516,451
    891,-625,532
    -652,-548,-490
    30,-46,-14";

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
        assert_eq!(correction.unwrap(), (array![5, 2, 0], ArrayBase::eye(3)));
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
            .collect::<Vec<Array1<i32>>>();

        let v2 = "1,-1,1
        2,-2,2
        3,-3,3
        2,-1,3
        -5,4,-6
        -8,-7,0"
            .lines()
            .map(str::trim)
            .map(parse_coords)
            .collect::<Vec<Array1<i32>>>();

        let v3 = "-1,-1,-1
        -2,-2,-2
        -3,-3,-3
        -1,-3,-2
        4,6,5
        -7,0,8"
            .lines()
            .map(str::trim)
            .map(parse_coords)
            .collect::<Vec<Array1<i32>>>();

        let v4 = "1,1,-1
        2,2,-2
        3,3,-3
        1,3,-2
        -4,-6,5
        7,0,8"
            .lines()
            .map(str::trim)
            .map(parse_coords)
            .collect::<Vec<Array1<i32>>>();

        let v5 = "1,1,1
        2,2,2
        3,3,3
        3,1,2
        -6,-4,-5
        0,7,-8"
            .lines()
            .map(str::trim)
            .map(parse_coords)
            .collect::<Vec<Array1<i32>>>();

        for i in 0..v1.len() {
            let r = &v1[i];
            let rotated_r = ROTATIONS
                .iter()
                .map(|rm| rm.dot(r))
                .collect::<FxHashSet<Array1<i32>>>();
            println!("r: {}, rotated_r: {:?}", r, rotated_r);
            assert!(rotated_r.contains(&v1[i]));
            assert!(rotated_r.contains(&v2[i]));
            assert!(rotated_r.contains(&v3[i]));
            assert!(rotated_r.contains(&v4[i]));
            assert!(rotated_r.contains(&v5[i]));
        }
    }

    #[test]
    fn test_scanner_locate() {
        let input = parse_input(INPUT);
        let s1 = try_align(&input[0], &input[1], 12).unwrap();
        assert_eq!(s1.0, array![68, -1246, -43]);
    }

    #[test]
    fn test_p() {
        let input = parse_input(INPUT);
        let solution = coalesce_all(&input);
        assert_eq!(p1(&solution), 79);
        assert_eq!(p2(&solution), 3621);
    }
}
