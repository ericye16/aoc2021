/*
--- Day 10: Syntax Scoring ---

You ask the submarine to determine the best route out of the deep-sea cave, but it only replies:

Syntax error in navigation subsystem on line: all of them
All of them?! The damage is worse than you thought. You bring up a copy of the navigation subsystem (your puzzle input).

The navigation subsystem syntax is made of several lines containing chunks. There are one or more chunks on each line, and chunks contain zero or more other chunks. Adjacent chunks are not separated by any delimiter; if one chunk stops, the next chunk (if any) can immediately start. Every chunk must open and close with one of four legal pairs of matching characters:

If a chunk opens with (, it must close with ).
If a chunk opens with [, it must close with ].
If a chunk opens with {, it must close with }.
If a chunk opens with <, it must close with >.
So, () is a legal chunk that contains no other chunks, as is []. More complex but valid chunks include ([]), {()()()}, <([{}])>, [<>({}){}[([])<>]], and even (((((((((()))))))))).

Some lines are incomplete, but others are corrupted. Find and discard the corrupted lines first.

A corrupted line is one where a chunk closes with the wrong character - that is, where the characters it opens and closes with do not form one of the four legal pairs listed above.

Examples of corrupted chunks include (], {()()()>, (((()))}, and <([]){()}[{}]). Such a chunk can appear anywhere within a line, and its presence causes the whole line to be considered corrupted.

For example, consider the following navigation subsystem:

[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
Some of the lines aren't corrupted, just incomplete; you can ignore these lines for now. The remaining five lines are corrupted:

{([(<{}[<>[]}>{[]{[(<()> - Expected ], but found } instead.
[[<[([]))<([[{}[[()]]] - Expected ], but found ) instead.
[{[{({}]{}}([{[{{{}}([] - Expected ), but found ] instead.
[<(<(<(<{}))><([]([]() - Expected >, but found ) instead.
<{([([[(<>()){}]>(<<{{ - Expected ], but found > instead.
Stop at the first incorrect closing character on each corrupted line.

Did you know that syntax checkers actually have contests to see who can get the high score for syntax errors in a file? It's true! To calculate the syntax error score for a line, take the first illegal character on the line and look it up in the following table:

): 3 points.
]: 57 points.
}: 1197 points.
>: 25137 points.
In the above example, an illegal ) was found twice (2*3 = 6 points), an illegal ] was found once (57 points), an illegal } was found once (1197 points), and an illegal > was found once (25137 points). So, the total syntax error score for this file is 6+57+1197+25137 = 26397 points!

Find the first illegal character in each corrupted line of the navigation subsystem. What is the total syntax error score for those errors?
*/

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
