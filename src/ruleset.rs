//! Generations calculation ruleset.

use itertools::iproduct;

use crate::cell::Cell;
use crate::generation::Generation;

/// Represents the type of functions needed to calculate generations.
pub type RulesetFn<T> = Fn(u64, u64, &Generation<T>) -> bool;

/// RulesetFn that applies the Conway's evolution rules.
pub fn apply_conway_ruleset<T>(x: u64, y: u64, gen: &Generation<T>) -> bool
where
    T: Cell,
{
    let anc = alive_neighbors(x, y, gen);
    if let Some(cell) = gen.cell(x, y) {
        (cell.is_alive() && anc == 2) || anc == 3
    } else {
        false
    }
}

// Count alive neighbor cells.
fn alive_neighbors<T>(x: u64, y: u64, gen: &Generation<T>) -> usize
where
    T: Cell,
{
    let x = x as i64;
    let y = y as i64;

    iproduct!((x - 1)..=(x + 1), (y - 1)..=(y + 1))
        .filter(|(ix, iy)| {
            let fixed_x = fix_coord(*ix, 0, gen.width() - 1);
            let fixed_y = fix_coord(*iy, 0, gen.height() - 1);
            let cell = gen.cell(fixed_x, fixed_y).unwrap();
            (fixed_x != x as u64 || fixed_y != y as u64) && cell.is_alive()
        })
        .count()
}

// Fix i64 relative coords to u64 absolute ones.
fn fix_coord(c: i64, lower: u64, upper: u64) -> u64 {
    if c < lower as i64 {
        upper
    } else if c > upper as i64 {
        lower
    } else {
        c as u64
    }
}

#[cfg(test)]
mod tests {
    use crate::cell::SimpleCell;

    use super::*;

    #[test]
    fn test_fix_coord() {
        let values: Vec<(i64, u64, u64, u64)> = vec![
            (1, 0, 2, 1),
            (0, 0, 2, 0),
            (2, 0, 2, 2),
            (-1, 0, 2, 2),
            (3, 0, 2, 0),
        ];

        for (c, l, u, e) in &values {
            assert_eq!(fix_coord(*c, *l, *u), *e);
        }
    }

    #[test]
    fn test_alive_neighbors() {
        let mut gen: Generation<SimpleCell> = Generation::new(3, 3).unwrap();

        gen.cell_mut(1, 1).unwrap().spawn();
        gen.cell_mut(1, 2).unwrap().spawn();
        gen.cell_mut(2, 2).unwrap().spawn();

        assert_eq!(alive_neighbors(1, 1, &gen), 2);
        assert_eq!(alive_neighbors(0, 0, &gen), 3);

        gen.cell_mut(1, 2).unwrap().kill();
        gen.cell_mut(2, 2).unwrap().kill();

        assert_eq!(alive_neighbors(1, 1, &gen), 0);
        assert_eq!(alive_neighbors(2, 2, &gen), 1);
    }

    #[test]
    fn test_apply_conway_ruleset() {
        let mut gen: Generation<SimpleCell> = Generation::new(5, 5).unwrap();

        gen.cell_mut(1, 1).unwrap().spawn();
        gen.cell_mut(1, 2).unwrap().spawn();
        gen.cell_mut(2, 2).unwrap().spawn();

        assert_eq!(apply_conway_ruleset(0, 0, &gen), false);
        assert_eq!(apply_conway_ruleset(2, 2, &gen), true);
        assert_eq!(apply_conway_ruleset(1, 2, &gen), true);
    }
}
