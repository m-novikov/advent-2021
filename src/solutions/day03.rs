use super::solver::Solver;

pub struct O2CO2Stats {}
pub struct GammaEpsilonRate {}

impl<T: Iterator<Item = String>> Solver<T> for GammaEpsilonRate {
    fn solver(&self, input: T) -> i64 {
        gamma_epsilon_rate(input)
    }
}

impl<T: Iterator<Item = String>> Solver<T> for O2CO2Stats {
    fn solver(&self, input: T) -> i64 {
        o2_co2_rate(input)
    }
}

fn gamma_epsilon_rate<T: Iterator<Item = String>>(mut input: T) -> i64 {
    let parse_binary = |c| match c {
        '1' => 1,
        '0' => 0,
        _ => panic!("Invalid character"),
    };
    let mut acc: Vec<u64> = input
        .next()
        .unwrap()
        .trim()
        .chars()
        .map(parse_binary)
        .collect();
    let total = input
        .map(|row| {
            row.trim()
                .chars()
                .map(parse_binary)
                .zip(acc.iter_mut())
                .for_each(|(val, dst)| {
                    *dst += val;
                });
        })
        .count() as u64;

    let mut gamma = 0;
    let mut epsilon = 0;
    for (pow, freq) in acc.iter().rev().enumerate() {
        if *freq > total / 2 {
            gamma += u64::pow(2, pow as u32);
        } else {
            epsilon += u64::pow(2, pow as u32);
        }
    }

    (gamma * epsilon) as i64
}

type CompareFn = fn(u32, u32) -> bool;

fn left_ptr(cur: usize) -> usize {
    2 * cur + 1
}

fn right_ptr(cur: usize) -> usize {
    2 * cur + 2
}

fn compute_metric(tree: &Vec<u32>, depth: usize, compare_fn: CompareFn) -> i64 {
    let mut cur = 0;
    let mut result = 0;

    for idx in 0..depth {
        let left_val = tree[left_ptr(cur)];
        let right_val = tree[right_ptr(cur)];

        if left_val != 0 && compare_fn(left_val, right_val) || right_val == 0 {
            cur = left_ptr(cur);
        } else {
            cur = right_ptr(cur);
            result += i64::pow(2, (depth - idx - 1) as u32);
        }
    }

    result
}

fn o2_co2_rate<T: Iterator<Item = String>>(input: T) -> i64 {
    let mut iter = input.peekable();
    let num_size = iter.peek().unwrap().trim().len();
    let mut tree = vec![0; usize::pow(2, (num_size + 1) as u32) - 1];
    let left_ptr = |cur| 2 * cur + 1;
    let right_ptr = |cur| 2 * cur + 2;

    for code in iter {
        let mut cur = 0;
        tree[cur] += 1;

        for ch in code.trim().chars() {
            match ch {
                '0' => cur = left_ptr(cur),
                '1' => cur = right_ptr(cur),
                _ => panic!("Invalid character"),
            }
            tree[cur] += 1;
        }
    }

    let o2_val = compute_metric(&tree, num_size, |lv, rv| lv > rv);
    let co2_val = compute_metric(&tree, num_size, |lv, rv| rv >= lv);

    o2_val * co2_val
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day3_gamma_epsilon_rate() {
        let input = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];
        assert_eq!(
            gamma_epsilon_rate(input.iter().map(|s| String::from(*s))),
            198
        );
    }

    #[test]
    fn day3_o2_co2_rate() {
        let input = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];
        assert_eq!(o2_co2_rate(input.iter().map(|s| String::from(*s))), 230);
    }
}
