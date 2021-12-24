#![feature(array_zip)]
use std::collections::HashMap;

use lazy_static::lazy_static;
use text_io::read;

#[allow(dead_code)]
fn print_registers(registers: &HashMap<char, i64>) {
    println!(
        "[w:{},x:{},y:{},z:{}]",
        registers[&'w'], registers[&'x'], registers[&'y'], registers[&'z']
    );
}

#[allow(dead_code)]
fn run_program(program: &[Vec<&str>], inputs: &[i64], presets: &HashMap<char, i64>) -> i64 {
    let mut registers = HashMap::new();
    registers.insert('w', 0);
    registers.insert('x', 0);
    registers.insert('y', 0);
    registers.insert('z', 0);
    for preset in presets {
        *registers.get_mut(preset.0).unwrap() = *preset.1;
    }
    let mut inputs_idx = 0;
    for line in program {
        let dest = line[1].chars().next().unwrap();
        let operand2 = if line.len() > 2 {
            let operand = line[2];
            let val = operand
                .parse::<i64>()
                .unwrap_or_else(|_| *registers.get(&operand.chars().next().unwrap()).unwrap());
            Some(val)
        } else {
            None
        };
        match line[0] {
            "inp" => {
                println!("Input: ");
                let val: i64 = if inputs_idx < inputs.len() {
                    let v = inputs[inputs_idx];
                    inputs_idx += 1;
                    v
                } else {
                    read!()
                };
                *registers.get_mut(&dest).unwrap() = val;
            }
            "add" => {
                *registers.get_mut(&dest).unwrap() += operand2.unwrap();
            }
            "mul" => {
                *registers.get_mut(&dest).unwrap() *= operand2.unwrap();
            }
            "div" => {
                *registers.get_mut(&dest).unwrap() /= operand2.unwrap();
            }
            "mod" => {
                *registers.get_mut(&dest).unwrap() %= operand2.unwrap();
            }
            "eql" => {
                *registers.get_mut(&dest).unwrap() =
                    if operand2.unwrap() == *registers.get(&dest).unwrap() {
                        1
                    } else {
                        0
                    };
            }
            _ => panic!(),
        }
        // println!("op: {:?}", line);
        // print_registers(&registers);
    }
    print_registers(&registers);
    registers[&'z']
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct RealProgram {
    c1: i64,
    c2: i64,
    div: i64,
}

impl RealProgram {
    fn process(&self, w: i64, zin: i64) -> (i64, i64) {
        let z1 = zin / self.div;
        let x_comp = zin % 26 + self.c1;
        let x_comp_result = if x_comp == w { 0 } else { 1 };
        let y1 = 25 * x_comp_result + 1;
        let z2 = y1 * z1;
        let y2 = (w + self.c2) * x_comp_result;
        let zout = z2 + y2;
        (x_comp_result, zout)
    }
}

lazy_static! {
    #[rustfmt::skip]
    static ref PROGRAM: [RealProgram; 14] = [
        RealProgram{c1: 11, c2: 1, div: 1},
        RealProgram{c1: 11, c2: 11, div: 1},
        RealProgram{c1: 14, c2: 1, div: 1},
        RealProgram{c1: 11, c2: 11, div: 1},
        RealProgram{c1: -8, c2: 2, div: 26},
        RealProgram{c1: -5, c2: 9, div: 26},
        RealProgram{c1: 11, c2: 7, div: 1},
        RealProgram{c1: -13, c2: 11, div: 26},
        RealProgram{c1: 12, c2: 6, div: 1},
        RealProgram{c1: -1, c2: 15, div: 26},
        RealProgram{c1: 14, c2: 7, div: 1},
        RealProgram{c1: -5, c2: 1, div: 26},
        RealProgram{c1: -4, c2: 8, div: 26},
        RealProgram{c1: -8, c2: 6, div: 26},
    ];
}

fn to_number(v: &[i64]) -> i64 {
    let mut n = 0;
    for ni in v {
        n *= 10;
        n += ni;
    }
    n
}

fn p1() -> i64 {
    let mut solved = false;
    let mut digits = vec![9];
    while !solved {
        // println!("Digits: {:?}", digits);
        let mut z = 0;
        let mut last_x_comp = 0;
        for idx in 0..digits.len() {
            let w = digits[idx];
            let res = PROGRAM[idx].process(w, z);
            z = res.1;
            last_x_comp = res.0;
        }
        if PROGRAM[digits.len() - 1].div == 26 && last_x_comp == 1 {
            // Need to backtrack!
            let mut last_dig = digits.pop().unwrap();
            while last_dig == 1 {
                last_dig = digits.pop().unwrap();
            }
            digits.push(last_dig - 1);
        } else if z == 0 && digits.len() == 14 {
            solved = true;
        } else {
            // iterate
            digits.push(9);
        }
    }
    println!("Digits: {:?}", digits);
    to_number(&digits)
}

fn p2() -> i64 {
    let mut solved = false;
    let mut digits = vec![1];
    while !solved {
        // println!("Digits: {:?}", digits);
        let mut z = 0;
        let mut last_x_comp = 0;
        for idx in 0..digits.len() {
            let w = digits[idx];
            let res = PROGRAM[idx].process(w, z);
            z = res.1;
            last_x_comp = res.0;
        }
        if PROGRAM[digits.len() - 1].div == 26 && last_x_comp == 1 {
            // Need to backtrack!
            let mut last_dig = digits.pop().unwrap();
            while last_dig == 9 {
                last_dig = digits.pop().unwrap();
            }
            digits.push(last_dig + 1);
        } else if z == 0 && digits.len() == 14 {
            solved = true;
        } else {
            // iterate
            digits.push(1);
        }
    }
    println!("Digits: {:?}", digits);
    to_number(&digits)
}

fn main() {
    // let input = common::read_file("d24.txt");
    println!("P1: {:?}", p1());
    println!("P2: {:?}", p2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program() {
        let mut z = 0;
        let inputs = [1, 3, 5, 7, 9, 2, 4, 6, 8, 9, 9, 9, 9, 9];
        for idx in 0..14 {
            let program = &PROGRAM[idx];
            let w = inputs[idx];
            z = program.process(w, z).1;
        }
        assert_eq!(z, 30273774);
    }

    #[test]
    fn test_p1() {
        let inputs = [9, 2, 9, 6, 9, 5, 9, 3, 4, 9, 7, 9, 9, 2];
        let mut z = 0;
        for idx in 0..14 {
            let program = &PROGRAM[idx];
            let w = inputs[idx];
            z = program.process(w, z).1;
        }
        assert_eq!(z, 0);
        assert_eq!(p1(), to_number(&inputs));
    }

    #[test]
    fn test_p2() {
        let inputs = [8, 1, 5, 1, 4, 1, 7, 1, 1, 6, 1, 3, 8, 1];
        let mut z = 0;
        for idx in 0..14 {
            let program = &PROGRAM[idx];
            let w = inputs[idx];
            z = program.process(w, z).1;
        }
        assert_eq!(z, 0);
        assert_eq!(p2(), to_number(&inputs));
    }
}
