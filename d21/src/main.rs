use std::collections::HashMap;

use text_io::scan;

fn parse_input(input: &str) -> (i32, i32) {
    let mut lines = input.lines();
    let p1: i32;
    scan!(lines.next().unwrap().trim().bytes() => "Player 1 starting position: {}", p1);
    let p2: i32;
    scan!(lines.next().unwrap().trim().bytes() => "Player 2 starting position: {}", p2);
    (p1, p2)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Die {
    num_rolls: i32,
    val: i32,
}

fn get_next_dieroll(die: &mut Die) -> i32 {
    let d = die.val;
    die.val = (die.val) % 100 + 1;
    die.num_rolls += 1;
    d
}

fn p1(input: &str) -> i32 {
    let (mut p1, mut p2) = parse_input(input);
    let mut score1 = 0;
    let mut score2 = 0;
    let mut die = Die {
        num_rolls: 0,
        val: 1,
    };
    while score1 < 1000 && score2 < 1000 {
        // p1
        let step1 =
            get_next_dieroll(&mut die) + get_next_dieroll(&mut die) + get_next_dieroll(&mut die);
        p1 = (p1 - 1 + step1) % 10 + 1;
        score1 += p1;
        if score1 >= 1000 {
            break;
        }

        // p2
        let step2 =
            get_next_dieroll(&mut die) + get_next_dieroll(&mut die) + get_next_dieroll(&mut die);
        p2 = (p2 - 1 + step2) % 10 + 1;
        score2 += p2;
        // println!("p1: {}, p2: {}, s1: {}, s2: {}", p1, p2, score1, score2);
    }
    let losing_score = std::cmp::min(score1, score2);
    losing_score * die.num_rolls
}

fn do_quantum_roll(states: &HashMap<(i32, i32), i64>) -> HashMap<(i32, i32), i64> {
    let mut new = HashMap::new();
    for ((p, score), q) in states {
        if *score >= 21 {
            continue;
        }
        for (step, newq) in [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)] {
            let newp = (p + step - 1) % 10 + 1;
            let newscore = score + newp;
            *new.entry((newp, newscore)).or_default() += q * newq;
        }
    }
    new
}

fn count_won_or_not(states: &HashMap<(i32, i32), i64>) -> (i64, i64) {
    let winning: i64 = states
        .iter()
        .filter(|((_, s), _)| *s >= 21)
        .map(|(_, q)| q)
        .sum();
    let losing: i64 = states
        .iter()
        .filter(|((_, s), _)| *s < 21)
        .map(|(_, q)| q)
        .sum();
    (winning, losing)
}

fn p2(input: &str) -> i64 {
    let (p1, p2) = parse_input(input);
    let mut p1_states = HashMap::new();
    let mut p2_states = HashMap::new();
    p1_states.insert((p1, 0), 1i64);
    p2_states.insert((p2, 0), 1i64);
    let mut total_p1_winning = 0;
    let mut total_p2_winning = 0;
    loop {
        p1_states = do_quantum_roll(&p1_states);
        let (p1_winning, p1_losing) = count_won_or_not(&p1_states);
        let (_p2_winning, p2_losing) = count_won_or_not(&p2_states);
        total_p1_winning += p1_winning * p2_losing;
        if p1_losing == 0 && p2_losing == 0 {
            break;
        }
        p2_states = do_quantum_roll(&p2_states);
        let (_p1_winning, p1_losing) = count_won_or_not(&p1_states);
        let (p2_winning, p2_losing) = count_won_or_not(&p2_states);
        total_p2_winning += p2_winning * p1_losing;
        if p1_losing == 0 && p2_losing == 0 {
            break;
        }
    }
    std::cmp::max(total_p1_winning, total_p2_winning)
}

fn main() {
    let input = common::read_file("d21.txt");
    println!("P1: {}", p1(&input));
    println!("P2: {}", p2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Player 1 starting position: 4
    Player 2 starting position: 8";

    #[test]
    fn test_p1() {
        assert_eq!(p1(INPUT), 739785);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(INPUT), 444356092776315);
    }
}
