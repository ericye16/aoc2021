fn matching(ch: char) -> char {
    match ch {
        '{' => '}',
        '[' => ']',
        '(' => ')',
        '<' => '>',
        _ => panic!(),
    }
}

fn score(ch: char) -> i32 {
    match ch {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!(),
    }
}

fn p1_single_line(line: &str) -> i32 {
    let mut stack = vec![];
    for ch in line.chars() {
        match ch {
            '{' | '[' | '(' | '<' => stack.push(ch),
            '}' | ']' | ')' | '>' => {
                if !stack.is_empty() && matching(stack[stack.len() - 1]) == ch {
                    stack.pop();
                } else {
                    return score(ch);
                }
            }
            _ => panic!(),
        }
    }
    0
}

fn p1(input: &str) -> i32 {
    input
        .trim()
        .lines()
        .map(|s| s.trim())
        .map(p1_single_line)
        .sum()
}

fn p2_score(ch: char) -> i64 {
    match ch {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("p2 score char not valid"),
    }
}

fn p2_single_line(line: &str) -> Option<i64> {
    let mut stack = vec![];
    for ch in line.chars() {
        match ch {
            '{' | '[' | '(' | '<' => stack.push(ch),
            '}' | ']' | ')' | '>' => {
                if !stack.is_empty() && matching(stack[stack.len() - 1]) == ch {
                    stack.pop();
                } else {
                    // Corrupt line
                    return None;
                }
            }
            _ => panic!(),
        }
    }
    let mut score = 0;
    while let Some(ch) = stack.pop() {
        let matching = matching(ch);
        let this_score = p2_score(matching);
        score *= 5;
        score += this_score;
    }
    Some(score)
}

fn p2(input: &str) -> i64 {
    let mut scores: Vec<i64> = input
        .trim()
        .lines()
        .map(|s| s.trim())
        .filter_map(p2_single_line)
        .collect();
    scores.sort_unstable();
    scores[scores.len() / 2]
}

fn main() {
    let input = common::read_file("d10.txt");
    println!("P1: {}", p1(input.trim()));
    println!("P2: {}", p2(input.trim()));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "[({(<(())[]>[[{[]{<()<>>
            [(()[<>])]({[<{<<[]>>(
            {([(<{}[<>[]}>{[]{[(<()>
            (((({<>}<{<{<>}{[]{[]{}
            [[<[([]))<([[{}[[()]]]
            [{[{({}]{}}([{[{{{}}([]
            {<[[]]>}<{[{[{[]{()[[[]
            [<(<(<(<{}))><([]([]()
            <{([([[(<>()){}]>(<<{{
            <{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn test_p1() {
        assert_eq!(p1(INPUT), 26397);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(INPUT), 288957);
    }
}
