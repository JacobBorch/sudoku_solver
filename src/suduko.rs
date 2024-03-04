use std::collections::HashSet;

use crate::{
    grid::{GridError, GridLoader, TextFileReader},
    Grid, Solver,
};

pub struct Bruteforce {
    pub original_grid: Grid,
    grid: Grid,
}

impl Solver for Bruteforce {
    fn solve(&mut self) -> Option<Grid> {
        self.aux_solver()
    }
}

impl Bruteforce {
    pub fn from_file(file_path: &str) -> Result<Self, GridError> {
        let grid = TextFileReader::load_grid(file_path)?;
        Ok(Self::new(grid))
    }

    pub fn new(grid: Grid) -> Self {
        Self {
            original_grid: grid.clone(),
            grid,
        }
    }

    /*Takes an entry (i,j) and value v, and returns a bool whether 
    not a this value is able to be inserted */
    fn can_insert(&self, v: u8, (i, j): (usize, usize)) -> bool {
        self.grid[i].iter().all(|&x| x != v) &&
        self.col(j).iter().all(|&x| x != v) &&
        self.square(((i/3)*3, (j / 3)*3)).iter().all(|&x| x != v)
    }

    /* Returns the subsquare of the sudoku. Needs to be called with the value
    of the upper left of the square.
     */
    fn square(&self, (x, y): (usize, usize)) -> Vec<u8> {
        let mut square = Vec::new();
        for dx in 0..3 {
            for dy in 0..3 {
                let entry = self.grid[x + dx][y + dy];
                square.push(entry)
            }
        }
        square
    }

    fn col(&self, j: usize) -> Vec<u8> {
        self.grid.iter().map(|row| row[j]).collect()
    }

    fn aux_solver(&mut self) -> Option<Grid> {
        let next_entry = self.next_entry_to_change();
        if next_entry.is_none() {
            return Some(self.grid.clone());
        }
        let (i, j) = next_entry.unwrap();
        for v in 1..=9 {
            if !self.can_insert(v, (i, j)) {
                continue;
            }
            self.grid[i][j] = v;
            let res = self.aux_solver();
            if res.is_some() {
                return res;
            }
            self.grid[i][j] = 0;
        }
        None
    }

    //Returns true if the row has any duplicate values (other than zero)
    fn has_duplicate(row: &Vec<u8>) -> bool {
        row.iter()
            .filter(|&&x| x != 0)
            .collect::<HashSet<_>>()
            .len()
            != row.iter().filter(|&&x| x != 0).collect::<Vec<_>>().len()
    }

    fn next_entry_to_change(&self) -> Option<(usize, usize)> {
        self.grid
            .iter()
            .enumerate()
            .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, &v)| (i, j, v)))
            .find(|(_, _, v)| *v == 0)
            .map(|(i, j, _)| (i, j))
    }

     fn is_valid(&self) -> bool {
        self.grid.iter().all(|row| !Self::has_duplicate(row))
            && (0..9)
                .map(|j| self.col(j))
                .all(|col| !Self::has_duplicate(&col))
            && (0..9)
                .map(|j| self.square(((j / 3) * 3, (j % 3) * 3 )))
                .all(|square| !Self::has_duplicate(&square))
    }
}

lazy_static::lazy_static! {
    static ref GRID: Bruteforce = Bruteforce::from_file("test_grids/grid.txt").unwrap();
    static ref INVALID_ROW_GRID: Bruteforce = Bruteforce::from_file("test_grids/invalid_row.txt").unwrap();
    static ref INVALID_COLUMN_GRID: Bruteforce = Bruteforce::from_file("test_grids/invalid_column.txt").unwrap();
    static ref INVALID_SQUARE_GRID: Bruteforce = Bruteforce::from_file("test_grids/invalid_square.txt").unwrap();
}

#[test]
fn test_next_entry() {
    let next_entry = Bruteforce::next_entry_to_change(&GRID);
    assert_eq!(next_entry, Some((0, 1)))
}

#[test]
fn has_duplicate_with_duplicate() {
    let row = vec![0, 1, 2, 0, 1, 0, 0, 0, 0];
    let has_duplicate = Bruteforce::has_duplicate(&row);
    assert!(has_duplicate)
}

#[test]
fn has_duplicate_with_no() {
    let row = vec![0, 1, 2, 0, 3, 0, 5, 0, 8];
    let has_duplicate = Bruteforce::has_duplicate(&row);
    assert!(!has_duplicate)
}

#[test]
fn can_insert() {
    let can_insert = Bruteforce::can_insert(&GRID, 1, (1, 4));
    assert!(can_insert)
}

#[test]
fn cannot_insert_duplicate_on_row() {
    let can_insert = Bruteforce::can_insert(&INVALID_ROW_GRID, 1, (0, 5));
    assert!(!can_insert)
}

#[test]
fn cannot_insert_duplicate_on_column() {
    let can_insert = Bruteforce::can_insert(&INVALID_COLUMN_GRID, 5, (7, 4));
    assert!(!can_insert)
}

#[test]
fn cannot_insert_duplicate_on_square() {
    let can_insert = Bruteforce::can_insert(&INVALID_SQUARE_GRID, 9, (6, 6));
    assert!(!can_insert)
} 

#[test]
fn extract_col_works() {
    let seventh_col = Bruteforce::col(&GRID, 6);
    let expected = vec![0, 3, 0, 0, 0, 0, 0, 0, 0];
    assert_eq!(seventh_col, expected)
}

#[test]
fn extract_square_works() {
    let sixth_square = Bruteforce::square(&INVALID_SQUARE_GRID, (3, 6));
    let expected = vec![1, 0, 0, 0, 0, 0, 0, 0, 1];
    assert_eq!(sixth_square, expected)
}

#[test]
fn test_solve() {
    let mut solver = Bruteforce::from_file("test_grids/solvable.txt").unwrap();
    let solution = solver.solve();
    assert!(solver.next_entry_to_change().is_none());
    assert!(solver.is_valid())
}
