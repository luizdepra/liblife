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

impl Default for SimpleCell {
    fn default() -> Self {
        SimpleCell { state: State::Dead }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simplecell_new() {
        let expected_alive = SimpleCell {
            state: State::Alive,
        };
        let expected_dead = SimpleCell { state: State::Dead };

        let alive = SimpleCell::new(true);
        let dead = SimpleCell::new(false);

        assert_eq!(alive, expected_alive);
        assert_eq!(dead, expected_dead);
    }

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

    #[test]
    fn test_simplecell_spawn() {
        let mut cell = SimpleCell::new(false);

        assert_eq!(cell.is_alive(), false);
        cell.spawn();
        assert_eq!(cell.is_alive(), true);
    }

    #[test]
    fn test_simplecell_kill() {
        let mut cell = SimpleCell::new(true);

        assert_eq!(cell.is_alive(), true);
        cell.kill();
        assert_eq!(cell.is_alive(), false);
    }
}
