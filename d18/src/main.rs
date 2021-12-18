#[derive(Debug, Clone, PartialEq, Eq)]
struct Pair {
    left: Element,
    right: Element,
}

impl std::fmt::Display for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.left, self.right)
    }
}

impl Pair {
    fn from_parse(input: &str) -> Pair {
        let element = Element::from_parse(input.as_bytes()).0;
        element.into_pair().unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Element {
    Number(i32),
    Pair(Box<Pair>),
}

impl std::fmt::Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Element::Number(n) => write!(f, "{}", n),
            Element::Pair(p) => write!(f, "{}", *p),
        }
    }
}

impl Element {
    fn to_number(&self) -> Option<i32> {
        match self {
            &Self::Number(s) => Some(s),
            _ => None,
        }
    }

    fn into_pair(self) -> Option<Pair> {
        match self {
            Self::Pair(p) => Some(*p),
            _ => None,
        }
    }

    fn from_parse(input: &[u8]) -> (Element, usize) {
        if input[0] == b'[' {
            let (l, additional_idx) = Element::from_parse(&input[1..]);
            let i2 = 1 + additional_idx + 1;
            let (r, a2) = Element::from_parse(&input[i2..]);
            (
                Element::Pair(Box::new(Pair { left: l, right: r })),
                i2 + a2 + 1,
            )
        } else {
            for i in 0..input.len() {
                if input[i] == b',' || input[i] == b']' {
                    return (
                        Element::Number(
                            std::str::from_utf8(&input[0..i]).unwrap().parse().unwrap(),
                        ),
                        i,
                    );
                }
            }
            panic!()
        }
    }
}

fn add(left: Element, right: Element) -> Pair {
    Pair { left, right }
}

fn add_leftwards(element: &mut Element, val: i32) {
    match element {
        Element::Number(n) => {
            *n += val;
        }
        Element::Pair(p) => add_leftwards(&mut p.left, val),
    }
}

fn add_rightwards(element: &mut Element, val: i32) {
    match element {
        Element::Number(n) => {
            *n += val;
        }
        Element::Pair(p) => add_rightwards(&mut p.right, val),
    }
}

fn maybe_explode0(pair: &mut Pair, level: i32) -> (Option<i32>, Option<i32>, bool) {
    if level >= 4 {
        // Reduce! Guaranteed to be numbers somehow?
        return (
            Some(pair.left.to_number().unwrap()),
            Some(pair.right.to_number().unwrap()),
            true,
        );
    } else {
        if let Element::Pair(ref mut p) = &mut pair.left {
            let (l, r, exploded) = maybe_explode0(&mut *p, level + 1);
            if exploded {
                // Left must be passed up
                // Right can go into right pair (guaranteed to work)
                if r.is_some() {
                    add_leftwards(&mut pair.right, r.unwrap());
                    if level == 3 {
                        pair.left = Element::Number(0);
                    }
                }
                return (l, None, true);
            }
        }
        if let Element::Pair(ref mut p) = &mut pair.right {
            let (l, r, exploded) = maybe_explode0(&mut *p, level + 1);
            if exploded {
                if l.is_some() {
                    add_rightwards(&mut pair.left, l.unwrap());
                    if level == 3 {
                        pair.right = Element::Number(0);
                    }
                }
                return (None, r, true);
            }
        }
        return (None, None, false);
    }
}

fn maybe_split(element: &mut Element) -> bool {
    match element {
        Element::Number(n) => {
            if *n >= 10 {
                *element = Element::Pair(Box::new(Pair {
                    left: Element::Number(*n / 2),
                    right: Element::Number((*n + 1) / 2),
                }));
                true
            } else {
                false
            }
        }
        Element::Pair(p) => maybe_split0(p),
    }
}

fn maybe_split0(pair: &mut Pair) -> bool {
    if maybe_split(&mut pair.left) {
        true
    } else {
        maybe_split(&mut pair.right)
    }
}

fn maybe_reduce(pair: &mut Pair) -> bool {
    let (_, _, explode) = maybe_explode0(pair, 0);
    if explode {
        return true;
    }
    let split = maybe_split0(pair);
    if split {
        return true;
    }
    return false;
}

fn reduce_to_completion(pair: &mut Pair) {
    let mut keep_reduce = true;
    while keep_reduce {
        keep_reduce = maybe_reduce(pair);
    }
}

fn sum_lines(input: &str) -> Pair {
    let mut lines = input.lines();
    let mut p = Pair::from_parse(lines.next().unwrap().trim());
    for line in lines {
        p = add(
            Element::Pair(Box::new(p)),
            Element::Pair(Box::new(Pair::from_parse(line.trim()))),
        );
        reduce_to_completion(&mut p);
    }
    p
}

fn magnitude(pair: &Pair) -> i32 {
    3 * match &pair.left {
        Element::Number(n) => *n,
        Element::Pair(p) => magnitude(p),
    } + 2 * match &pair.right {
        Element::Number(n) => *n,
        Element::Pair(p) => magnitude(p),
    }
}

fn p1(input: &str) -> i32 {
    let sum = sum_lines(input);
    magnitude(&sum)
}

fn add_and_reduce(a: &Pair, b: &Pair) -> i32 {
    let mut c = add(
        Element::Pair(Box::new(a.clone())),
        Element::Pair(Box::new(b.clone())),
    );
    reduce_to_completion(&mut c);
    magnitude(&c)
}

fn p2(input: &str) -> i32 {
    let pairs: Vec<Pair> = input.lines().map(str::trim).map(Pair::from_parse).collect();
    let mut max_mag = 0;
    for i in 0..pairs.len() {
        for j in 0..pairs.len() {
            if i == j {
                continue;
            }
            let magnitude = add_and_reduce(&pairs[i], &pairs[j]);
            if magnitude > max_mag {
                max_mag = magnitude
            }
        }
    }
    max_mag
}

fn main() {
    let input = common::read_file("d18.txt");
    println!("P1: {}", p1(&input.trim()));
    println!("P2: {}", p2(&input.trim()));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reduce0(mut pair: Pair) -> Pair {
        maybe_reduce(&mut pair);
        pair
    }

    #[test]
    fn test_addition() {
        assert_eq!(
            Pair::from_parse("[[1,2],[[3,4],5]]"),
            add(
                Element::from_parse(b"[1,2]").0,
                Element::from_parse(b"[[3,4],5]").0
            )
        );
    }

    #[test]
    fn test_explode() {
        assert_eq!(
            reduce0(Pair::from_parse("[[[[[9,8],1],2],3],4]")),
            Pair::from_parse("[[[[0,9],2],3],4]")
        );
        assert_eq!(
            reduce0(Pair::from_parse("[7,[6,[5,[4,[3,2]]]]]")),
            Pair::from_parse("[7,[6,[5,[7,0]]]]")
        );
        assert_eq!(
            reduce0(Pair::from_parse("[[6,[5,[4,[3,2]]]],1]")),
            Pair::from_parse("[[6,[5,[7,0]]],3]")
        );
        assert_eq!(
            reduce0(Pair::from_parse("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]")),
            Pair::from_parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]")
        );
        assert_eq!(
            reduce0(Pair::from_parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]")),
            Pair::from_parse("[[3,[2,[8,0]]],[9,[5,[7,0]]]]")
        );
    }

    #[test]
    fn test_failing() {
        let a = Pair::from_parse("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]");
        let b = Pair::from_parse("[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]");
        let mut c = add(Element::Pair(Box::new(a)), Element::Pair(Box::new(b)));
        println!("c: {}", c);
        reduce_to_completion(&mut c);
        assert_eq!(
            c,
            Pair::from_parse("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]")
        );
    }

    #[test]
    fn test_sum() {
        assert_eq!(
            sum_lines(
                "[1,1]
                [2,2]
                [3,3]
                [4,4]"
            ),
            Pair::from_parse("[[[[1,1],[2,2]],[3,3]],[4,4]]")
        );

        assert_eq!(
            sum_lines(
                "[1,1]
                [2,2]
                [3,3]
                [4,4]
                [5,5]"
            ),
            Pair::from_parse("[[[[3,0],[5,3]],[4,4]],[5,5]]")
        );

        assert_eq!(
            sum_lines(
                "[1,1]
                [2,2]
                [3,3]
                [4,4]
                [5,5]
                [6,6]"
            ),
            Pair::from_parse("[[[[5,0],[7,4]],[5,5]],[6,6]]")
        );

        assert_eq!(
            sum_lines(
                "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
                [7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
                [[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
                [[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
                [7,[5,[[3,8],[1,4]]]]
                [[2,[2,2]],[8,[8,1]]]
                [2,9]
                [1,[[[9,3],9],[[9,0],[0,7]]]]
                [[[5,[7,4]],7],1]
                [[[[4,2],2],6],[8,7]]"
            ),
            Pair::from_parse("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")
        );

        assert_eq!(
            sum_lines(
                "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
                [[[5,[2,8]],4],[5,[[9,9],0]]]
                [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
                [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
                [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
                [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
                [[[[5,4],[7,7]],8],[[8,3],8]]
                [[9,3],[[9,9],[6,[4,9]]]]
                [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
                [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"
            ),
            Pair::from_parse("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]")
        );
    }

    #[test]
    fn test_split() {
        assert_eq!(
            reduce0(Pair::from_parse("[10,0]")),
            Pair::from_parse("[[5,5],0]")
        );
        assert_eq!(
            reduce0(Pair::from_parse("[11,0]")),
            Pair::from_parse("[[5,6],0]")
        );
        assert_eq!(
            reduce0(Pair::from_parse("[12,0]")),
            Pair::from_parse("[[6,6],0]")
        );
    }

    #[test]
    fn test_sequence() {
        let a = Pair::from_parse("[[[[4,3],4],4],[7,[[8,4],9]]]");
        let b = Pair::from_parse("[1,1]");
        let mut c = add(Element::Pair(Box::new(a)), Element::Pair(Box::new(b)));
        assert_eq!(c, Pair::from_parse("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]"));

        let reduced = maybe_reduce(&mut c);
        assert!(reduced);
        assert_eq!(c, Pair::from_parse("[[[[0,7],4],[7,[[8,4],9]]],[1,1]]"));

        let reduced = maybe_reduce(&mut c);
        assert!(reduced);
        assert_eq!(c, Pair::from_parse("[[[[0,7],4],[15,[0,13]]],[1,1]]"));

        let reduced = maybe_reduce(&mut c);
        assert!(reduced);
        assert_eq!(c, Pair::from_parse("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]"));

        let reduced = maybe_reduce(&mut c);
        assert!(reduced);
        assert_eq!(c, Pair::from_parse("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]"));

        let reduced = maybe_reduce(&mut c);
        assert!(reduced);
        assert_eq!(c, Pair::from_parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"));

        let reduced = maybe_reduce(&mut c);
        assert!(!reduced);
        assert_eq!(c, Pair::from_parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"));
    }

    #[test]
    fn test_magnitudes() {
        assert_eq!(magnitude(&Pair::from_parse("[9,1]")), 29);
        assert_eq!(magnitude(&Pair::from_parse("[1,9]")), 21);
        assert_eq!(magnitude(&Pair::from_parse("[[9,1],[1,9]]")), 129);

        assert_eq!(magnitude(&Pair::from_parse("[[1,2],[[3,4],5]]")), 143);
        assert_eq!(
            magnitude(&Pair::from_parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")),
            1384
        );
        assert_eq!(
            magnitude(&Pair::from_parse("[[[[1,1],[2,2]],[3,3]],[4,4]]")),
            445
        );
        assert_eq!(
            magnitude(&Pair::from_parse("[[[[3,0],[5,3]],[4,4]],[5,5]]")),
            791
        );
        assert_eq!(
            magnitude(&Pair::from_parse("[[[[5,0],[7,4]],[5,5]],[6,6]]")),
            1137
        );
        assert_eq!(
            magnitude(&Pair::from_parse(
                "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
            )),
            3488
        );
    }

    #[test]
    fn test_p1() {
        assert_eq!(
            p1("[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
        [[[5,[2,8]],4],[5,[[9,9],0]]]
        [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
        [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
        [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
        [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
        [[[[5,4],[7,7]],8],[[8,3],8]]
        [[9,3],[[9,9],[6,[4,9]]]]
        [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
        [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"),
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
