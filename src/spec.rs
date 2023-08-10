//  SPEC.rs
//    by Lut99
// 
//  Created:
//    10 Aug 2023, 23:02:38
//  Last edited:
//    11 Aug 2023, 00:33:42
//  Auto updated?
//    Yes
// 
//  Description:
//!   Defines how we represent a Sudoku.
// 

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FResult};
use std::str::FromStr;

use enum_debug::EnumDebug;
use ratatui::widgets::{Row, Table};
use serde::{Deserialize, Serialize};


/***** ERRORS *****/
/// Describes what can go wrong when parsing [`FileType`]s.
#[derive(Debug)]
pub enum FileTypeParseError {
    /// Unknown file type given.
    Unknown { raw: String },
}
impl Display for FileTypeParseError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use FileTypeParseError::*;
        match self {
            Unknown { raw } => write!(f, "Unknown file type '{raw}'"),
        }
    }
}
impl Error for FileTypeParseError {}





/***** AUXILLARY *****/
/// Defines possible Sudoku file types.
#[derive(Clone, Copy, Debug, EnumDebug, Eq, Hash, PartialEq)]
pub enum FileType {
    /// Load it as JSON
    Json,
}

impl Display for FileType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use FileType::*;
        match self {
            Json => write!(f, "JSON"),
        }
    }
}
impl FromStr for FileType {
    type Err = FileTypeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(Self::Json),
            _      => Err(FileTypeParseError::Unknown { raw: s.into() }),
        }
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
    fn default() -> Self { Self::new() }
}
impl Sudoku {
    /// Constructor for an empty Sudoku.
    /// 
    /// # Returns
    /// A new instance of Self with no values in the cells.
    #[inline]
    pub fn new() -> Self {
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



    /// Returns whether the Sudoku is well-formed.
    /// 
    /// This is like [finished](Sudoku::is_finished()), except that not all cells have to be filled-in.
    /// 
    /// # Returns
    /// True if the filled-in values are OK, or false otherwise.
    pub fn is_well_formed(&self) -> bool {
        // Try to find any incorrect cell
        for y in 0..self.rows.len() {
            for x in 0..self.rows[y].len() {
                // Skip if None
                if self.rows[y][x].is_none() { continue; }

                // Check if it's unique in the row direction so far
                for i in 0..x {
                    if self.rows[y][x] == self.rows[y][i] { return false; }
                }
                // Check if it's unique in the column direction so far
                for i in 0..y {
                    if self.rows[y][x] == self.rows[i][x] { return false; }
                }
                // Check if it's unique in this 3x3 grid
                for y2 in 3 * (y / 3)..3 * (y / 3 + 1) {
                    for x2 in 3 * (x / 3)..3 * (x / 3 + 1) {
                        if self.rows[y][x] == self.rows[y2][x2] { return false; }
                    }
                }
            }
        }

        // Failed to prove it wasn't; so it's well-formed!
        true
    }

    /// Returns how many percentage of cells is filled-in.
    /// 
    /// This does not consider well-formedness.
    /// 
    /// # Returns
    /// A ratio of cells filled-in.
    pub fn score(&self) -> f64 { self.rows.iter().map(|r| r.iter().filter(|c| c.is_some()).count()).sum::<usize>() as f64 / 81.0 }

    /// Returns whether the Sudoku is finished.
    /// 
    /// This is like [well-formed](Sudoku::is_well_formed()), except that all cells have to be filled-in.
    /// 
    /// # Returns
    /// True if all values are filled-in and they are OK, or false otherwise.
    pub fn is_finished(&self) -> bool {
        // Try to find any incorrect cell
        for y in 0..self.rows.len() {
            for x in 0..self.rows[y].len() {
                // Fail if None
                if self.rows[y][x].is_none() { return false; }

                // Check if it's unique in the row direction so far
                for i in 0..x {
                    if self.rows[y][x] == self.rows[y][i] { return false; }
                }
                // Check if it's unique in the column direction so far
                for i in 0..y {
                    if self.rows[y][x] == self.rows[i][x] { return false; }
                }
                // Check if it's unique in this 3x3 grid
                for y2 in 3 * (y / 3)..3 * (y / 3 + 1) {
                    for x2 in 3 * (x / 3)..3 * (x / 3 + 1) {
                        if self.rows[y][x] == self.rows[y2][x2] { return false; }
                    }
                }
            }
        }

        // Failed to prove it wasn't; so it's well-formed!
        true
    }



    /// Renders the Sudoku as a ratatui [`Table`] widget.
    /// 
    /// # Returns
    /// A new [`Table`] widget instance that will draw the current Sudoku state when rendered.
    #[inline]
    pub fn render(&self) -> Table {
        Table::new((0..9).map(|i| Row::new(self.rows[i].iter().map(|v| if let Some(v) = v { format!("{v}") } else { " ".into() }))))
    }
}
