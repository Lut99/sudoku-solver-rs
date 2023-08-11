//  SOLVERS.rs
//    by Lut99
// 
//  Created:
//    10 Aug 2023, 23:03:23
//  Last edited:
//    11 Aug 2023, 17:01:28
//  Auto updated?
//    Yes
// 
//  Description:
//!   Implements the Sudoku solver(s).
// 

use crate::sudoku::Sudoku;


/***** TESTS *****/
#[cfg(test)]
mod tests {
    use crate::utils::{load_sudoku, PrettyError as _};
    use super::*;

    #[test]
    fn test_brute_force_solver() {
        // Test an empty Sudoku can be solved
        {
            let empty: Sudoku = load_sudoku("./tests/empty.json").unwrap_or_else(|err| panic!("Failed to load empty Sudoku: {}", err.pretty())).swap_remove(0);
            println!("\n{empty}");

            let mut solver: BruteForceSolver = BruteForceSolver::new();
            let solved: Sudoku = solver.run(empty);
            assert!(solved.is_finished());
        }

        // Test if a fixed Sudoku is solved
        {
            let correct: Sudoku = load_sudoku("./tests/correct.json").unwrap_or_else(|err| panic!("Failed to load correct Sudoku: {}", err.pretty())).swap_remove(0);
            println!("{correct}");

            let mut solver: BruteForceSolver = BruteForceSolver::new();
            let solved: Sudoku = solver.run(correct);
            assert!(solved.is_finished());
            assert_eq!(correct, solved);
        }
    }
}





/***** AUXILLARY *****/
/// Defines what all Sudoku solvers have in common.
pub trait Solver {
    /// Solves the given sudoku.
    /// 
    /// # Arguments
    /// - `sudoku`: The [`Sudoku`] to solve.
    /// 
    /// # Returns
    /// The solved [`Sudoku`], or else the best attempt.
    #[inline]
    fn run(&mut self, sudoku: Sudoku) -> Sudoku { self.run_with_callback(sudoku, |_| Ok::<bool, std::convert::Infallible>(true)).unwrap().unwrap() }

    /// Solves the given sudoku, calling the given code at the end of every step.
    /// 
    /// # Arguments
    /// - `sudoku`: The [`Sudoku`] to solve.
    /// - `callback`: The callback to run after every step taken. Returns a boolean that indicates whether to continue (true) or not (false), and is allowed to error.
    /// 
    /// # Returns
    /// The solved [`Sudoku`], or else the best attempt. However, if `callback` return false at some point, `None` is returned instead.
    fn run_with_callback<E>(&mut self, sudoku: Sudoku, callback: impl FnMut(&Sudoku) -> Result<bool, E>) -> Result<Option<Sudoku>, E>;
}





/***** LIBRARY *****/
/// Implements a dumb-but-effective, brute-force solver.
#[derive(Clone, Debug)]
pub struct BruteForceSolver {}

impl Default for BruteForceSolver {
    #[inline]
    fn default() -> Self { Self::new() }
}
impl BruteForceSolver {
    /// Constructor for the BruteForceSolver.
    /// 
    /// # Returns
    /// A new instance of Self.
    #[inline]
    pub fn new() -> Self {
        Self {}
    }
}
impl Solver for BruteForceSolver {
    fn run_with_callback<E>(&mut self, sudoku: Sudoku, mut callback: impl FnMut(&Sudoku) -> Result<bool, E>) -> Result<Option<Sudoku>, E> {
        let mut best         : (f64, Sudoku) = (sudoku.score(), sudoku);
        let mut search_space : Vec<Sudoku>   = vec![ sudoku ];
        while let Some(attempt) = search_space.pop() {
            // Discard this attempt if it is not well-formed
            if !attempt.is_well_formed() { continue; }
            // Update the best one
            if attempt.score() > best.0 { best.1 = attempt; }
            // If it's finished, we're done!
            if attempt.is_finished() { break; }

            // Run the callback
            if !callback(&attempt)? { return Ok(None); };

            // Find the first empty cell
            'empty_cell: for y in 0..9 {
                for x in 0..9 {
                    // Skip if not None
                    if attempt.rows[y][x].is_some() { continue; }

                    // Iterate over the possibilities
                    for v in 1..=9 {
                        // Check if valid
                        if !attempt.is_cell_valid(x, y, v) { continue; }

                        // Alright add the possibility
                        let mut next_attempt: Sudoku = attempt;
                        next_attempt.rows[y][x] = Some(v);
                        search_space.push(next_attempt);
                    }

                    // Always break if we found an empty cell, since we only want to consider valid solutions
                    break 'empty_cell;
                }
            }
        }

        // Return the best attempt
        Ok(Some(best.1))
    }
}
