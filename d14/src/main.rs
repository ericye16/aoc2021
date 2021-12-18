use std::collections::HashMap;

use text_io::scan;

/*
--- Day 14: Extended Polymerization ---

The incredible pressures at this depth are starting to put a strain on your submarine. The submarine has polymerization equipment that would produce suitable materials to reinforce the submarine, and the nearby volcanically-active caves should even have the necessary input elements in sufficient quantities.

The submarine manual contains instructions for finding the optimal polymer formula; specifically, it offers a polymer template and a list of pair insertion rules (your puzzle input). You just need to work out what polymer would result after repeating the pair insertion process a few times.

For example:

NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
The first line is the polymer template - this is the starting point of the process.

The following section defines the pair insertion rules. A rule like AB -> C means that when elements A and B are immediately adjacent, element C should be inserted between them. These insertions all happen simultaneously.

So, starting with the polymer template NNCB, the first step simultaneously considers all three pairs:

The first pair (NN) matches the rule NN -> C, so element C is inserted between the first N and the second N.
The second pair (NC) matches the rule NC -> B, so element B is inserted between the N and the C.
The third pair (CB) matches the rule CB -> H, so element H is inserted between the C and the B.
Note that these pairs overlap: the second element of one pair is the first element of the next pair. Also, because all pairs are considered simultaneously, inserted elements are not considered to be part of a pair until the next step.

After the first step of this process, the polymer becomes NCNBCHB.

Here are the results of a few steps using the above rules:

Template:     NNCB
After step 1: NCNBCHB
After step 2: NBCCNBBBCBHCB
After step 3: NBBBCNCCNBBNBNBBCHBHHBCHB
After step 4: NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB
This polymer grows quickly. After step 5, it has length 97; After step 10, it has length 3073. After step 10, B occurs 1749 times, C occurs 298 times, H occurs 191 times, and N occurs 865 times; taking the quantity of the most common element (B, 1749) and subtracting the quantity of the least common element (H, 161) produces 1749 - 161 = 1588.

Apply 10 steps of pair insertion to the polymer template and find the most and least common elements in the result. What do you get if you take the quantity of the most common element and subtract the quantity of the least common element?
*/

#[derive(Debug, Clone, PartialEq, Eq)]
struct InsertionRule {
    left: (char, char),
    right: char,
}

/* Old solution

fn transition(polymer: &Vec<char>, rules: &[InsertionRule]) -> Vec<char> {
    let mut s = Vec::new();
    for i in 0..(polymer.len() - 1) {
        s.push(polymer[i]);
        for rule in rules {
            if polymer[i] == rule.left.0 && polymer[i + 1] == rule.left.1 {
                s.push(rule.right);
            }
        }
    }
    s.push(polymer[polymer.len() - 1]);
    s
}

fn count_most_and_least_frequent_and_diff(polymer: &Vec<char>) -> i64 {
    let mut frequencies = HashMap::new();
    let mut maxn = 1;
    let mut minch = polymer[0];
    let mut minn = 1;
    for ch in polymer {
        let this_entry = frequencies.entry(ch).or_insert(0);
        *this_entry += 1;
        if *this_entry > maxn {
            maxn = *this_entry;
        }
        if *ch == minch {
            minn = *this_entry;
        }
        if *this_entry < minn {
            minn = *this_entry;
            minch = *ch;
        }
    }
    maxn - minn
}

fn p1_old(input: &str) -> i64 {
    let (polymer_template, rules) = parse_input(input);
    let mut polymer = polymer_template.chars().collect();
    for _ in 0..10 {
        polymer = transition(&polymer, &rules);
    }
    count_most_and_least_frequent_and_diff(&polymer)
}
*/

fn parse_input(input: &str) -> (String, Vec<InsertionRule>) {
    let mut lines = input.lines().map(str::trim);
    let polymer_template = lines.next().unwrap();
    lines.next();
    let mut rules = vec![];
    for line in lines {
        let left: String;
        let right: String;
        scan!(line.bytes() => "{} -> {}", left, right);
        let mut left = left.chars();
        let left_1 = left.next().unwrap();
        let left_2 = left.next().unwrap();
        rules.push(InsertionRule {
            left: (left_1, left_2),
            right: right.chars().next().unwrap(),
        });
    }
    (polymer_template.to_string(), rules)
}

/*

--- Part Two ---

The resulting polymer isn't nearly strong enough to reinforce the submarine. You'll need to run more steps of the pair insertion process; a total of 40 steps should do it.

In the above example, the most common element is B (occurring 2192039569602 times) and the least common element is H (occurring 3849876073 times); subtracting these produces 2188189693529.

Apply 40 steps of pair insertion to the polymer template and find the most and least common elements in the result. What do you get if you take the quantity of the most common element and subtract the quantity of the least common element?
*/

#[derive(Debug, Clone, PartialEq, Eq)]
struct PolymerPairs {
    pairs: HashMap<(char, char), i64>,
    first: char,
    last: char,
}

fn polymer_to_polymerpairs(polymer: &Vec<char>) -> PolymerPairs {
    PolymerPairs {
        pairs: {
            let mut pairs = HashMap::new();
            for i in 0..(polymer.len() - 1) {
                let entry = pairs.entry((polymer[i], polymer[i + 1])).or_insert(0);
                *entry += 1;
            }
            pairs
        },
        first: polymer[0],
        last: polymer[polymer.len() - 1],
    }
}

fn transition_pairs(polymer_pairs: &mut PolymerPairs, rules: &[InsertionRule]) {
    let mut new_pairs = polymer_pairs.pairs.clone();
    for rule in rules {
        if let Some(pair_cnt) = polymer_pairs.pairs.get(&rule.left).copied() {
            let left_pair = (rule.left.0, rule.right);
            let right_pair = (rule.right, rule.left.1);
            // println!(
            //     "Found pair {:?} count {}, inserting {:?}, {:?}",
            //     rule.left, pair_cnt, left_pair, right_pair
            // );
            let left_entry = new_pairs.entry(left_pair).or_insert(0);
            *left_entry += pair_cnt;
            let right_entry = new_pairs.entry(right_pair).or_insert(0);
            *right_entry += pair_cnt;
            let old_entry = new_pairs.get_mut(&rule.left).unwrap();
            *old_entry -= pair_cnt;
        }
    }
    polymer_pairs.pairs = new_pairs;
}

fn count_diff(polymer_pairs: &PolymerPairs) -> i64 {
    let mut frequencies = HashMap::new();
    for (pair, pair_cnt) in &polymer_pairs.pairs {
        *frequencies.entry(pair.0).or_insert(0) += pair_cnt;
        // *frequencies.entry(pair.1).or_insert(0) += 1;
    }
    *frequencies.entry(polymer_pairs.last).or_insert(0) += 1;
    let maxn = *frequencies.values().max().unwrap();
    let minn = *frequencies.values().min().unwrap();
    // frequencies[&polymer_pairs.first] -= 1;
    maxn - minn
}

fn p1(input: &str) -> i64 {
    let (polymer_template, rules) = parse_input(input);
    let polymer = polymer_template.chars().collect();
    let mut polymer_pairs = polymer_to_polymerpairs(&polymer);
    // println!("Transition pairs: {:?}", polymer_pairs.pairs);
    for _ in 0..10 {
        transition_pairs(&mut polymer_pairs, &rules);
        // println!("Transition pairs: {:?}", polymer_pairs.pairs);
    }
    count_diff(&polymer_pairs)
}

fn p2(input: &str) -> i64 {
    let (polymer_template, rules) = parse_input(input);
    let polymer = polymer_template.chars().collect();
    let mut polymer_pairs = polymer_to_polymerpairs(&polymer);
    for _ in 0..40 {
        transition_pairs(&mut polymer_pairs, &rules);
    }
    count_diff(&polymer_pairs)
}

fn main() {
    let input = common::read_file("d14.txt");
    println!("P1: {}", p1(input.trim()));
    println!("P2: {}", p2(input.trim()));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "NNCB

    CH -> B
    HH -> N
    CB -> H
    NH -> C
    HB -> C
    HC -> B
    HN -> C
    NN -> C
    BH -> H
    NC -> B
    NB -> B
    BN -> B
    BB -> N
    BC -> B
    CC -> N
    CN -> C";

    #[test]
    fn test_p1() {
        assert_eq!(p1(INPUT), 1588);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(INPUT), 2188189693529);
    }
}
