// https://adventofcode.com/2021/day/1
use anyhow::Result;

fn p1(input: &str) -> Result<i32> {
    let mut increments = 0;
    let mut lines = input.lines();
    let mut last_num = lines.next().unwrap().parse::<i32>()?;
    for line in lines {
        let next_num = line.parse::<i32>()?;
        if next_num > last_num {
            increments += 1;
        }
        last_num = next_num;
    }
    Ok(increments)
}

fn p2(input: &str) -> Result<i32> {
    let mut window = std::collections::VecDeque::<i32>::new();
    let mut lines = input.lines();
    let mut increments = 0;
    // "prime" the window
    window.push_back(lines.next().unwrap().parse::<i32>()?);
    window.push_back(lines.next().unwrap().parse::<i32>()?);
    window.push_back(lines.next().unwrap().parse::<i32>()?);
    let mut last_sum: i32 = window.iter().sum();
    for line in lines {
        window.pop_front();
        window.push_back(line.parse::<i32>()?);
        let sum: i32 = window.iter().sum();
        if sum > last_sum {
            increments += 1
        }
        last_sum = sum;
    }
    Ok(increments)
}

fn main() -> Result<()> {
    let input = common::read_file("d1.txt");
    let increments_p1 = p1(&input)?;
    println!("Increments part 1: {}", increments_p1);
    let increments_p2 = p2(&input)?;
    println!("Increments part 2: {}", increments_p2);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p1() {
        let input = "199
200
208
210
200
207
240
269
260
263";
        assert_eq!(p1(input).unwrap(), 7);
    }

    #[test]
    fn test_p2() {
        let input = "199
200
208
210
200
207
240
269
260
263";
        assert_eq!(p2(input).unwrap(), 5);
    }
}
