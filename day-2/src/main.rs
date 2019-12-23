use gnuplot::AxesCommon;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::process;
use std::result::Result;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args.get(1).expect("Expected a filename as an arg");
    let file = File::open(filename).expect("Unable to open file");
    let line: String = BufReader::new(file)
        .lines()
        .next()
        .expect("File needs at least one line")
        .expect("Error reading line from file");
    let (intcode, errs): (Vec<_>, Vec<_>) = line
        .split(',')
        .map(|num| num.parse::<usize>())
        .partition(Result::is_ok);
    if !errs.is_empty() {
        for error in errs.into_iter().filter_map(Result::err) {
            eprintln!("{}", error);
        }
        process::exit(1);
    }
    let intcode: Vec<usize> = intcode.into_iter().filter_map(Result::ok).collect();
    println!("Part 1: {:?}", part_1(&intcode, 12, 2));
    println!("Part 2: {:?}", part_2(&intcode, 19690720));
    // println!("Part 1: Total fuel needed: {}", part_1(&module_sizes));
    // println!("Part 2: Total fuel needed: {}", part_2(&module_sizes));
}

#[derive(Clone, Debug)]
struct IntcodeState {
    intcode: Vec<usize>,
    pointer: usize,
}

impl IntcodeState {
    fn new(intcode: Vec<usize>) -> IntcodeState {
        IntcodeState {
            intcode: intcode,
            pointer: 0,
        }
    }
}

#[derive(Debug)]
struct IntcodeStateIterator {
    intcode_state: IntcodeState,
    error: bool,
}

impl IntcodeStateIterator {
    fn new(starting_state: Vec<usize>) -> IntcodeStateIterator {
        IntcodeStateIterator {
            intcode_state: IntcodeState::new(starting_state),
            error: false,
        }
    }
}

impl Iterator for IntcodeStateIterator {
    type Item = Result<IntcodeState, String>;

    fn next(&mut self) -> Option<Self::Item> {
        // After error, stop processing
        if self.error {
            return None;
        }

        let intcode = &mut self.intcode_state.intcode;
        let pointer = self.intcode_state.pointer;
        let opcode = intcode[pointer];

        match opcode {
            1 => {
                // ADD
                let a = intcode[intcode[pointer + 1]];
                let b = intcode[intcode[pointer + 2]];
                let c = intcode[pointer + 3];
                intcode[c] = a + b;
            }
            2 => {
                // MULT
                let a = intcode[intcode[pointer + 1]];
                let b = intcode[intcode[pointer + 2]];
                let c = intcode[pointer + 3];
                intcode[c] = a * b;
            }
            99 => {
                // END
                return None;
            }
            _ => {
                // UNKNOWN
                self.error = true;
                return Some(Err("Unknown Opcode".to_string()));
            }
        }
        self.intcode_state.pointer += 4;
        Some(Ok(self.intcode_state.clone()))
    }
}

fn part_1(starting_state: &Vec<usize>, arg1: usize, arg2: usize) -> Result<usize, String> {
    let mut new_starting_values = starting_state.clone();
    new_starting_values[1] = arg1;
    new_starting_values[2] = arg2;
    let it = IntcodeStateIterator::new(new_starting_values);
    match it.last() {
        Some(Err(x)) => Err(x),
        Some(Ok(x)) => Ok(x.intcode[0]),
        None => unimplemented!(),
    }
}

fn graph(starting_state: &Vec<usize>) {
    let mut z_vals = Vec::new();
    const X_DIM: usize = 10;
    const Y_DIM: usize = 10;

    for y in 0..X_DIM {
        for x in 0..Y_DIM {
            dbg!((x, y));
            z_vals.push(dbg!(part_1(starting_state, x, y).unwrap_or(0) as f32));
        }
    }
    let mut fg = gnuplot::Figure::new();
    fg.axes3d()
        .surface(&z_vals, X_DIM, Y_DIM, None, &[])
        .set_x_label("x", &[])
        .set_y_label("y", &[]);
    fg.show().unwrap();
}

fn part_2(starting_state: &Vec<usize>, target: usize) -> Result<(usize, usize), String> {
    // Assumption based on graph:
    // result is linear ie: = a + b * x + c * y
    let base = part_1(starting_state, 0, 0)?;
    let x_grad = part_1(starting_state, 1, 0)? - base;
    let y_grad = part_1(starting_state, 0, 1)? - base;
    let diff = target - base;
    let x = diff / x_grad;
    let y = diff - (x * x_grad) / y_grad;
    // Assert to ensure assumption was true
    assert_eq!(part_1(starting_state, x, y)?, target);
    Ok((x, y))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(
            IntcodeStateIterator::new(vec![1, 0, 0, 0, 99])
                .last()
                .unwrap()
                .unwrap()
                .intcode,
            vec![2, 0, 0, 0, 99]
        );
        assert_eq!(
            IntcodeStateIterator::new(vec![2, 3, 0, 3, 99])
                .last()
                .unwrap()
                .unwrap()
                .intcode,
            vec![2, 3, 0, 6, 99]
        );
        assert_eq!(
            IntcodeStateIterator::new(vec![2, 4, 4, 5, 99, 0])
                .last()
                .unwrap()
                .unwrap()
                .intcode,
            vec![2, 4, 4, 5, 99, 9801]
        );
        assert_eq!(
            IntcodeStateIterator::new(vec![1, 1, 1, 4, 99, 5, 6, 0, 99])
                .last()
                .unwrap()
                .unwrap()
                .intcode,
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
        );

        // assert_eq!(part_2(&vec![14]), 2);
        // assert_eq!(part_2(&vec![1969]), 966);
        // assert_eq!(part_2(&vec![100756]), 50346);
    }
}
