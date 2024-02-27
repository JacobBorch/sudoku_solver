use crate::Grid;

pub struct Solver {
    grid: Grid
}

impl Solver {
    pub fn new(grid: Grid) -> Self {
        Self { grid }
    }

    pub fn solve(&self) -> Option<Grid> {
        let mut grid = self.grid.clone();






        todo!()
    }

    fn aux_solver(grid: &mut Grid) -> Option<Grid> {
        todo!()
    }

    fn next_entry_to_change(grid: &Grid) -> Option<(usize, usize)> {
        let a = grid.iter()
            .enumerate()
            .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, &v)| (i, j, v)))
            .find(|(_, _, v)| *v == 0);
        a.map(|(i, j, _)| (i, j))
    }

    
}

#[test]
fn test_next_entry() {

}