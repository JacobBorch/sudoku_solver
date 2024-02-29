use std::{fs::File, io::{self, BufRead}};

use crate::Grid;

#[derive(Debug)]
pub enum GridError {

}

pub trait GridLoader {
    fn load_grid(file: &str) -> Result<Grid, GridError>;
}

pub struct TextFileReader { }

impl TextFileReader {
    fn parse_line(line: &str) -> Vec<u8> {
        line.chars()
            .filter(|&c| c.is_numeric() || c == '.')
            .map(|c| if c == '.' {'0'} else {c})
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect()
    }
}

impl GridLoader for TextFileReader {
    fn load_grid(file: &str) -> Result<Grid, GridError> {
        let file = File::open(file).expect("Couldn't find file");
        let reader = io::BufReader::new(file);
        let mut grid = Vec::new();
        for line in reader.lines() {
            let row = Self::parse_line(&line.unwrap());
            if row.len() != 9 {
                continue;
            }
            grid.push(row);
        }
        assert_eq!(grid.len(), 9);
        Ok(grid)
    }
}

struct ImageReader {}

impl GridLoader for ImageReader {
    fn load_grid(file: &str) -> Result<Grid, GridError> {
        todo!()
    }
}

#[test]
fn test_text_file_reader() {
    let grid = TextFileReader::load_grid("test_grids/grid.txt");
    let mut expected_grid = vec![vec![0; 9]; 9];
    expected_grid[0][0] = 1;
    expected_grid[1][6] = 3;
    expected_grid[4][4] = 5;
    expected_grid[8][8] = 9;
    assert_eq!(grid.unwrap(), expected_grid) 
}
