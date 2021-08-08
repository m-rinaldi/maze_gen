#![allow(unused)]

mod grid;

use grid::{Grid, Cell};

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_grid_creation() {
        let grid = Grid::new(4, 4);
    }
}
