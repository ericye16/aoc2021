use anyhow::Result;

type Population = [i64; 9];

fn transition(population: &mut Population) {
    let to_spawn = population[0];
    for i in 0..8 {
        population[i] = population[i + 1];
    }
    population[6] += to_spawn;
    population[8] = to_spawn;
}

fn count_fish(population: &Population) -> i64 {
    population.iter().sum()
}

fn parse_to_fishes(input: &str) -> Population {
    let mut population = [0; 9];
    input
        .split(',')
        .map(|f| f.parse::<i32>().unwrap())
        .for_each(|fish_count| population[fish_count as usize] += 1);
    population
}

fn p1(input: &str) -> i64 {
    let mut population = parse_to_fishes(input);
    for _ in 0..80 {
        // println!("Population: {:?}, {:?}", population, count_fish(&population));
        transition(&mut population);
    }
    count_fish(&population)
}

fn p2(input: &str) -> i64 {
    let mut population = parse_to_fishes(input);
    for _ in 0..256 {
        // println!("Population: {:?}, {:?}", population, count_fish(&population));
        transition(&mut population);
    }
    count_fish(&population)
}

fn main() -> Result<()> {
    let input = common::read_file("d6.txt");
    println!("P1: {}", p1(input.trim()));
    println!("P2: {}", p2(input.trim()));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = "3,4,3,1,2";
        assert_eq!(p1(input), 5934);
    }

    #[test]
    fn test_p2() {
        let input = "3,4,3,1,2";
        assert_eq!(p2(input), 26984457539);
    }
}
