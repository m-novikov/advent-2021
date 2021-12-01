use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut prev_value: Option<i64> = None;
    let mut increases: u64 = 0;

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(cur) = line {
                let value = cur.parse::<i64>().unwrap();
                if let Some(x) = prev_value {
                    if value > x {
                        increases += 1;
                    }
                }
                prev_value = Some(value);
            }
        }
    }
    println!("{}", increases);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
