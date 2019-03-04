//! Generation representation.

use crate::cell::{Cell, SimpleCell};

/// Represents a cell's generation. Generation is a 2D grid of Cells.
#[derive(Debug, PartialEq)]
pub struct Generation<T>
where
    T: Cell,
{
    width: u64,
    height: u64,
    cells: Box<[T]>,
}

// Represents a Generation made of SimpleCells.
pub type SimpleGeneration = Generation<SimpleCell>;

impl<T> Generation<T>
where
    T: Cell,
{
    /// Creates a new generation.
    pub fn new(width: u64, height: u64) -> Result<Self, &'static str> {
        if width < 3 || height < 3 {
            return Err("Generation's width and height must be equal or greater than 3.");
        }

        let size = (width * height) as usize;
        Ok(Generation {
            width,
            height,
            cells: vec![Default::default(); size].into_boxed_slice(),
        })
    }

    /// Gets width value.
    pub fn width(&self) -> u64 {
        self.width
    }

    /// Gets height value.
    pub fn height(&self) -> u64 {
        self.height
    }

    /// Retrieves a &Cell from Generation's (x, y) position.
    pub fn cell(&self, x: u64, y: u64) -> Option<&T> {
        let position = y * self.width + x;
        if position > self.width * self.height {
            return None;
        }
        Some(&self.cells[position as usize])
    }

    /// Retrieves a mutable &Cell from Generation's (x, y) position.
    pub fn cell_mut(&mut self, x: u64, y: u64) -> Option<&mut T> {
        let position = y * self.width + x;
        if position > self.width * self.height {
            return None;
        }
        Some(&mut self.cells[position as usize])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_generation() {
        let width: u64 = 5;
        let height: u64 = 5;
        let size = (width * height) as usize;

        let expected = SimpleGeneration {
            width,
            height,
            cells: vec![Default::default(); size].into_boxed_slice(),
        };

        let generation = Generation::new(width, height);

        assert_eq!(generation.unwrap(), expected);
    }

    #[test]
    fn test_new_generation_minimum_sizes() {
        let expected = "Generation's width and height must be equal or greater than 3.";

        let width_error = SimpleGeneration::new(2, 3);
        let height_error = SimpleGeneration::new(3, 2);

        assert_eq!(width_error.unwrap_err(), expected);
        assert_eq!(height_error.unwrap_err(), expected);
    }

    #[test]
    fn test_generation_width() {
        let generation = SimpleGeneration::new(3, 3).unwrap();

        assert_eq!(generation.width(), 3);
    }

    #[test]
    fn test_generation_height() {
        let generation = SimpleGeneration::new(3, 3).unwrap();

        assert_eq!(generation.height(), 3);
    }

    #[test]
    fn test_generation_cell() {
        let generation = SimpleGeneration::new(3, 3).unwrap();

        let cell = generation.cell(0, 0);
        assert_eq!(cell.is_some(), true);
        assert_eq!(cell.unwrap().is_alive(), false);

        let none = generation.cell(3, 3);
        assert_eq!(none.is_none(), true);
    }

    #[test]
    fn test_generation_cell_mut() {
        let mut generation = SimpleGeneration::new(3, 3).unwrap();

        let cell = generation.cell_mut(1, 1);
        assert_eq!(cell.is_some(), true);
        let unwrapped = cell.unwrap();
        assert_eq!(unwrapped.is_alive(), false);
        unwrapped.spawn();
        assert_eq!(unwrapped.is_alive(), true);
    }
}
