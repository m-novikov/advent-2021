use regex::Regex;
use solutions::*;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

pub mod solutions;

struct FileInputProvider {
    lines: io::Lines<io::BufReader<File>>,
}

impl FileInputProvider {
    fn new(filename: &String) -> FileInputProvider {
        let file = File::open(filename).expect("Error opening file");
        FileInputProvider {
            lines: io::BufReader::new(file).lines(),
        }
    }
}

impl Iterator for FileInputProvider {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.lines.next();
        match value {
            None => None,
            Some(x) => Some(x.expect("Failed to read line")),
        }
    }
}

fn main() {
    let mut solver_registry = solver::SolverRegistry::<FileInputProvider>::new();
    solver_registry.register(1, 1, &day01::DaySolver { window_size: 1 });
    solver_registry.register(1, 2, &day01::DaySolver { window_size: 3 });
    solver_registry.register(2, 2, &day02::DaySolver {});
    solver_registry.register(3, 1, &day03::GammaEpsilonRate {});
    solver_registry.register(3, 2, &day03::O2CO2Stats {});

    let re_day = Regex::new(r"day0*(?P<day>\d+)").unwrap();
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let part = if args.len() == 3 {
        args[2].parse::<usize>().unwrap()
    } else {
        2
    };

    let captures = re_day.captures(filename).unwrap();
    let day = &captures["day"].parse::<usize>().unwrap();

    println!("Day {} part {}", day, part);
    let provider = FileInputProvider::new(filename);
    let result = solver_registry
        .get(*day, part)
        .as_ref()
        .expect("No solver")
        .solver(provider);
    println!("{}", result);
}
