extern crate jemallocator;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

use core::fmt;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    hash::Hash,
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
    static ref GRID2: Vec<Vec<char>> = common::read_2d(
        "#############
#...........#
###.#.#.#.###
  #.#.#.#.#
  #.#.#.#.#
  #.#.#.#.#
  #########"
    );
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Position(i32, i32);

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct State {
    animals: [Vec<Position>; 4],
    currently_moving: (usize, usize),
    grid: Vec<Vec<char>>,
    num_animals: usize,
    // debug: Vec<Vec<Vec<char>>>,
}

/*
impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.animals == other.animals
            && self.currently_moving == other.currently_moving
            && self.grid == other.grid
            && self.num_animals == other.num_animals
    }
}

impl Eq for State {}

impl Hash for State {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.animals.hash(state);
        self.currently_moving.hash(state);
        self.grid.hash(state);
        self.num_animals.hash(state);
    }
}
*/

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut grid = self.grid.clone();
        let position = get_position(&self.currently_moving, &self.animals);
        grid[position.0 as usize][position.1 as usize] =
            animal_to_char(self.currently_moving.0).to_ascii_lowercase();
        let grids = grid
            .iter()
            .map(|s| s.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n");
        writeln!(f, "{}", grids)
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
    for animal in 0..4 {
        let room_col = get_col_for(animal) as usize;
        let expected_animal = animal_to_char(animal);
        for row in 2..(2 + state.num_animals) {
            if state.grid[row][room_col] != expected_animal {
                return false;
            }
        }
    }
    true
}

fn get_position(animal: &(usize, usize), positions: &[Vec<Position>; 4]) -> Position {
    positions[animal.0][animal.1]
}

fn in_hall(target: &Position) -> bool {
    target.0 == 1
}

fn animal_to_char(animal: usize) -> char {
    match animal {
        0 => 'A',
        1 => 'B',
        2 => 'C',
        3 => 'D',
        _ => panic!(),
    }
}

fn char_to_animal(ch: char) -> usize {
    match ch {
        'A' => 0,
        'B' => 1,
        'C' => 2,
        'D' => 3,
        _ => panic!(),
    }
}

fn set_occupancy(state: &mut State) {
    state.grid = if state.num_animals == 2 {
        GRID.clone()
    } else {
        GRID2.clone()
    };
    for (idx, animal) in state.animals.iter().enumerate() {
        for animal0 in animal {
            let achar = animal_to_char(idx);
            state.grid[animal0.0 as usize][animal0.1 as usize] = achar;
        }
    }
}

fn parse_input(input: &str, n: usize) -> State {
    let input = common::read_2d(input);
    let grid: &Vec<Vec<char>> = if n == 2 { &GRID } else { &GRID2 };
    let mut state = State {
        animals: [
            vec![Position(0, 0); n],
            vec![Position(0, 0); n],
            vec![Position(0, 0); n],
            vec![Position(0, 0); n],
        ],
        currently_moving: (0, 0),
        num_animals: n,
        grid: grid.clone(),
        // debug: vec![],
    };

    let mut animals = vec![0; 4];
    for r in 2..=3 {
        for c in [3, 5, 7, 9] {
            let animal_idx = char_to_animal(input[r][c]);
            if animals[animal_idx] == 0 {
                state.animals[animal_idx][0] = Position(r as i32, c as i32);
                animals[animal_idx] += 1;
            } else {
                state.animals[animal_idx][1] = Position(r as i32, c as i32);
            }
        }
    }
    set_occupancy(&mut state);
    // state.debug.push(state.grid.clone());
    state
}

fn room_ready(state: &State, animal: usize) -> Option<usize> {
    let room_col = get_col_for(animal) as usize;
    // Check no wrong animals in room
    let animal_char = animal_to_char(animal);
    for row in 2..(state.num_animals + 2) {
        if state.grid[row][room_col] != animal_char && state.grid[row][room_col] != '.' {
            return None;
        }
    }
    for row in (2..(state.num_animals + 2)).rev() {
        if state.grid[row][room_col] == '.' {
            for newr in 2..row {
                assert_eq!(state.grid[newr][room_col], '.');
            }
            return Some(row);
        }
    }
    None
}

fn animal_can_move(state: &State, animal: &(usize, usize)) -> bool {
    let position = get_position(animal, &state.animals);
    if position.0 == 1 {
        // in hallway, can only go home if it's free
        return room_ready(state, animal.0).is_some();
    }
    let room = get_col_for(animal.0);
    if position.1 != room {
        // can't be in another room
        return true;
    }
    let expected_char = animal_to_char(animal.0);
    // If there is any animal that shouldn't be in our room, we can move
    for r in (position.0 as usize + 1)..(state.num_animals + 2) {
        if state.grid[r][room as usize] != expected_char {
            return true;
        }
    }
    false
}

fn choose_next_animal(state: &State) -> Option<(usize, usize)> {
    let mut animal = state.currently_moving;
    if animal.1 < state.num_animals - 1 {
        animal.1 += 1;
    } else {
        animal.0 += 1;
        animal.0 %= 4;
        animal.1 = 0;
    }
    while !animal_can_move(state, &animal) {
        if animal.1 < state.num_animals - 1 {
            animal.1 += 1;
        } else {
            animal.0 += 1;
            animal.0 %= 4;
            animal.1 = 0;
        }
        if animal == state.currently_moving {
            return None;
        }
    }
    Some(animal)
}

fn push_explore_state(
    new_state: State,
    new_energy: i64,
    to_explore: &mut BinaryHeap<Reverse<(i64, State)>>,
    visited: &HashSet<State>,
) {
    if !visited.contains(&new_state) {
        to_explore.push(Reverse((new_energy, new_state)));
    }
}

fn outside_room(col: usize) -> bool {
    col == 3 || col == 5 || col == 7 || col == 9
}

/*
fn maybe_add_debug(new_state: &mut State, current_grid: &Vec<Vec<char>>) {
    if new_state.debug[new_state.debug.len() - 1] != *current_grid {
        new_state.debug.push(current_grid.clone());
    }
}
*/

fn solve(initial_state: State) -> i64 {
    let mut to_explore = BinaryHeap::new();
    let mut visited = HashSet::new();
    to_explore.push(Reverse((0, initial_state)));
    while let Some(Reverse((energy, state))) = to_explore.pop() {
        if visited.contains(&state) {
            continue;
        }
        let state_for_clone = State {
            animals: state.animals.clone(),
            currently_moving: state.currently_moving,
            grid: state.grid.clone(),
            num_animals: state.num_animals,
            // debug: vec![],
        };
        visited.insert(state_for_clone);
        // state.debug.push(state.grid.clone());
        if visited.len() % 100000 == 0 {
            println!(
                "Energy: {}, to explore size: {}, visited size: {}, Current state: \n{}",
                energy,
                to_explore.len(),
                visited.len(),
                state,
            );
        }
        if done(&state) {
            // for past_state in &state.debug {
            //     let grids = past_state
            //         .iter()
            //         .map(|s| s.iter().collect::<String>())
            //         .collect::<Vec<String>>()
            //         .join("\n");
            //     println!("Past state:\n{}", grids);
            // }
            println!(
                "Energy: {}, to explore size: {}, visited size: {}, Current state: \n{}",
                energy,
                to_explore.len(),
                visited.len(),
                state,
            );
            return energy;
        }
        let current_position = get_position(&state.currently_moving, &state.animals);

        // First, try moving current animal, if possible
        if in_hall(&current_position) {
            let mut clear_path_to_room = true;
            let target_col = get_col_for(state.currently_moving.0);
            let hall_range = if target_col > current_position.1 {
                (current_position.1 + 1)..(target_col + 1)
            } else {
                target_col..current_position.1
            };
            for hall_col in hall_range {
                if state.grid[1][hall_col as usize] != '.' {
                    clear_path_to_room = false;
                }
            }
            if clear_path_to_room {
                if let Some(final_row) = room_ready(&state, state.currently_moving.0) {
                    let final_row = final_row as i32;
                    let distance = (final_row - current_position.0) as i64
                        + (target_col - current_position.1).abs() as i64;
                    let new_energy = energy + distance * get_energy(&state.currently_moving);
                    let mut new_state = state.clone();
                    new_state.animals[new_state.currently_moving.0][new_state.currently_moving.1] =
                        Position(final_row, target_col);
                    set_occupancy(&mut new_state);
                    // maybe_add_debug(&mut new_state, &state.grid);
                    push_explore_state(new_state, new_energy, &mut to_explore, &visited);
                }
            }
        } else {
            // Check if animal in room is blocking
            let mut room_blocked = false;
            for row_idx in 1..(current_position.0 as usize) {
                if state.grid[row_idx][current_position.1 as usize] != '.' {
                    room_blocked = true;
                }
            }
            if !room_blocked {
                let hall_row = 1_usize;
                let mut col = current_position.1 as usize - 1;
                while state.grid[hall_row][col] == '.' {
                    if outside_room(col) {
                        col -= 1;
                        continue;
                    }
                    let target = Position(hall_row as i32, col as i32);
                    let dist = (current_position.0 - target.0) as i64
                        + (current_position.1 - target.1).abs() as i64;
                    let mut new_state = state.clone();
                    let new_energy = energy + dist * get_energy(&state.currently_moving);
                    new_state.animals[state.currently_moving.0][state.currently_moving.1] = target;
                    set_occupancy(&mut new_state);
                    // maybe_add_debug(&mut new_state, &state.grid);
                    push_explore_state(new_state, new_energy, &mut to_explore, &visited);
                    col -= 1;
                }
                col = current_position.1 as usize + 1;
                while state.grid[hall_row][col] == '.' {
                    if outside_room(col) {
                        col += 1;
                        continue;
                    }
                    let target = Position(hall_row as i32, col as i32);
                    let dist = (current_position.0 - target.0) as i64
                        + (current_position.1 - target.1).abs() as i64;
                    let mut new_state = state.clone();
                    let new_energy = energy + dist * get_energy(&state.currently_moving);
                    new_state.animals[state.currently_moving.0][state.currently_moving.1] = target;
                    set_occupancy(&mut new_state);
                    // maybe_add_debug(&mut new_state, &state.grid);
                    push_explore_state(new_state, new_energy, &mut to_explore, &visited);
                    col += 1;
                }
            }
        }
        // Pick a new animal and keep trying
        if let Some(new_moving) = choose_next_animal(&state) {
            let mut new_state = state.clone();
            new_state.currently_moving = new_moving;
            // maybe_add_debug(&mut new_state, &state.grid);
            push_explore_state(new_state, energy, &mut to_explore, &visited);
        }
    }
    panic!("Ran out of things to try");
}

fn p1(input: &str) -> i64 {
    let state0 = parse_input(input, 2);
    solve(state0)
}

fn p2(input: &str) -> i64 {
    let mut state0 = parse_input(input, 4);
    for animal in &mut state0.animals {
        for animal0 in animal {
            if animal0.0 == 3 {
                animal0.0 = 5;
            }
        }
    }
    /* Need to insert
      #D#C#B#A#
      #D#B#A#C#
    */
    state0.animals[0][2] = Position(3, 9);
    state0.animals[0][3] = Position(4, 7);
    state0.animals[1][2] = Position(3, 7);
    state0.animals[1][3] = Position(4, 5);
    state0.animals[2][2] = Position(3, 5);
    state0.animals[2][3] = Position(4, 9);
    state0.animals[3][2] = Position(3, 3);
    state0.animals[3][3] = Position(4, 3);
    set_occupancy(&mut state0);
    // state0.debug.push(state0.grid.clone());
    solve(state0)
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
        let state = parse_input(input, 2);
        assert!(done(&state));
        assert_eq!(p1(input), 0);
    }

    #[test]
    fn test_p1() {
        assert_eq!(p1(INPUT), 12521);
    }

    #[test]
    #[ignore]
    fn test_p2() {
        assert_eq!(p2(INPUT), 44169);
    }
}
