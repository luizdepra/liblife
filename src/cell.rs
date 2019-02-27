//! Cell representations.

/// Trait that should be implemented to represent cells.
///
/// This crate provides SimpleCell as a default implementation.
pub trait Cell: Default {
    fn is_alive(&self) -> bool;
    fn spawn(&mut self);
    fn kill(&mut self);
}

/// Represents the state of a SimpleCell.
#[derive(Debug, PartialEq)]
enum State {
    Alive,
    Dead,
}

/// Default representation of a cell.
#[derive(Debug, PartialEq)]
pub struct SimpleCell {
    state: State,
}

impl SimpleCell {
    /// Crate a new SimpleCell alive or dead.
    pub fn new(alive: bool) -> Self {
        let state = if alive { State::Alive } else { State::Dead };
        SimpleCell { state }
    }
}

impl Cell for SimpleCell {
    /// Returns if the SimpleCell is alive or not.
    fn is_alive(&self) -> bool {
        self.state == State::Alive
    }

    /// Turn the SimpleCell alive.
    fn spawn(&mut self) {
        self.state = State::Alive;
    }

    /// Turn the SimpleCell dead.
    fn kill(&mut self) {
        self.state = State::Dead;
    }
}

impl Default for SimpleCell {
    fn default() -> Self {
        SimpleCell { state: State::Dead }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simplecell_default() {
        let expected = SimpleCell { state: State::Dead };

        let result: SimpleCell = Default::default();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_simplecell_is_alive() {
        let alive_cell = SimpleCell {
            state: State::Alive,
        };
        let dead_cell = SimpleCell { state: State::Dead };

        assert!(alive_cell.is_alive());
        assert!(!dead_cell.is_alive());
    }
}
