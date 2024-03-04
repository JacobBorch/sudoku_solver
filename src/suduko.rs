use std::collections::HashSet;

use crate::{
    grid::{GridLoader, TextFileReader},
    Grid,
};

pub struct Solver {
    grid: Grid,
}

impl Solver {
    pub fn new(grid: Grid) -> Self {
        Self { grid }
    }

    pub fn solve(&self) -> Option<Grid> {
        Self::aux_solver(self.grid.clone())
    }

    fn extract_square(grid: &Grid, no: usize) -> Vec<u8> {
        let mut square = Vec::new();
        let col = no % 3;
        let row = no / 3;
        let (x, y) = (row * 3, col * 3);
        for dx in 0..3 {
            for dy in 0..3 {
                let entry = grid[x + dx][y + dy];
                square.push(entry)
            }
        }
        square
    }

    fn extract_col(grid: &Grid, j: usize) -> Vec<u8> {
        grid.iter().map(|row| row[j]).collect()
    }

    fn aux_solver(mut grid: Grid) -> Option<Grid> {
        if !Self::is_valid(&grid) {
            return None;
        }
        let next_entry = Self::next_entry_to_change(&grid);
        if next_entry.is_none() {
            return Some(grid);
        }
        let (i, j) = next_entry.unwrap();
        for v in 1..=9 {
            grid[i][j] = v;
            let res = Self::aux_solver(grid.clone());
            if res.is_some() {
                return res;
            }
        }
        None
    }
    //Returns true if the row has any duplicate values (other than zero)
    fn has_duplicate(row: &Vec<u8>) -> bool {
        let it = row.iter().filter(|&&x| x != 0);
        it.clone().collect::<HashSet<_>>().len() != it.collect::<Vec<_>>().len()
    }

    fn is_valid(grid: &Grid) -> bool {
        grid.iter().all(|row| !Solver::has_duplicate(row))
            && (0..9)
                .map(|j| Self::extract_col(grid, j))
                .all(|col| !Solver::has_duplicate(&col))
            && (0..9)
                .map(|j| Self::extract_square(grid, j))
                .all(|square| !Solver::has_duplicate(&square))
    }

    fn next_entry_to_change(grid: &Grid) -> Option<(usize, usize)> {
        grid.iter()
            .enumerate()
            .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, &v)| (i, j, v)))
            .find(|(_, _, v)| *v == 0)
            .map(|(i, j, _)| (i, j))
    }
}

lazy_static::lazy_static! {
    static ref GRID: Grid = TextFileReader::load_grid("test_grids/grid.txt").unwrap();
    static ref INVALID_ROW_GRID: Grid = TextFileReader::load_grid("test_grids/invalid_row.txt").unwrap();
    static ref INVALID_COLUMN_GRID: Grid = TextFileReader::load_grid("test_grids/invalid_column.txt").unwrap();
    static ref INVALID_SQUARE_GRID: Grid = TextFileReader::load_grid("test_grids/invalid_square.txt").unwrap();
    static ref SOLVABLE: Grid = TextFileReader::load_grid("test_grids/solvable.txt").unwrap();
}

#[test]
fn test_next_entry() {
    let next_entry = Solver::next_entry_to_change(&GRID);
    assert_eq!(next_entry, Some((0, 1)))
}

#[test]
fn has_duplicate_with_duplicate() {
    let row = vec![0, 1, 2, 0, 1, 0, 0, 0, 0];
    let has_duplicate = Solver::has_duplicate(&row);
    assert!(has_duplicate)
}

#[test]
fn has_duplicate_with_no() {
    let row = vec![0, 1, 2, 0, 3, 0, 5, 0, 8];
    let has_duplicate = Solver::has_duplicate(&row);
    assert!(!has_duplicate)
}

#[test]
fn is_valid_when_valid() {
    let is_valid = Solver::is_valid(&GRID);
    assert!(is_valid)
}

#[test]
fn invalid_row() {
    let is_valid = Solver::is_valid(&INVALID_ROW_GRID);
    assert!(!is_valid)
}

#[test]
fn invalid_column() {
    let is_valid = Solver::is_valid(&INVALID_COLUMN_GRID);
    assert!(!is_valid)
}

#[test]
fn invalid_square() {
    let is_valid = Solver::is_valid(&INVALID_SQUARE_GRID);
    assert!(!is_valid)
}

#[test]
fn extract_col_works() {
    let seventh_col = Solver::extract_col(&GRID, 6);
    let expected = vec![0, 3, 0, 0, 0, 0, 0, 0, 0];
    assert_eq!(seventh_col, expected)
}

#[test]
fn extract_square_works() {
    let sixth_square = Solver::extract_square(&INVALID_SQUARE_GRID, 5);
    let expected = vec![1, 0, 0, 0, 0, 0, 0, 0, 1];
    assert_eq!(sixth_square, expected)
}

#[test]
fn test_solve() {
    let solver = Solver::new(SOLVABLE.clone());
    let solution = solver.solve();
    let is_valid_solution = Solver::is_valid(&solution.unwrap());
    assert!(is_valid_solution)
}
