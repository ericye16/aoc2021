use core::fmt;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use lazy_static::lazy_static;

lazy_static! {
    static ref GRID: Vec<Vec<char>> = common::read_2d(
        "#############
#...........#
###.#.#.#.###
  #.#.#.#.#
  #########"
    );
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Position(i32, i32);

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct State2 {
    animals: [[Position; 2]; 4],
    currently_moving: (usize, usize),
    started_from_room: bool,
}

impl std::fmt::Display for State2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut grid = GRID.clone();
        for (idx, animals) in self.animals.iter().enumerate() {
            for (idx0, animal) in animals.iter().enumerate() {
                if (idx, idx0) == self.currently_moving {
                    grid[animal.0 as usize][animal.1 as usize] = match idx {
                        0 => 'A',
                        1 => 'B',
                        2 => 'C',
                        3 => 'D',
                        _ => panic!(),
                    };
                } else {
                    grid[animal.0 as usize][animal.1 as usize] = match idx {
                        0 => 'a',
                        1 => 'b',
                        2 => 'c',
                        3 => 'd',
                        _ => panic!(),
                    };
                }
            }
        }
        let grids = grid
            .iter()
            .map(|s| s.into_iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}\n", grids)
    }
}

fn get_energy(animal: &(usize, usize)) -> i64 {
    match animal.0 {
        0 => 1,
        1 => 10,
        2 => 100,
        3 => 1000,
        _ => panic!(),
    }
}

fn get_col_for(animal: usize) -> i32 {
    match animal {
        0 => 3,
        1 => 5,
        2 => 7,
        3 => 9,
        _ => panic!(),
    }
}

fn done(state: &State2) -> bool {
    state.animals.iter().enumerate().all(|(idx, animal)| {
        animal[0] != animal[1]
            && animal
                .iter()
                .all(|Position(r, c)| (*r == 2 || *r == 3) && *c == get_col_for(idx))
    })
}

fn get_position(animal: &(usize, usize), positions: &[[Position; 2]; 4]) -> Position {
    positions[animal.0][animal.1].clone()
}

fn in_hall(target: &Position) -> bool {
    target.0 == 1
}

fn parse_input(input: &str) -> State2 {
    let input = common::read_2d(input);
    let mut state = State2 {
        animals: [[Position(0, 0); 2]; 4],
        currently_moving: (0, 0),
        started_from_room: true,
    };

    let mut animals = vec![0; 4];
    for r in 2..=3 {
        for c in [3, 5, 7, 9] {
            match input[r][c] {
                'A' => {
                    if animals[0] == 0 {
                        state.animals[0][0] = Position(r as i32, c as i32);
                        animals[0] += 1;
                    } else {
                        state.animals[0][1] = Position(r as i32, c as i32);
                    }
                }
                'B' => {
                    if animals[1] == 0 {
                        state.animals[1][0] = Position(r as i32, c as i32);
                        animals[1] += 1;
                    } else {
                        state.animals[1][1] = Position(r as i32, c as i32);
                    }
                }
                'C' => {
                    if animals[2] == 0 {
                        state.animals[2][0] = Position(r as i32, c as i32);
                        animals[2] += 1;
                    } else {
                        state.animals[2][1] = Position(r as i32, c as i32);
                    }
                }
                'D' => {
                    if animals[3] == 0 {
                        state.animals[3][0] = Position(r as i32, c as i32);
                        animals[3] += 1;
                    } else {
                        state.animals[3][1] = Position(r as i32, c as i32);
                    }
                }
                _ => panic!(),
            }
        }
    }

    state
}

fn p1(input: &str) -> i64 {
    let state0 = parse_input(input);
    let mut to_explore = BinaryHeap::new();
    let mut visited = HashSet::new();
    to_explore.push(Reverse((0, state0)));
    while let Some(Reverse((energy, state))) = to_explore.pop() {
        if visited.contains(&state) {
            continue;
        }
        visited.insert(state.clone());
        if done(&state) {
            return energy;
        }
        if visited.len() % 100000 == 0 {
            println!(
                "Energy: {}, to explore size: {}, visited size: {}, Current state: \n{}",
                energy,
                to_explore.len(),
                visited.len(),
                state,
            );
        }
        let current_position = get_position(&state.currently_moving, &state.animals);

        // First, try moving current animal, if possible
        for (dr, dc) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let target = Position(current_position.0 + dr, current_position.1 + dc);
            if GRID[target.0 as usize][target.1 as usize] == '#' {
                continue;
            }
            let animal_in_spot = state
                .animals
                .iter()
                .any(|animal| animal.into_iter().any(|animal0| *animal0 == target));
            if animal_in_spot {
                continue;
            }
            let mut new_state = state.clone();
            let new_energy = energy + get_energy(&state.currently_moving);
            new_state.animals[state.currently_moving.0][state.currently_moving.1] = target;
            if current_position.0 == 1 && target.0 == 2 {
                // Cannot enter not your room
                if target.1 != get_col_for(state.currently_moving.0) {
                    continue;
                } else {
                    // target.1 is going to its room
                    // Cannot enter your room if there is another animal in there, that
                    // shouldn't be
                    let mut other_animal_in_room = false;
                    for (idx, animal) in state.animals.iter().enumerate() {
                        // Other animal of the same type is fine
                        if idx == state.currently_moving.0 {
                            continue;
                        }
                        for animal0 in animal {
                            if animal0.1 == target.1 && animal0.0 >= 2 {
                                other_animal_in_room = true;
                            }
                        }
                    }
                    if other_animal_in_room {
                        continue;
                    }
                }
            }

            if visited.contains(&new_state) {
                continue;
            }
            to_explore.push(Reverse((new_energy, new_state)));
        }

        // Otherwise, switch to another animal, if possible
        // Don't stop outside rooms
        if current_position.0 == 1
            && (current_position.1 == 3
                || current_position.1 == 5
                || current_position.1 == 7
                || current_position.1 == 9)
        {
            continue;
        }
        // If the animal started from hall, cannot stop in hall
        if !state.started_from_room {
            if in_hall(&current_position) {
                continue;
            }
        }
        // Pick a new animal and keep trying
        let new_moving = if state.currently_moving.1 < 1 {
            (state.currently_moving.0, 1)
        } else {
            ((state.currently_moving.0 + 1) % 4, 0)
        };
        let mut new_state = state.clone();
        new_state.currently_moving = new_moving;
        if visited.contains(&new_state) {
            continue;
        }
        to_explore.push(Reverse((energy, new_state)));
    }
    panic!("Ran out of things to try");
}

fn p2(input: &str) -> i64 {
    todo!()
}

fn main() {
    let input = common::read_file("d23.txt");
    println!("P1: {}", p1(&input));
    println!("P2: {}", p2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";

    #[test]
    fn test_done() {
        let input = "#############
#...........#
###A#B#C#D###
  #A#B#C#D#
  #########";
        let state = parse_input(input);
        assert!(done(&state));
        assert_eq!(p1(input), 0);
    }

    #[ignore]
    #[test]
    fn test_p1() {
        assert_eq!(p1(INPUT), 12521);
    }

    #[ignore]
    #[test]
    fn test_p2() {
        assert_eq!(p2(INPUT), 44169);
    }
}
