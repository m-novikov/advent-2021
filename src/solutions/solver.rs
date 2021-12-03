pub trait Solver<T: Iterator<Item = String>> {
    fn solver(&self, input: T) -> i64;
}

pub struct SolverRegistry<'a, T: Iterator<Item = String>> {
    solvers: Vec<Option<&'a dyn Solver<T>>>,
}

impl<'a, T: Iterator<Item = String>> SolverRegistry<'a, T> {
    pub fn new() -> SolverRegistry<'a, T> {
        SolverRegistry::<T> {
            solvers: (0..50).map(|_| None).collect(),
        }
    }

    fn get_index(day: usize, part: usize) -> usize {
        if (day > 25 || day < 1) {
            panic!("Day should be in range [1; 25]");
        }
        if (part != 1 && part != 2) {
            panic!("Part should be either 1 or 2");
        }
        (day - 1) * 2 + part - 1
    }

    pub fn register(&mut self, day: usize, part: usize, solver: &'a dyn Solver<T>) {
        self.solvers[Self::get_index(day, part)] = Some(solver);
    }

    pub fn get(&self, day: usize, part: usize) -> &Option<&dyn Solver<T>> {
        &self.solvers[Self::get_index(day, part)]
    }
}
