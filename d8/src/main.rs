use lazy_static::lazy_static;
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fmt::Write as FmtWrite;

/*
--- Day 8: Seven Segment Search ---

You barely reach the safety of the cave when the whale smashes into the cave mouth, collapsing it. Sensors indicate another exit to this cave at a much greater depth, so you have no choice but to press on.

As your submarine slowly makes its way through the cave system, you notice that the four-digit seven-segment displays in your submarine are malfunctioning; they must have been damaged during the escape. You'll be in a lot of trouble without them, so you'd better figure out what's wrong.

Each digit of a seven-segment display is rendered by turning on or off any of seven segments named a through g:

  0:      1:      2:      3:      4:
 aaaa    ....    aaaa    aaaa    ....
b    c  .    c  .    c  .    c  b    c
b    c  .    c  .    c  .    c  b    c
 ....    ....    dddd    dddd    dddd
e    f  .    f  e    .  .    f  .    f
e    f  .    f  e    .  .    f  .    f
 gggg    ....    gggg    gggg    ....

  5:      6:      7:      8:      9:
 aaaa    aaaa    aaaa    aaaa    aaaa
b    .  b    .  .    c  b    c  b    c
b    .  b    .  .    c  b    c  b    c
 dddd    dddd    ....    dddd    dddd
.    f  e    f  .    f  e    f  .    f
.    f  e    f  .    f  e    f  .    f
 gggg    gggg    ....    gggg    gggg
So, to render a 1, only segments c and f would be turned on; the rest would be off. To render a 7, only segments a, c, and f would be turned on.

The problem is that the signals which control the segments have been mixed up on each display. The submarine is still trying to display numbers by producing output on signal wires a through g, but those wires are connected to segments randomly. Worse, the wire/segment connections are mixed up separately for each four-digit display! (All of the digits within a display use the same connections, though.)

So, you might know that only signal wires b and g are turned on, but that doesn't mean segments b and g are turned on: the only digit that uses two segments is 1, so it must mean segments c and f are meant to be on. With just that information, you still can't tell which wire (b/g) goes to which segment (c/f). For that, you'll need to collect more information.

For each display, you watch the changing signals for a while, make a note of all ten unique signal patterns you see, and then write down a single four digit output value (your puzzle input). Using the signal patterns, you should be able to work out which pattern corresponds to which digit.

For example, here is what you might see in a single entry in your notes:

acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf
(The entry is wrapped here to two lines so it fits; in your notes, it will all be on a single line.)

Each entry consists of ten unique signal patterns, a | delimiter, and finally the four digit output value. Within an entry, the same wire/segment connections are used (but you don't know what the connections actually are). The unique signal patterns correspond to the ten different ways the submarine tries to render a digit using the current wire/segment connections. Because 7 is the only digit that uses three segments, dab in the above example means that to render a 7, signal lines d, a, and b are on. Because 4 is the only digit that uses four segments, eafb means that to render a 4, signal lines e, a, f, and b are on.

Using this information, you should be able to work out which combination of signal wires corresponds to each of the ten digits. Then, you can decode the four digit output value. Unfortunately, in the above example, all of the digits in the output value (cdfeb fcadb cdfeb cdbaf) use five segments and are more difficult to deduce.

For now, focus on the easy digits. Consider this larger example:

be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce

Because the digits 1, 4, 7, and 8 each use a unique number of segments, you should be able to tell which combinations of signals correspond to those digits. Counting only digits in the output values (the part after | on each line), in the above example, there are 26 instances of digits that use a unique number of segments (highlighted above).

In the output values, how many times do digits 1, 4, 7, or 8 appear?
 */

/*
1 - 2 segments
4 - 4 segments
7 - 3 segments
8 - 7 segments
*/

// Gives list of lengths of segments
fn read_line_for_p1(input: &str) -> Vec<i32> {
    let ouputs = input.trim().split("|").collect::<Vec<&str>>()[1];
    ouputs.trim().split(" ").map(|s| s.len() as i32).collect()
}

fn p1(input: &str) -> i32 {
    let mut counts = 0;
    for line in input.lines().map(read_line_for_p1) {
        line.iter().for_each(|num_segments| {
            if *num_segments == 2 || *num_segments == 4 || *num_segments == 3 || *num_segments == 7
            {
                counts += 1;
            }
        })
    }
    counts
}

/*
--- Part Two ---

Through a little deduction, you should now be able to determine the remaining digits. Consider again the first example above:

acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab |
cdfeb fcadb cdfeb cdbaf
After some careful analysis, the mapping between signal wires and segments only make sense in the following configuration:

 dddd
e    a
e    a
 ffff
g    b
g    b
 cccc
So, the unique signal patterns would correspond to the following digits:

acedgfb: 8
cdfbe: 5
gcdfa: 2
fbcad: 3
dab: 7
cefabd: 9
cdfgeb: 6
eafb: 4
cagedb: 0
ab: 1
Then, the four digits of the output value can be decoded:

cdfeb: 5
fcadb: 3
cdfeb: 5
cdbaf: 3
Therefore, the output value for this entry is 5353.

Following this same process for each entry in the second, larger example above, the output value of each entry can be determined:

fdgacbe cefdb cefbgd gcbe: 8394
fcgedb cgb dgebacf gc: 9781
cg cg fdcagb cbg: 1197
efabcd cedba gadfec cb: 9361
gecf egdcabf bgf bfgea: 4873
gebdcfa ecba ca fadegcb: 8418
cefg dcbef fcge gbcadfe: 4548
ed bcgafe cdgba cbgef: 1625
gbdfcae bgc cg cgb: 8717
fgae cfgab fg bagce: 4315
Adding all of the output values in this larger example produces 61229.

For each entry, determine all of the wire/segment connections and decode the four-digit output values. What do you get if you add up all of the output values?
*/

type Possibilities = BTreeMap<u8, BTreeMap<u8, bool>>;

fn display_possibility(possibilities: &Possibilities) {
    println!("Possibilities:");
    for (segment, possibility_for_segment) in possibilities.iter() {
        let mut s = String::new();
        for (segment_to_match, is_matching) in possibility_for_segment {
            if *is_matching {
                write!(&mut s, "{}, ", *segment_to_match as char).unwrap();
            }
        }
        println!("{}: {}", *segment as char, s);
    }
}

fn create_all_possibilities() -> Possibilities {
    let mut h = BTreeMap::new();
    for a in b'a'..(b'g' + 1) {
        h.insert(a, BTreeMap::new());
        for b in b'a'..(b'g' + 1) {
            h.get_mut(&a).unwrap().insert(b, true);
        }
    }
    h
}

#[derive(Debug, PartialEq, Eq)]
struct DigitSpec {
    segments_idx: BTreeMap<u8, bool>,
    segments_set: BTreeSet<u8>,
}

fn get_digit_spec(digit: u8) -> DigitSpec {
    let mut h = BTreeMap::new();
    for ch in b'a'..(b'g' + 1) {
        h.insert(ch, false);
    }
    let arr = &match digit {
        0 => vec![b'a', b'b', b'c', b'e', b'f', b'g'],
        1 => vec![b'c', b'f'],
        2 => vec![b'a', b'c', b'd', b'e', b'g'],
        3 => vec![b'a', b'c', b'd', b'f', b'g'],
        4 => vec![b'b', b'c', b'd', b'f'],
        5 => vec![b'a', b'b', b'd', b'f', b'g'],
        6 => vec![b'a', b'b', b'd', b'e', b'f', b'g'],
        7 => vec![b'a', b'c', b'f'],
        8 => vec![b'a', b'b', b'c', b'd', b'e', b'f', b'g'],
        9 => vec![b'a', b'b', b'c', b'd', b'f', b'g'],
        _ => panic!("digit not 0-9"),
    };
    for seg in arr {
        h.insert(*seg, true);
    }
    DigitSpec {
        segments_idx: h,
        segments_set: BTreeSet::from_iter(arr.clone()),
    }
}

lazy_static! {
    static ref DIGIT_SPECS: BTreeMap<u8, DigitSpec> = {
        let mut m = BTreeMap::new();
        for d in 0..10 {
            m.insert(d, get_digit_spec(d));
        }
        m
    };
    static ref REVERSE_MAP: BTreeMap<BTreeSet<u8>, u8> = {
        let mut m = BTreeMap::new();
        for (val, spec) in DIGIT_SPECS.iter() {
            m.insert(spec.segments_set.clone(), *val);
        }
        m
    };
}

fn eliminate(digit_spec: &DigitSpec, segments: &str, possibilities: &mut Possibilities) {
    for ch in segments.bytes() {
        let possibility_for_ch = possibilities.get_mut(&ch).unwrap();
        for possibility in b'a'..(b'g' + 1) {
            possibility_for_ch
                .entry(possibility)
                .and_modify(|v| *v = *v && *digit_spec.segments_idx.get(&possibility).unwrap());
        }
    }
}

fn process_digit(digit: &str, possibilities: &mut Possibilities) {
    /*
    Let's count!
    0 -> 6
    1 -> 2 **
    2 -> 5
    3 -> 5
    4 -> 4 **
    5 -> 5
    6 -> 6
    7 -> 3 **
    8 -> 7 **
    9 -> 6
    */
    match digit.len() {
        2 => {
            eliminate(&DIGIT_SPECS.get(&1).unwrap(), digit, possibilities);
        }
        4 => {
            eliminate(&DIGIT_SPECS.get(&4).unwrap(), digit, possibilities);
        }
        3 => {
            eliminate(&DIGIT_SPECS.get(&7).unwrap(), digit, possibilities);
        }
        7 => {
            eliminate(&DIGIT_SPECS.get(&8).unwrap(), digit, possibilities);
        }
        _ => (),
    };
}

fn get_single_possibility(possibilities_for_ch: &BTreeMap<u8, bool>) -> Option<u8> {
    let mut singleton = None;
    let num = possibilities_for_ch
        .iter()
        .fold(0, |acc, (possibility, b)| {
            acc + if *b {
                singleton = Some(*possibility);
                1
            } else {
                0
            }
        });
    if num == 1 {
        singleton
    } else {
        None
    }
}

fn find_known(possibilities: &Possibilities) -> Vec<(u8, u8)> {
    let mut known = vec![];
    for (segment, possibilities_for_segment) in possibilities.iter() {
        if let Some(singleton) = get_single_possibility(possibilities_for_segment) {
            known.push((*segment, singleton));
        }
    }
    known
}

fn propagate_possibilities(possibilities: &mut Possibilities) -> bool {
    let mut modified = true;
    let mut num_loops = 0;
    while modified {
        modified = false;
        let known = find_known(possibilities);
        for (segment, true_segment) in known {
            for (segment_i, segment_i_possibilities) in possibilities.iter_mut() {
                if segment == *segment_i {
                    continue;
                }
                segment_i_possibilities.entry(true_segment).and_modify(|e| {
                    if *e {
                        modified = true;
                        *e = false;
                    }
                });
            }
        }
        num_loops += 1;
    }
    num_loops > 1
}

fn segments_to_number(segments: &BTreeSet<u8>) -> Option<u8> {
    REVERSE_MAP.get(&segments).copied()
}

type Trial = BTreeMap<u8, u8>;

fn segments_to_segments(segments: &str, trial: &Trial) -> BTreeSet<u8> {
    BTreeSet::from_iter(segments.bytes().map(|i| *trial.get(&i).unwrap()))
}

fn make_trials(possibilities: &Possibilities) {
    let mut trial: Trial;
}

// Does an algorithm exist for this? idk
// Do two passes, one where we go through
fn p2_single_line(input: &str) -> i32 {
    let single_line_parts: Vec<&str> = input.split("|").collect();
    let left_part = single_line_parts[0].trim();
    let right_part = single_line_parts[1].trim();
    let mut possibilities = create_all_possibilities();
    for digit in left_part.split(" ") {
        println!("Digit: {}", digit);
        process_digit(digit, &mut possibilities);
        display_possibility(&possibilities);
    }
    println!("Propagating units");
    propagate_possibilities(&mut possibilities);
    display_possibility(&possibilities);
    let known: Vec<(char, char)> = find_known(&possibilities)
        .iter()
        .map(|(a, b)| (*a as char, *b as char))
        .collect();
    println!("{:?}", known);
    todo!()
}

fn p2(input: &str) -> i32 {
    input.lines().map(|s| s.trim()).map(p2_single_line).sum()
}

fn main() {
    let input = common::read_file("d8.txt");
    println!("P1: {}", p1(&input.trim()));
    println!("P2: {}", p2(&input.trim()));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
    edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
    fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
    fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
    aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
    fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
    dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
    bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
    egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
    gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn test_read_line() {
        assert_eq!(
            read_line_for_p1(
                "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"
            ),
            vec![4, 5, 2, 5]
        );
    }

    #[test]
    fn test_p1() {
        assert_eq!(p1(INPUT), 26);
    }

    #[test]
    fn test_p2_single_line() {
        assert_eq!(
            p2_single_line(
                "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"
            ),
        5353);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(INPUT), 61229);
    }
}
