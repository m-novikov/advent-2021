pub trait Solver<T: Iterator<Item = String>> {
    fn solver(&self, input: T) -> i64;
}

pub struct SolverRegistry<'a, T: Iterator<Item = String>> {
    solvers: [Option<&'a dyn Solver<T>>; 50],
}

impl<'a, T: Iterator<Item = String>> SolverRegistry<'a, T> {
    pub fn new() -> SolverRegistry<'a, T> {
        SolverRegistry::<T> {
            solvers: [None; 50],
        }
    }

    fn get_index(day: usize, part: usize) -> usize {
        if day > 25 || day < 1 {
            panic!("Day should be in range [1; 25]");
        }
        if part != 1 && part != 2 {
            panic!("Part should be either 1 or 2");
        }
        (day - 1) * 2 + part - 1
    }

    // Register a solver for the day by putting it into the lookup table
    pub fn register(&mut self, day: usize, part: usize, solver: &'a dyn Solver<T>) {
        if self.solvers[Self::get_index(day, part)].is_some() {
            panic!("Solver for day {} part {} already registered!", day, part);
        }
        self.solvers[Self::get_index(day, part)] = Some(solver);
    }

    // Get a solver for day/part
    pub fn get(&self, day: usize, part: usize) -> &Option<&dyn Solver<T>> {
        &self.solvers[Self::get_index(day, part)]
    }
}
