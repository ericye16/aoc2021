type Crabs = Vec<i32>;

fn parse_input(input: &str) -> Crabs {
    input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect()
}

/*
diffs = sum(abs(x - i))
d diffs / d x = sum (1 if x - i > 0 or -1 if x - i < 0 or 0)
set to zero == equalize aboves and belows
sort and find half point?
*/

fn p1(input: &str) -> i32 {
    let mut crabs = parse_input(input);
    crabs.sort_unstable();
    let pt = crabs[crabs.len() / 2];
    crabs.iter().fold(0, |acc, x| acc + (pt - x).abs())
}

/*
Cost between = 5 -> 16 = 66 = (12 * 11) / 2
*/

fn cost_between(a: i32, b: i32) -> i32 {
    let diff = (a - b).abs();
    (diff + 1) * diff / 2
}

// Lol just brute force it
fn p2(input: &str) -> i32 {
    let crabs = parse_input(input);
    let min = *crabs.iter().min().unwrap();
    let max = *crabs.iter().max().unwrap();
    let mut min_fuel = std::i32::MAX;
    for target in min..(max + 1) {
        let total_fuel: i32 = crabs.iter().map(|crab| cost_between(target, *crab)).sum();
        if total_fuel < min_fuel {
            min_fuel = total_fuel;
        }
    }
    min_fuel
}

fn main() {
    let input = common::read_file("d7.txt");
    println!("P1: {}", p1(input.trim()));
    println!("P2: {}", p2(input.trim()));
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_parse() {
        assert_eq!(parse_input(INPUT), vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]);
    }

    #[test]
    fn test_p1() {
        assert_eq!(p1(INPUT), 37);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(INPUT), 168);
    }
}
