use crate::{grid::{GridLoader, TextFileReader}, Grid};

pub struct Solver {
    grid: Grid
}

impl Solver {
    pub fn new(grid: Grid) -> Self {
        Self { grid }
    }

    pub fn solve(&self) -> Option<Grid> {
        Self::aux_solver(self.grid.clone())
    }

    fn aux_solver(mut grid: Grid) -> Option<Grid> {
        if !Self::is_valid(&grid) {
            return None
        }
        let next_entry = Self::next_entry_to_change(&grid);
        if next_entry.is_none() {
            return Some(grid)
        }
        let (i, j) = next_entry.unwrap();
        for v in 1..=9 {
            grid[i][j] = v;
            let res = Self::aux_solver(grid.clone());
            if res.is_some() { return Some(grid)}
        }
        None
    }

    fn is_valid(grid: &Grid) -> bool {
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
    let grid = TextFileReader::load_grid("grid.txt").unwrap();
    let solver = Solver::new(grid);
    let next_entry = solver.next_entry_to_change();
    assert_eq!(next_entry, Some((0, 1)))
}