extern crate blas_src;
extern crate clap;
extern crate ndarray_linalg;

use clap::{App, Arg};

use std::collections::{HashMap, HashSet};

use ndarray::*;
use ndarray_linalg::solve::Inverse;
use ndarray_linalg::LeastSquaresSvd;

use text_io::try_scan;

type Coord = Array1<i32>;
type Transform = Array2<i32>;

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

fn parse_input(input: &str) -> Vec<Vec<Coord>> {
    let mut v = vec![];
    let lines = input.lines().map(str::trim);
    let mut scanner_id = 0;
    for line in lines {
        if let Ok(s) = maybe_parse_scanner(line) {
            v.push(vec![]);
            scanner_id = s;
        } else if !line.is_empty() {
            let coords = parse_coords(line);
            v[scanner_id].push(coords);
        }
    }
    v
}

fn find_pairwise_distances(beacons: &Vec<Coord>) -> Vec<HashSet<i32>> {
    let mut sq_distances = vec![];
    for c1 in beacons {
        let mut distances_this_scanner = HashSet::new();
        for c2 in beacons {
            if c1 == c2 {
                continue;
            }
            distances_this_scanner.insert((c1 - c2).mapv(|x| x.pow(2)).sum());
        }
        sq_distances.push(distances_this_scanner);
    }
    sq_distances
}

fn as_float(a: Array2<i32>) -> Array2<f32> {
    a.mapv(|i| i as f32)
}

fn as_int(a: Array2<f32>) -> Array2<i32> {
    a.mapv(|f| f.round() as i32)
}

// Find matrix A s.t. A * (any vector from b) = (any beacon from a)
fn solve_vecs(mut a: Array2<i32>, mut b: Array2<i32>) -> Transform {
    a.push_row(Array::from_elem(a.shape()[1], 1).view())
        .unwrap();
    b.push_row(Array::from_elem(b.shape()[1], 1).view())
        .unwrap();
    // BLAS routines only work on floats, so convert to float and then convert
    // back
    as_int(
        as_float(b)
            .t()
            .least_squares(&as_float(a).t())
            .unwrap()
            .solution
            .t()
            .to_owned(),
    )
}

fn solve(input: &Vec<Vec<Coord>>) -> (HashSet<Coord>, Vec<Coord>) {
    let sq_distances_all: Vec<Vec<HashSet<i32>>> =
        input.iter().map(find_pairwise_distances).collect();
    // transforms[i][j] describes the transform going from j -> i
    let mut transforms = HashMap::<usize, HashMap<usize, Transform>>::new();
    let mut all_coords = HashSet::new();
    let mut scanners = vec![];
    for (i, beacon_set) in sq_distances_all.iter().enumerate() {
        for j in (i + 1)..sq_distances_all.len() {
            let beacon_set2 = &sq_distances_all[j];
            let mut matches = vec![];
            for (bi, beacon_a) in beacon_set.iter().enumerate() {
                for (bj, beacon_b) in beacon_set2.iter().enumerate() {
                    if beacon_a.intersection(beacon_b).count() >= 11 {
                        matches.push((bi, bj));
                    }
                }
            }
            if matches.len() < 12 {
                continue;
            }
            let mut a_vecs = input[i][matches[0].0].clone().insert_axis(Axis(1));
            let mut b_vecs = input[j][matches[0].1].clone().insert_axis(Axis(1));
            for (bidx, bjdx) in &matches[1..] {
                a_vecs.push_column(input[i][*bidx].view()).unwrap();
                b_vecs.push_column(input[j][*bjdx].view()).unwrap();
            }
            let affine_matrix = solve_vecs(a_vecs, b_vecs);
            // affine_matrix * b_vecs = a_vecs
            // println!("Affine matrix from {} to {}?\n{}", j, i, affine_matrix);
            transforms
                .entry(i)
                .or_insert_with(HashMap::new)
                .insert(j, affine_matrix.clone());
            transforms
                .entry(j)
                .or_insert_with(HashMap::new)
                .insert(i, as_int(as_float(affine_matrix).inv().unwrap()));
        }
    }

    // Find all transforms to 0
    transforms
        .get_mut(&0)
        .unwrap()
        .insert(0, ndarray::Array2::eye(4));
    let mut transforms_to_zero = transforms.get(&0).unwrap().clone();
    while transforms_to_zero.len() < input.len() {
        for (i, t2) in &transforms {
            for (j, transform) in t2 {
                if transforms_to_zero.contains_key(j) {
                    continue;
                }
                if let Some(a_matrix) = transforms_to_zero.get(i).cloned() {
                    transforms_to_zero.insert(*j, a_matrix.dot(transform));
                }
            }
        }
        *transforms.get_mut(&0).unwrap() = transforms_to_zero.clone();
    }
    // println!("Transforms to 0: {:?}", transforms_to_zero);
    for (scanner_idx, scanner_set) in input.iter().enumerate() {
        let transform = transforms_to_zero.get(&scanner_idx).unwrap();
        for beacon in scanner_set {
            all_coords.insert(
                transform.dot(&concatenate(Axis(0), &[beacon.view(), array![1].view()]).unwrap()),
            );
        }
        scanners.push(transform.slice(s![..3_usize, 3_usize]).to_owned());
    }
    (all_coords, scanners)
}

fn p1(solution: &(HashSet<Coord>, Vec<Coord>)) -> i32 {
    solution.0.len() as i32
}

fn manhattan_dist(a: &Coord, b: &Coord) -> i32 {
    (a[0] - b[0]).abs() + (a[1] - b[1]).abs() + (a[2] - b[2]).abs()
}

fn p2(solution: &(HashSet<Coord>, Vec<Coord>)) -> i32 {
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
    let matches = App::new("P19")
        .arg(
            Arg::with_name("file")
                .default_value("d19.txt")
                .takes_value(true),
        )
        .get_matches();
    let input = common::read_file(matches.value_of("file").unwrap());
    let input = parse_input(&input);
    let solution = solve(&input);
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
    fn test_p() {
        let input = parse_input(INPUT);
        let solution = solve(&input);
        let scanner_locations = &solution.1;
        assert_eq!(scanner_locations[0], array![0, 0, 0]);
        assert_eq!(scanner_locations[1], array![68, -1246, -43]);
        assert_eq!(scanner_locations[2], array![1105, -1205, 1229]);
        assert_eq!(scanner_locations[3], array![-92, -2380, -20]);
        assert_eq!(scanner_locations[4], array![-20, -1133, 1061]);
        assert_eq!(p1(&solution), 79);
        assert_eq!(p2(&solution), 3621);
    }
}
