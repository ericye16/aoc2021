use std::{collections::HashSet, num::ParseIntError};

use anyhow::{anyhow, Result};

type Sequence = Vec<u32>;

#[derive(Debug)]
struct Card {
    card: [[u32; 5]; 5],
}

fn parse_card(lines: &mut std::str::Lines) -> Result<Card> {
    let mut card = Card { card: [[0; 5]; 5] };
    for row in 0..5 {
        let values = lines
            .next()
            .unwrap()
            .split(' ')
            .filter_map(|s| s.parse::<u32>().ok())
            .collect::<Vec<u32>>();
        for col in 0..5 {
            card.card[row][col] = values[col];
        }
    }
    Ok(card)
}

fn parse_input(input: &str) -> Result<(Sequence, Vec<Card>)> {
    let mut lines = input.lines();
    let first_line = lines.next().unwrap();
    let sequence: Sequence = first_line
        .split(',')
        .map(|s| s.parse::<u32>())
        .collect::<Result<Vec<u32>, ParseIntError>>()?;
    let mut cards = vec![];
    while let Some(_) = lines.next() {
        let card = parse_card(&mut lines)?;
        cards.push(card);
    }
    Ok((sequence, cards))
}

fn is_winning(card: &Card, already_called: &HashSet<u32>) -> bool {
    for i in 0..5 {
        let mut winning_row = true;
        let mut winning_col = true;
        for j in 0..5 {
            if !already_called.contains(&card.card[i][j]) {
                winning_row = false;
            }
            if !already_called.contains(&card.card[j][i]) {
                winning_col = false;
            }
        }
        if winning_col || winning_row {
            return true;
        }
    }
    false
}

fn calculate_score(card: &Card, already_called: &HashSet<u32>, just_called: u32) -> u32 {
    let mut s = 0;
    for i in 0..5 {
        for j in 0..5 {
            if !already_called.contains(&card.card[i][j]) {
                s += card.card[i][j];
            }
        }
    }
    s * just_called
}

fn p1(sequence: &Sequence, cards: &[Card]) -> Result<u32> {
    let mut already_called = HashSet::new();
    for number in sequence {
        already_called.insert(*number);
        for card in cards {
            if is_winning(card, &already_called) {
                return Ok(calculate_score(card, &already_called, *number));
            }
        }
    }
    Err(anyhow!("Out of numbers!"))
}

fn p2(sequence: &Sequence, cards: &[Card]) -> Result<u32> {
    let mut already_called = HashSet::new();
    let mut already_won = HashSet::new();
    let num_cards = cards.len();
    for number in sequence {
        already_called.insert(*number);
        for (idx, card) in cards.iter().enumerate() {
            if already_won.contains(&idx) {
                continue;
            }
            if is_winning(card, &already_called) {
                if already_won.len() == num_cards - 1 {
                    // This is the last card
                    return Ok(calculate_score(card, &already_called, *number));
                }
                already_won.insert(idx);
            }
        }
    }
    Err(anyhow!("Out of numbers!"))
}

fn main() -> Result<()> {
    let input = common::read_file("d4.txt");
    let (sequence, cards) = parse_input(&input)?;
    println!("p1: {}", p1(&sequence, &cards)?);
    println!("p2: {}", p2(&sequence, &cards)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "1,2,3,4";
        let (sequence, _) = parse_input(input).unwrap();
        assert_eq!(sequence, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_parse_card() {
        let input = "22 13 17 11  0
        8  2 23  4 24
       21  9 14 16  7
        6 10  3 18  5
        1 12 20 15 19";
        let card = parse_card(&mut input.lines()).unwrap();
        assert_eq!(card.card[0], [22, 13, 17, 11, 0]);
        assert_eq!(card.card[4], [1, 12, 20, 15, 19]);
    }

    #[test]
    fn test_p1() {
        let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

        let (sequence, cards) = parse_input(input).unwrap();

        assert_eq!(p1(&sequence, &cards).unwrap(), 4512);
    }

    #[test]
    fn test_p2() {
        let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

        let (sequence, cards) = parse_input(input).unwrap();

        assert_eq!(p2(&sequence, &cards).unwrap(), 1924);
    }
}
