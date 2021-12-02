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

fn count_increases(input: impl Iterator<Item = String>, window_size: usize) -> i64 {
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

fn parse_cmd(cmd: &str) -> (i64, i64) {
    let parts = cmd.split_whitespace().take(2).collect::<Vec<&str>>();
    if let [cmd_name, change] = &parts[..] {
        let change_val = change.parse::<i64>().unwrap();
        return match *cmd_name {
            "forward" => (change_val, 0),
            "down" => (0, change_val),
            "up" => (0, -change_val),
            _ => panic!("Unknown command"),
        };
    }
    panic!("Invalid command")
}

fn pilot(input: impl Iterator<Item = String>) -> i64 {
    let mut horizonal_pos = 0;
    let mut depth = 0;
    let mut aim = 0;

    for cmd in input {
        let (hor_diff, aim_diff) = parse_cmd(&cmd);
        aim += aim_diff;
        horizonal_pos += hor_diff;
        depth += hor_diff * aim;
    }

    horizonal_pos * depth
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let provider = FileInputProvider::new(filename);
    let result = pilot(provider);
    println!("{}", result);
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

    #[test]
    fn day2_example_input() {
        let input = vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ];
        assert_eq!(pilot(input.iter().map(|s| String::from(*s))), 900)
    }

    #[test]
    fn day2_parse_cmd() {
        let input = vec![
            ("forward 5", (5, 0)),
            ("down 5", (0, 5)),
            ("forward 8", (8, 0)),
            ("up 3", (0, -3)),
            ("down 8", (0, 8)),
            ("forward 2", (2, 0)),
        ];
        for (cmd, expect) in input.iter() {
            assert_eq!(parse_cmd(cmd), *expect);
        }
    }
}
