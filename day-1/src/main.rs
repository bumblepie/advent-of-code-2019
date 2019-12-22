use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename).expect("Unable to open file");
    let (lines, errs): (Vec<_>, Vec<_>) = BufReader::new(file).lines().partition(Result::is_ok);
    if !errs.is_empty() {
        for err in errs.into_iter().filter_map(Result::err) {
            eprintln!("{}", err);
        }
        process::exit(1);
    }
    let (module_sizes, errs): (Vec<_>, Vec<_>) = lines.into_iter()
        .map( |line_result| line_result.unwrap())
        .map( |line| line.parse::<i64>())
        .partition(Result::is_ok);
    if !errs.is_empty() {
        for err in errs.into_iter().filter_map(Result::err) {
            eprintln!("{}", err);
        }
        process::exit(1);
    }
    let module_sizes = module_sizes.into_iter()
        .map( |result| result.unwrap())
        .collect();

    println!("Part 1: Total fuel needed: {}", part_1(&module_sizes));
    println!("Part 2: Total fuel needed: {}", part_2(&module_sizes));
}

fn mass_needed(mass: i64) -> i64 {
    mass / 3 -2
}

fn part_1(module_sizes: &Vec<i64>) -> i64 {
    module_sizes.into_iter()
        .map(|size| mass_needed(*size))
        .sum()
}

fn part_2(module_sizes: &Vec<i64>) -> i64 {
    module_sizes.into_iter()
        .map(|&size| {
            let mut fuel_needed = 0;
            let mut bonus_fuel_needed = mass_needed(size);
            while bonus_fuel_needed > 0 {
                fuel_needed += bonus_fuel_needed;
                bonus_fuel_needed = mass_needed(bonus_fuel_needed);
            }
            fuel_needed
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(part_1(&vec![12]), 2);
        assert_eq!(part_1(&vec![14]), 2);
        assert_eq!(part_1(&vec![1969]), 654);
        assert_eq!(part_1(&vec![100756]), 33583);

        assert_eq!(part_2(&vec![14]), 2);
        assert_eq!(part_2(&vec![1969]), 966);
        assert_eq!(part_2(&vec![100756]), 50346);
    }
}
