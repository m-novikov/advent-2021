use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

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

fn count_increases(input: impl Iterator<Item = String>, window_size: usize) -> u64 {
    let mut cur: VecDeque<i64> = VecDeque::with_capacity(window_size + 1);
    let mut result = 0;

    for value in input.map(|s| s.parse::<i64>().unwrap()) {
        cur.push_back(value);
        if cur.len() == window_size + 1 {
            let cur_total = cur.range(1..).sum::<i64>();
            let prev_total = cur.range(..cur.len() - 1).sum::<i64>();
            if cur_total > prev_total {
                result += 1;
            }
            cur.pop_front();
        }
    }

    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let provider = FileInputProvider::new(filename);
    let increases: u64 = count_increases(provider, 3);
    println!("{}", increases);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1_example_input_size_1() {
        let input = vec![
            "199", "200", "208", "210", "200", "207", "240", "269", "260", "263",
        ];
        assert_eq!(
            count_increases(input.iter().map(|s| String::from(*s)), 1),
            7
        )
    }

    #[test]
    fn day1_example_input_size_3() {
        let input = vec![
            "199", "200", "208", "210", "200", "207", "240", "269", "260", "263",
        ];
        assert_eq!(
            count_increases(input.iter().map(|s| String::from(*s)), 3),
            5
        )
    }
}
