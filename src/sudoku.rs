//  SUDOKU.rs
//    by Lut99
// 
//  Created:
//    11 Aug 2023, 11:42:21
//  Last edited:
//    11 Aug 2023, 15:26:21
//  Auto updated?
//    Yes
// 
//  Description:
//!   Defines a Sudoku and its behaviour.
// 

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FResult};

use console::Style;
use enum_debug::EnumDebug;
use ratatui::widgets::{Row, Table};
use serde::{Deserialize, Serialize};


/***** TESTS *****/
#[cfg(test)]
mod tests {
    use crate::utils::{load_sudoku, PrettyError as _};
    use super::*;

    #[test]
    fn test_sudoku_well_formedness() {
        let sudoku: Sudoku = load_sudoku("./tests/correct.json").unwrap_or_else(|err| panic!("Failed to load correct Sudoku: {}", err.pretty()));
        println!("\n{sudoku}");

        // Assert the correct one is well formed & finished
        assert_eq!(sudoku.score(), 1.0);
        assert_eq!(sudoku.well_formed(), Ok(()));
        assert_eq!(sudoku.finished(), Ok(()));

        // Apply some row permutation and check
        {
            let mut row_err: Sudoku = sudoku;
            row_err.rows[3][4] = Some(5);
            println!("{row_err}");
            assert_eq!(row_err.score(), 1.0);
            assert_eq!(row_err.well_formed(), Err(InvalidReason::RowConflict { cell: (5, 3), conflict: (4, 3) }));
            assert_eq!(row_err.finished(), Err(InvalidReason::RowConflict { cell: (5, 3), conflict: (4, 3) }));
        }

        // Apply some column permutation and check
        {
            let mut col_err: Sudoku = sudoku;
            col_err.rows[4][5] = Some(5);
            println!("{col_err}");
            assert_eq!(col_err.score(), 1.0);
            assert_eq!(col_err.well_formed(), Err(InvalidReason::ColConflict { cell: (5, 4), conflict: (5, 3) }));
            assert_eq!(col_err.finished(), Err(InvalidReason::ColConflict { cell: (5, 4), conflict: (5, 3) }));
        }

        // Apply some box permutation and check
        {
            let mut box_err: Sudoku = sudoku;
            box_err.rows[2][5] = Some(5);
            println!("{box_err}");
            assert_eq!(box_err.score(), 1.0);
            assert_eq!(box_err.well_formed(), Err(InvalidReason::BoxConflict { cell: (5, 2), conflict: (3, 1) }));
            assert_eq!(box_err.finished(), Err(InvalidReason::BoxConflict { cell: (5, 2), conflict: (3, 1) }));
        }
    }
}





/***** ERRORS *****/
/// Explains why a cell isn't valid.
#[derive(Clone, Copy, Debug, EnumDebug, Eq, Hash, PartialEq)]
pub enum InvalidReason {
    /// A cell was empty (only for [`Sudoku::finished()`])
    EmptyCell{ cell: (usize, usize) },

    /// There is a conflicting cell in the cell's row.
    RowConflict{ cell: (usize, usize), conflict: (usize, usize) },
    /// There is a conflicting cell in the cell's column.
    ColConflict{ cell: (usize, usize), conflict: (usize, usize) },
    /// There is a conflicting cell in the cell's box.
    BoxConflict{ cell: (usize, usize), conflict: (usize, usize) },
}
impl Display for InvalidReason {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use InvalidReason::*;
        match self {
            EmptyCell { cell: (x, y) } => write!(f, "Cell ({},{}) is empty", x + 1, y + 2),

            RowConflict { cell: (x1, y1), conflict: (x2, y2) } => write!(f, "Cell ({},{}) conflicts with cell ({},{}) in the same row", x1 + 1, y1 + 1, x2 + 1, y2 + 1),
            ColConflict { cell: (x1, y1), conflict: (x2, y2) } => write!(f, "Cell ({},{}) conflicts with cell ({},{}) in the same column", x1 + 1, y1 + 1, x2 + 1, y2 + 1),
            BoxConflict { cell: (x1, y1), conflict: (x2, y2) } => write!(f, "Cell ({},{}) conflicts with cell ({},{}) in the same box", x1 + 1, y1 + 1, x2 + 1, y2 + 1),
        }
    }
}
impl Error for InvalidReason {}





/***** FORMATTERS *****/
/// Formats the Sudoku with colour.
#[derive(Debug)]
pub struct SudokuColourFormatter<'s> {
    /// The Sudoku to format.
    sudoku : &'s Sudoku,
}
impl<'s> Display for SudokuColourFormatter<'s> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        // Define the colours
        let gray: Style = Style::new().black().bright();

        // Generate the rows...
        for y in 0..9 {
            // Generate the top thing if needed
            if y == 0 {
                writeln!(f, "{}", gray.apply_to("┌───┬───┬───╥───┬───┬───╥───┬───┬───┐"))?;
            }

            // Print the values in this row
            write!(f, "{}", gray.apply_to("│"))?;
            for x in 0..9 {
                write!(f, " {} ", self.sudoku.rows[y][x].map(|i| format!("{i}")).unwrap_or(" ".into()))?;
                if x < 8 && x % 3 == 2 { write!(f, "{}", gray.apply_to("║"))?; }
                else { write!(f, "{}", gray.apply_to("│"))?; }
            }
            writeln!(f)?;

            // Print the bottom thing
            if y < 8 && y % 3 == 2 {
                writeln!(f, "{}", gray.apply_to("╞═══╪═══╪═══╬═══╪═══╪═══╬═══╪═══╪═══╡"))?;
            } else if y < 8 {
                writeln!(f, "{}", gray.apply_to("├───┼───┼───╫───┼───┼───╫───┼───┼───┤"))?;
            } else {
                writeln!(f, "{}", gray.apply_to("└───┴───┴───╨───┴───┴───╨───┴───┴───┘"))?;
            }
        }

        // Done
        Ok(())
    }
}

/// Formats the Sudoku with colour and a mask to determine which are 'fixed' numbers.
#[derive(Debug)]
pub struct SudokuMaskFormatter<'s, 'm> {
    /// The Sudoku to format.
    sudoku : &'s Sudoku,
    /// The mask to apply.
    mask   : &'m Sudoku,
}
impl<'s, 'm> Display for SudokuMaskFormatter<'s, 'm> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        // Define the colours
        let masked : Style = Style::new().bold();
        let error  : Style = Style::new().black().on_red();
        let dim    : Style = Style::new();
        let gray   : Style = Style::new().black().bright();

        // Generate the rows...
        for y in 0..9 {
            // Generate the top thing if needed
            if y == 0 {
                writeln!(f, "{}", gray.apply_to("┌───┬───┬───╥───┬───┬───╥───┬───┬───┐"))?;
            }

            // Print the values in this row
            write!(f, "{}", gray.apply_to("│"))?;
            for x in 0..9 {
                let value  : Option<u8> = self.sudoku.rows[y][x];
                let svalue : String     = value.map(|i| format!("{i}")).unwrap_or(" ".into());

                // WRite it with bold or not, depending on the mask
                if value == self.mask.rows[y][x] {
                    write!(f, " {} ", masked.apply_to(svalue))?;
                } else if self.mask.rows[y][x].is_some() {
                    write!(f, " {} ", error.apply_to(svalue))?;
                } else {
                    write!(f, " {} ", dim.apply_to(svalue))?;
                }

                // Write the border
                if x < 8 && x % 3 == 2 { write!(f, "{}", gray.apply_to("║"))?; }
                else { write!(f, "{}", gray.apply_to("│"))?; }
            }
            writeln!(f)?;

            // Print the bottom thing
            if y < 8 && y % 3 == 2 {
                writeln!(f, "{}", gray.apply_to("╞═══╪═══╪═══╬═══╪═══╪═══╬═══╪═══╪═══╡"))?;
            } else if y < 8 {
                writeln!(f, "{}", gray.apply_to("├───┼───┼───╫───┼───┼───╫───┼───┼───┤"))?;
            } else {
                writeln!(f, "{}", gray.apply_to("└───┴───┴───╨───┴───┴───╨───┴───┴───┘"))?;
            }
        }

        // Done
        Ok(())
    }
}





/***** LIBRARY *****/
/// Represents a single Sudoku.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Sudoku {
    /// It's a 9x9 grid of cells!
    pub rows : [ [ Option<u8>; 9 ]; 9 ],
}

impl Default for Sudoku {
    #[inline]
    fn default() -> Self { Self::empty() }
}
impl Sudoku {
    /// Constructor for an empty Sudoku.
    /// 
    /// # Returns
    /// A new instance of Self with no values in the cells.
    #[inline]
    pub fn empty() -> Self {
        Self {
            rows : [ [ None; 9 ]; 9 ],
        }
    }

    /// Constructor for a Sudoku with given values.
    /// 
    /// # Arguments
    /// - `cells`: The cells to initialize the Sudoku with.
    /// 
    /// # Returns
    /// A new instance of Self with the given values in the cells.
    #[inline]
    pub fn with_values(cells: impl Into<[ [ Option<u8>; 9 ]; 9 ]>) -> Self {
        Self {
            rows : cells.into(),
        }
    }



    /// Returns how many percentage of cells is filled-in.
    /// 
    /// This does not consider well-formedness.
    /// 
    /// # Returns
    /// A ratio of cells filled-in.
    pub fn score(&self) -> f64 { self.rows.iter().map(|r| r.iter().filter(|c| c.is_some()).count()).sum::<usize>() as f64 / 81.0 }

    /// Returns whether a particular cell is valid and, if not, why not.
    /// 
    /// # Arguments
    /// - `x`: The X-coordinate of the cell in the sudoku.
    /// - `y`: The Y-coordinate of the cell in the sudoku.
    /// - `value`: The value of the cell to check.
    /// 
    /// # Errors
    /// This function errors with an [`InvalidReason`] explaining why the cell isn't valid if it wasn't.
    pub fn cell_valid(&self, x: usize, y: usize, value: u8) -> Result<(), InvalidReason> {
        // Check if it's unique in the row direction so far
        for i in 0..x {
            if Some(value) == self.rows[y][i] { return Err(InvalidReason::RowConflict { cell: (x, y), conflict: (i, y) }); }
        }
        // Check if it's unique in the column direction so far
        for i in 0..y {
            if Some(value) == self.rows[i][x] { return Err(InvalidReason::ColConflict { cell: (x, y), conflict: (x, i) }); }
        }
        // Check if it's unique in this 3x3 grid
        let xy_in_grid: usize = (y % 3) * 3 + (x % 3);
        for i in 0..xy_in_grid {
            // Convert back to new x & y
            let x2: usize = 3 * (x / 3) + (i % 3);
            let y2: usize = 3 * (y / 3) + (i / 3);

            // Check if we are in conflict
            if Some(value) == self.rows[y2][x2] { return Err(InvalidReason::BoxConflict { cell: (x, y), conflict: (x2, y2) }); }
        }

        // We made it this far so valid indeed
        Ok(())
    }

    /// Returns whether the Sudoku is well-formed and, if not, why not.
    /// 
    /// This is like [finished](Sudoku::is_finished()), except that not all cells have to be filled-in.
    /// 
    /// # Errors
    /// This function errors with an [`InvalidReason`] explaining why the cell isn't valid if it wasn't.
    pub fn well_formed(&self) -> Result<(), InvalidReason> {
        // Try to find any incorrect cell
        for y in 0..self.rows.len() {
            for x in 0..self.rows[y].len() {
                // Skip if None
                if self.rows[y][x].is_none() { continue; }
                // Assert the cell is valid
                if let Err(reason) = self.cell_valid(x, y, self.rows[y][x].unwrap()) { return Err(reason); }
            }
        }

        // Failed to prove it wasn't; so it's well-formed!
        Ok(())
    }

    /// Returns whether the Sudoku is finished and, if not, why not..
    /// 
    /// This is like [well-formed](Sudoku::is_well_formed()), except that all cells have to be filled-in.
    /// 
    /// # Errors
    /// This function errors with an [`InvalidReason`] explaining why the cell isn't valid if it wasn't.
    pub fn finished(&self) -> Result<(), InvalidReason> {
        // Try to find any incorrect cell
        for y in 0..self.rows.len() {
            for x in 0..self.rows[y].len() {
                // Fail if None
                if self.rows[y][x].is_none() { return Err(InvalidReason::EmptyCell{ cell: (x, y) }); }
                // Assert the cell is valid
                if let Err(reason) = self.cell_valid(x, y, self.rows[y][x].unwrap()) { return Err(reason); }
            }
        }

        // Failed to prove it wasn't; so it's well-formed!
        Ok(())
    }



    /// Returns whether a particular cell is valid.
    /// 
    /// # Arguments
    /// - `x`: The X-coordinate of the cell in the sudoku.
    /// - `y`: The Y-coordinate of the cell in the sudoku.
    /// - `value`: The value of the cell to check.
    /// 
    /// # Returns
    /// True if the cell is valid according to the rules, or else false.
    #[inline]
    pub fn is_cell_valid(&self, x: usize, y: usize, value: u8) -> bool { self.cell_valid(x, y, value).is_ok() }

    /// Returns whether the Sudoku is well-formed.
    /// 
    /// This is like [finished](Sudoku::is_finished()), except that not all cells have to be filled-in.
    /// 
    /// # Returns
    /// True if the filled-in values are OK, or false otherwise.
    #[inline]
    pub fn is_well_formed(&self) -> bool { self.well_formed().is_ok() }

    /// Returns whether the Sudoku is finished.
    /// 
    /// This is like [well-formed](Sudoku::is_well_formed()), except that all cells have to be filled-in.
    /// 
    /// # Returns
    /// True if all values are filled-in and they are OK, or false otherwise.
    #[inline]
    pub fn is_finished(&self) -> bool { self.finished().is_ok() }



    /// Displays the Sudoku with ANSI colours.
    /// 
    /// # Returns
    /// A [`SudokuColourFormatter`] that can format the Sudoku with colours.
    #[inline]
    pub fn coloured(&self) -> SudokuColourFormatter { SudokuColourFormatter { sudoku: self } }

    /// Displays the Sudoku with ANSI colours and a mask.
    /// 
    /// The mask is the original Sudoku so it can be highlighted which parts are 'fixed' and which are 'solved'.
    /// 
    /// # Arguments
    /// - `mask`: The mask [`Sudoku`] to apply.
    /// 
    /// # Returns
    /// A [`SudokuMaskFormatter`] that can format the Sudoku with colours.
    #[inline]
    pub fn masked<'s, 'm>(&'s self, mask: &'m Sudoku) -> SudokuMaskFormatter<'s, 'm> { SudokuMaskFormatter { sudoku: self, mask } }

    /// Renders the Sudoku as a ratatui [`Table`] widget.
    /// 
    /// # Returns
    /// A new [`Table`] widget instance that will draw the current Sudoku state when rendered.
    #[inline]
    pub fn render(&self) -> Table {
        Table::new((0..9).map(|i| Row::new(self.rows[i].iter().map(|v| if let Some(v) = v { format!("{v}") } else { " ".into() }))))
    }
}

impl Display for Sudoku {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        // Generate the rows...
        for y in 0..9 {
            // Generate the top thing if needed
            if y == 0 {
                writeln!(f, "┌───┬───┬───╥───┬───┬───╥───┬───┬───┐")?;
            }

            // Print the values in this row
            write!(f, "│")?;
            for x in 0..9 {
                write!(f, " {} ", self.rows[y][x].map(|i| format!("{i}")).unwrap_or(" ".into()))?;
                if x < 8 && x % 3 == 2 { write!(f, "║")?; }
                else { write!(f, "│")?; }
            }
            writeln!(f)?;

            // Print the bottom thing
            if y < 8 && y % 3 == 2 {
                writeln!(f, "╞═══╪═══╪═══╬═══╪═══╪═══╬═══╪═══╪═══╡")?;
            } else if y < 8 {
                writeln!(f, "├───┼───┼───╫───┼───┼───╫───┼───┼───┤")?;
            } else {
                writeln!(f, "└───┴───┴───╨───┴───┴───╨───┴───┴───┘")?;
            }
        }

        // Done
        Ok(())
    }
}
