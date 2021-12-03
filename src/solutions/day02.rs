use super::solver::Solver;

pub struct DaySolver;

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

impl<T: Iterator<Item = String>> Solver<T> for DaySolver {
    fn solver(&self, input: T) -> i64 {
        pilot(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
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
    fn test_parse_command() {
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
