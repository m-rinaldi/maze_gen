pub struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

#[derive(Copy, Clone, Debug)]
pub enum Cell {
    North,
    East,
    Closed,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        assert!(width > 0 && height > 0);
        let len = width.checked_mul(height).unwrap();

        Grid {
            width,
            height,
            cells: vec![Cell::Closed; len],
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

impl std::ops::Index<usize> for Grid {
    type Output = [Cell];

    fn index(&self, row_idx: usize) -> &Self::Output {
        let begin = row_idx.checked_mul(self.width).unwrap();
        let end = begin.checked_add(self.width).unwrap();
        &self.cells[begin..end]
    }
}