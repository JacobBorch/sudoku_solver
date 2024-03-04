pub mod grid;
pub mod suduko;

pub type Grid = Vec<Vec<u8>>;

pub trait Solver {
    fn solve(&mut self) -> Option<Grid>;
}