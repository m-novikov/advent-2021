use super::solver::Solver;
use std::collections::VecDeque;

pub struct DaySolver {
    pub window_size: usize,
}

impl<T: Iterator<Item = String>> Solver<T> for DaySolver {
    fn solver(&self, input: T) -> i64 {
        count_increases(input, self.window_size)
    }
}

fn count_increases<T: Iterator<Item = String>>(input: T, window_size: usize) -> i64 {
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
