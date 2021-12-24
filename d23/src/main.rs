use core::fmt;
use std::collections::HashSet;

use lazy_static::lazy_static;

lazy_static! {
    static ref GRID: Vec<Vec<char>> = common::read_2d(
        "#############
#...........#
###.#.#.#.###
  #.#.#.#.#
  #########"
    );
    static ref INTERESTING_POSES: Vec<[[Position;2];4]> = vec![

    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position(i32, i32);

#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    animals: [[Position; 2]; 4],
    currently_moving: (usize, usize),
    animal_started_from_room: bool,
    energy: i64,
    last_moved: (usize, usize),
}

impl std::fmt::Display for State {
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
        write!(
            f,
            "{}\n
        energy = {}\n
        last_moved = {:?}\n",
            grids, self.energy, self.last_moved,
        )
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

fn done(state: &State) -> bool {
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

fn can_move(
    state: &State,
    target: &Position,
    already_visited: &HashSet<[[Position; 2]; 4]>,
) -> bool {
    if GRID[target.0 as usize][target.1 as usize] == '#' {
        return false;
    }
    let mut animals = state.animals.clone();
    animals[state.currently_moving.0][state.currently_moving.1] = *target;
    if already_visited.contains(&animals) {
        return false;
    }
    for animal in &state.animals {
        for animal0 in animal {
            if animal0 == target {
                return false;
            }
        }
    }
    let current_position = get_position(&state.currently_moving, &state.animals);
    if current_position.0 == 1 && target.0 == 2 {
        // Cannot enter not your room
        if target.1 != get_col_for(state.currently_moving.0) {
            return false;
        } else {
            // target.1 is going to its room
            // Cannot enter your room if there is another animal in there, that
            // shouldn't be
            for (idx, animal) in state.animals.iter().enumerate() {
                if idx == state.currently_moving.0 {
                    continue;
                }
                for animal0 in animal {
                    if animal0.1 == target.1 && animal0.0 >= 2 {
                        return false;
                    }
                }
            }
        }
    }

    return true;
}

fn in_hall(target: &Position) -> bool {
    target.0 == 1
}

fn rearrange(state: &State, already_visited: &mut HashSet<[[Position; 2]; 4]>) -> Option<i64> {
    // println!("=======\n{}", state);
    if done(&state) {
        println!("Found completed state: \n{}", state);
        return Some(state.energy);
    }
    already_visited.insert(state.animals.clone());
    let current_position = get_position(&state.currently_moving, &state.animals);
    let mut current_min = None;
    // First, try moving current animal, if possible
    for (dr, dc) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
        let target = Position(current_position.0 + dr, current_position.1 + dc);
        if can_move(&state, &target, &already_visited) {
            let mut new_state = state.clone();
            new_state.energy = state.energy + get_energy(&state.currently_moving);
            new_state.animals[state.currently_moving.0][state.currently_moving.1] = target;
            new_state.last_moved = new_state.currently_moving;
            if let Some(new_energy) = rearrange(&new_state, already_visited) {
                if current_min.is_none() || new_energy < current_min.unwrap() {
                    current_min = Some(new_energy);
                }
            }
        }
    }
    // Don't stop outside rooms
    if current_position.0 == 1
        && (current_position.1 == 3
            || current_position.1 == 5
            || current_position.1 == 7
            || current_position.1 == 9)
    {
        return current_min;
    }
    // If the animal started from hall, cannot stop in hall
    if !state.animal_started_from_room {
        if in_hall(&current_position) {
            return current_min;
        }
    }
    // Pick a new animal and keep trying
    let new_moving = if state.currently_moving.1 < 1 {
        (state.currently_moving.0, 1)
    } else {
        ((state.currently_moving.0 + 1) % 4, 0)
    };

    if new_moving == state.last_moved {
        return current_min;
    }

    let mut new_state = state.clone();
    new_state.currently_moving = new_moving;
    new_state.animal_started_from_room =
        get_position(&new_state.currently_moving, &new_state.animals).0 >= 2;
    if let Some(new_energy) = rearrange(&new_state, already_visited) {
        if current_min.is_none() || new_energy < current_min.unwrap() {
            current_min = Some(new_energy);
        }
    }

    current_min
}

fn parse_input(input: &str) -> State {
    let input = common::read_2d(input);
    let mut state = State {
        animals: [[Position(0, 0); 2]; 4],
        currently_moving: (0, 0),
        animal_started_from_room: true,
        energy: 0,
        last_moved: (0, 0),
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
    println!("Initial state: {}", state0);
    let mut already_visited = HashSet::new();
    rearrange(&state0, &mut already_visited).unwrap()
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

    #[test]
    fn test_p1() {
        assert_eq!(p1(INPUT), 12521);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(INPUT), 2758514936282235);
    }
}
