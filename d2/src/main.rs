use anyhow::{anyhow, Result};

fn p1(input: &str) -> Result<i32> {
    let mut depth = 0;
    let mut forward = 0;
    for line in input.lines() {
        let v: Vec<_> = line.trim().split(' ').collect();
        let command = v[0];
        let val = v[1].parse::<i32>()?;
        match command {
            "forward" => forward += val,
            "down" => depth += val,
            "up" => depth -= val,
            &_ => {
                return Err(anyhow!("{} is not a valid command", command));
            }
        }
    }
    Ok(depth * forward)
}

fn p2(input: &str) -> Result<i32> {
    let mut depth = 0;
    let mut forward = 0;
    let mut aim = 0;
    for line in input.lines() {
        let v: Vec<_> = line.trim().split(' ').collect();
        let command = v[0];
        let val = v[1].parse::<i32>()?;
        match command {
            "forward" => {
                forward += val;
                depth += aim * val;
            }
            "down" => aim += val,
            "up" => aim -= val,
            &_ => {
                return Err(anyhow!("{} is not a valid command", command));
            }
        }
    }
    Ok(depth * forward)
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("d2/data/input.txt")?;
    println!("p1: {}", p1(&input)?);
    println!("p2: {}", p2(&input)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = "forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2";
        assert_eq!(p1(input).unwrap(), 150);
    }

    #[test]
    fn test_p2() {
        let input = "forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2";
        assert_eq!(p2(input).unwrap(), 900);
    }
}
