use anyhow::Result;
use common::read_file;

fn parse_line(line: &str) -> Vec<u8> {
    let mut line_num = Vec::<u8>::new();
    let line = line.as_bytes();
    for bit in line {
        if *bit == "1".as_bytes()[0] {
            line_num.push(1u8);
        } else {
            line_num.push(0u8);
        }
    }
    line_num
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    let mut v = vec![];
    for line in input.lines() {
        v.push(parse_line(line.trim()));
    }
    v
}

fn process_line(line: &Vec<u8>, counts: &mut Vec<i32>) -> Result<()> {
    for i in 0..line.len() {
        counts[i] += line[i] as i32;
    }
    Ok(())
}

fn p1(input: &Vec<Vec<u8>>) -> Result<i32> {
    let width = input[0].len();
    let mut counts = vec![0; width];
    let mut total_lines = 0;
    for line in input {
        process_line(line, &mut counts)?;
        total_lines += 1;
    }
    let mut gamma = 0;
    let mut epsilon = 0;
    for i in 0..width {
        let bit = width - i - 1;
        let add = 1 << bit;
        if counts[i] > total_lines - counts[i] {
            gamma += add;
        } else {
            epsilon += add;
        }
    }
    Ok(gamma * epsilon)
}

fn convert_binary(v: &Vec<u8>) -> i32 {
    let mut s = 0;
    for i in 0..v.len() {
        let bit = v.len() - i - 1;
        if v[i] == 1u8 {
            s += 1 << bit;
        }
    }
    s
}

fn p2(input: &Vec<Vec<u8>>) -> Result<i32> {
    let width = input[0].len();
    let mut oxygen = vec![];
    let mut scrubber = vec![];
    for idx in 0..input.len() {
        oxygen.push(idx);
        scrubber.push(idx);
    }
    for w in 0..width {
        let mut oxygen_zeros = vec![];
        let mut oxygen_ones = vec![];
        for idx in &oxygen {
            if input[*idx][w] == 1u8 {
                oxygen_ones.push(*idx);
            } else {
                oxygen_zeros.push(*idx);
            }
        }
        if oxygen_ones.len() >= oxygen_zeros.len() {
            oxygen = oxygen_ones;
        } else {
            oxygen = oxygen_zeros;
        };
        if scrubber.len() > 1 {
            let mut scrubber_zeros = vec![];
            let mut scrubber_ones = vec![];
            for idx in &scrubber {
                if input[*idx][w] == 1u8 {
                    scrubber_ones.push(*idx);
                } else {
                    scrubber_zeros.push(*idx);
                }
            }
            if scrubber_ones.len() < scrubber_zeros.len() {
                scrubber = scrubber_ones;
            } else {
                scrubber = scrubber_zeros;
            };
        }
    }
    let oxygen = convert_binary(&input[oxygen[0]]);
    let scrubber = convert_binary(&input[scrubber[0]]);
    Ok(oxygen * scrubber)
}

fn main() -> Result<()> {
    let input = read_file("d3.txt");
    let input = parse_input(&input);
    println!("p1: {}", p1(&input)?);
    println!("p2: {}", p2(&input)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = "00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010";
        assert_eq!(p1(&parse_input(input)).unwrap(), 198);
    }

    #[test]
    fn test_p2() {
        let input = "00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010";
        assert_eq!(p2(&parse_input(input)).unwrap(), 230);
    }
}
