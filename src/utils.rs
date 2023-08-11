//  UTILS.rs
//    by Lut99
// 
//  Created:
//    10 Aug 2023, 23:45:41
//  Last edited:
//    11 Aug 2023, 17:32:30
//  Auto updated?
//    Yes
// 
//  Description:
//!   Provides some common utilities for the crate.
// 

use std::error::Error;
use std::ffi::OsString;
use std::fmt::{Debug, Display, Formatter, Result as FResult};
use std::fs::File;
use std::io::Read as _;
use std::path::{Path, PathBuf};
use std::str::FromStr as _;

use unicode_segmentation::UnicodeSegmentation as _;

use crate::spec::FileType;
use crate::sudoku::Sudoku;


/***** TESTS *****/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_sudoku_puzzle() {
        // Load the example
        let sudoku: Sudoku = match load_sudoku_of_type("./tests/example.sdk", FileType::SudokuPuzzle) {
            Ok(mut sudoku) => sudoku.swap_remove(0),
            Err(err)       => { panic!("Failed to parse sudoku file './tests/example.sdk': {}", err.pretty()); },
        };

        // Assert it is what we expect
        assert_eq!(
            sudoku,
            Sudoku::with_values([
                [ Some(2),    None,    None, Some(1),    None, Some(5),    None,    None, Some(3) ],
                [    None, Some(5), Some(4),    None,    None,    None, Some(7), Some(1),    None ],
                [    None, Some(1),    None, Some(2),    None, Some(3),    None, Some(8),    None ],

                [ Some(6),    None, Some(2), Some(8),    None, Some(7), Some(3),    None, Some(4) ],
                [    None,    None,    None,    None,    None,    None,    None,    None,    None ],
                [ Some(1),    None, Some(5), Some(3),    None, Some(9), Some(8),    None, Some(6) ],

                [    None, Some(2),    None, Some(7),    None, Some(1),    None, Some(6),    None ],
                [    None, Some(8), Some(1),    None,    None,    None, Some(2), Some(4),    None ],
                [ Some(7),    None,    None, Some(4),    None, Some(2),    None,    None, Some(1) ],
            ])
        )
    }

    #[test]
    fn test_load_sudoku_puzzle_progress() {
        // Load the example
        let sudoku: Sudoku = match load_sudoku_of_type("./tests/example.sdx", FileType::SudokuPuzzleProgress) {
            Ok(mut sudoku) => sudoku.swap_remove(0),
            Err(err)       => { panic!("Failed to parse sudoku file './tests/example.sdx': {}", err.pretty()); },
        };

        // Assert it is what we expect
        assert_eq!(
            sudoku,
            Sudoku::with_values([
                [ Some(2),    None,    None, Some(1),    None, Some(5),    None, Some(9), Some(3) ],
                [    None, Some(5), Some(4),    None,    None,    None, Some(7), Some(1),    None ],
                [ Some(9), Some(1),    None, Some(2),    None, Some(3),    None, Some(8),    None ],

                [ Some(6), Some(9), Some(2), Some(8), Some(1), Some(7), Some(3),    None, Some(4) ],
                [    None,    None,    None,    None,    None,    None, Some(1),    None,    None ],
                [ Some(1),    None, Some(5), Some(3),    None, Some(9), Some(8),    None, Some(6) ],

                [    None, Some(2),    None, Some(7),    None, Some(1),    None, Some(6),    None ],
                [    None, Some(8), Some(1),    None,    None, Some(6), Some(2), Some(4),    None ],
                [ Some(7),    None,    None, Some(4),    None, Some(2),    None,    None, Some(1) ],
            ])
        )
    }

    #[test]
    fn test_load_sudoku_puzzle_collection() {
        // Load the example
        let sudokus: Vec<Sudoku> = match load_sudoku_of_type("./tests/example.sdm", FileType::SudokuPuzzleCollection) {
            Ok(sudokus) => sudokus,
            Err(err)    => { panic!("Failed to parse sudoku file './tests/example.sdm': {}", err.pretty()); },
        };

        // Assert it is what we expect
        assert_eq!(
            sudokus,
            vec![
                Sudoku::from_compact([ 0,1,6,4,0,0,0,0,0,2,0,0,0,0,9,0,0,0,4,0,0,0,0,0,0,6,2,0,7,0,2,3,0,1,0,0,1,0,0,0,0,0,0,0,3,0,0,3,0,8,7,0,4,0,9,6,0,0,0,0,0,0,5,0,0,0,8,0,0,0,0,7,0,0,0,0,0,6,8,2,0 ]),
                Sudoku::from_compact([ 0,4,9,0,0,8,6,0,5,0,0,3,0,0,7,0,0,0,0,0,0,0,0,0,0,3,0,0,0,0,4,0,0,8,0,0,0,6,0,8,1,5,0,2,0,0,0,1,0,0,9,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,6,0,0,4,0,0,8,0,4,5,0,0,3,9,0]),
                Sudoku::from_compact([ 7,6,0,5,0,0,0,0,0,0,0,0,0,6,0,0,0,8,0,0,0,0,0,0,4,0,3,2,0,0,4,0,0,8,0,0,0,8,0,0,0,0,0,3,0,0,0,5,0,0,1,0,0,7,8,0,9,0,0,0,0,0,0,6,0,0,0,1,0,0,0,0,0,0,0,0,0,3,0,4,1 ]),
                Sudoku::from_compact([ 0,0,0,6,0,5,0,0,0,0,0,3,0,2,0,8,0,0,0,4,5,0,9,0,2,7,0,5,0,0,0,0,0,0,0,1,0,6,2,0,0,0,5,4,0,4,0,0,0,0,0,0,0,7,0,9,8,0,6,0,4,5,0,0,0,6,0,4,0,7,0,0,0,0,0,2,0,3,0,0,0 ]),
                Sudoku::from_compact([ 4,0,9,0,0,0,7,0,5,0,0,0,0,1,0,0,0,0,0,0,6,2,0,7,8,0,0,2,0,0,0,0,0,0,0,9,0,0,3,7,0,4,2,0,0,8,0,0,0,0,0,0,0,4,0,0,2,8,0,1,5,0,0,0,0,0,0,6,0,0,0,0,9,0,5,0,0,0,4,0,6 ]),
                Sudoku::from_compact([ 0,0,0,0,1,0,0,3,0,0,4,0,0,7,0,5,0,1,0,0,2,0,0,8,0,0,6,6,8,0,0,0,0,0,0,3,0,0,0,3,0,2,0,0,0,3,0,0,0,0,0,0,4,5,2,0,0,5,0,0,8,0,0,8,0,1,0,4,0,0,2,0,0,9,0,0,2,0,0,0,0 ]),
                Sudoku::from_compact([ 0,8,0,0,7,0,0,3,0,2,6,0,0,5,0,0,1,8,0,0,0,0,0,0,4,0,0,0,0,0,6,0,2,0,0,0,3,9,0,0,1,0,0,8,6,0,0,0,7,0,9,0,0,0,0,0,4,0,0,0,8,0,0,8,1,0,0,4,0,0,5,2,0,5,0,0,9,0,0,7,0 ]),
                Sudoku::from_compact([ 0,0,0,0,9,3,0,0,6,0,0,0,8,0,0,9,0,0,0,2,0,0,0,6,1,0,0,0,0,0,0,8,0,0,5,3,0,0,6,0,0,0,2,0,0,3,7,0,0,5,0,0,0,0,0,0,2,5,0,0,0,4,0,0,0,1,0,0,9,0,0,0,7,0,0,1,3,0,0,0,0 ]),
            ]
        )
    }

    #[test]
    fn test_load_simple_sudoku_new() {
        // Load the example
        let sudoku: Sudoku = match load_sudoku_of_type("./tests/example_new.ss", FileType::SimpleSudokuNew) {
            Ok(mut sudoku) => sudoku.swap_remove(0),
            Err(err)       => { panic!("Failed to parse sudoku file './tests/example_new.ss': {}", err.pretty()); },
        };

        // Assert it is what we expect
        assert_eq!(
            sudoku,
            Sudoku::from_compact([ 1,0,0,0,0,0,7,0,0,0,2,0,0,0,0,5,0,0,6,0,0,3,8,0,0,0,0,0,7,8,0,0,0,0,0,0,0,0,0,6,0,9,0,0,0,0,0,0,0,0,0,1,4,0,0,0,0,0,2,5,0,0,9,0,0,3,0,0,0,0,6,0,0,0,4,0,0,0,0,0,2 ]),
        )
    }
}





/***** ERRORS *****/
/// Describes what can happen when loading Sudokus
#[derive(Debug)]
pub enum LoadError {
    /// No extension found.
    NoExtension { path: PathBuf },
    /// No known extension given.
    UnknownExtension { path: PathBuf, ext: OsString },

    /// Failed to open a file.
    FileOpen { path: PathBuf, err: std::io::Error },
    /// Failed to parse a file with serde.
    FileParse { ftype: FileType, path: PathBuf, err: Box<dyn Error> },
}
impl Display for LoadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use LoadError::*;
        match self {
            NoExtension { path }           => write!(f, "Given file path '{}' has no extension; cannot deduce type (specify it manually using '--file-type')", path.display()),
            UnknownExtension { path, ext } => write!(f, "Extension '{}' in given file path '{}' is unknown; cannot deduce type (specify it manually using '--file-type')", ext.to_string_lossy(), path.display()),

            FileOpen { path, .. }         => write!(f, "Failed to open file '{}'", path.display()),
            FileParse { ftype, path, .. } => write!(f, "Failed to parse file '{}' as a {} file", path.display(), ftype),
        }
    }
}
impl Error for LoadError {
    fn source(&self) -> Option<&(dyn 'static + Error)> {
        use LoadError::*;
        match self {
            NoExtension { .. }      => None,
            UnknownExtension { .. } => None,

            FileOpen { err, .. }  => Some(err),
            FileParse { err, .. } => Some(&**err),
        }
    }
}

/// Describes what can happen when loading [Sudoku Puzzle](FileType::SudokuPuzzle) [`Sudoku`]s.
#[derive(Debug)]
pub enum SudokuPuzzleError {
    /// Failed to read the input file.
    FileRead { err: std::io::Error },

    /// A metadata rule carried an unknown metadata identifier
    UnknownMetadata { line: usize, marker: String },
    /// A line had too many cells
    IncorrectLength { line: usize, got: usize },
    /// Got an illegal character for a cell.
    IllegalCellChar { line: usize, col: usize, got: String },
    /// Got too many rows.
    TooManyRows { line: usize },
}
impl Display for SudokuPuzzleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use SudokuPuzzleError::*;
        match self {
            FileRead { .. } => write!(f, "Failed to read input file"),

            UnknownMetadata { line, marker }   => write!(f, "Unknown metadata identifier '{marker}' in line {line}"),
            IncorrectLength { line, got }      => write!(f, "Line {line} got {got} cells, expected 9"),
            IllegalCellChar { line, col, got } => write!(f, "Encountered illegal cell character '{got}' in line {line}, column {col}"),
            TooManyRows { line }               => write!(f, "Line {line} adds a row too many"),
        }
    }
}
impl Error for SudokuPuzzleError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        use SudokuPuzzleError::*;
        match self {
            FileRead { err } => Some(err),

            UnknownMetadata { .. } => None,
            IncorrectLength { .. } => None,
            IllegalCellChar { .. } => None,
            TooManyRows { .. }     => None,
        }
    }
}

/// Describes what can happen when loading [Sudoku Puzzle Progress](FileType::SudokuPuzzleProgress) [`Sudoku`]s.
#[derive(Debug)]
pub enum SudokuPuzzleProgressError {
    /// Failed to read the input file.
    FileRead { err: std::io::Error },

    /// A cell is empty.
    EmptyCell { line: usize, cell: usize },
    /// Got an illegal character for a cell.
    IllegalCellChar { line: usize, col: usize, got: String },
    /// Got too many rows.
    TooManyRows { line: usize },
    /// Got too many columns.
    TooManyCols { line: usize },
}
impl Display for SudokuPuzzleProgressError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use SudokuPuzzleProgressError::*;
        match self {
            FileRead { .. } => write!(f, "Failed to read input file"),

            EmptyCell { line, cell }           => write!(f, "No (possible) value assigned to cell {cell} on line {line} (it is empty)"),
            IllegalCellChar { line, col, got } => write!(f, "Encountered illegal cell character '{got}' in line {line}, column {col}"),
            TooManyRows { line }               => write!(f, "Line {line} adds a row too many"),
            TooManyCols { line }               => write!(f, "Line {line} has too many cells"),
        }
    }
}
impl Error for SudokuPuzzleProgressError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        use SudokuPuzzleProgressError::*;
        match self {
            FileRead { err } => Some(err),

            EmptyCell { .. }       => None,
            IllegalCellChar { .. } => None,
            TooManyRows { .. }     => None,
            TooManyCols { .. }     => None,
        }
    }
}

/// Describes what can happen when loading [Sudoku Puzzle Collection](FileType::SudokuPuzzleCollection) [`Sudoku`]s.
#[derive(Debug)]
pub enum SudokuPuzzleCollectionError {
    /// Failed to read the input file.
    FileRead { err: std::io::Error },

    /// Got too many columns.
    TooManyCells { line: usize, got: usize },
    /// Got an illegal character for a cell.
    IllegalCellChar { line: usize, cell: usize, got: String },
}
impl Display for SudokuPuzzleCollectionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use SudokuPuzzleCollectionError::*;
        match self {
            FileRead { .. } => write!(f, "Failed to read input file"),

            TooManyCells { line, got }          => write!(f, "Sudoku on line {line} has too many cells (got {got}, expected 81)"),
            IllegalCellChar { line, cell, got } => write!(f, "Encountered illegal cell character '{got}' in line {line}, cell {cell}"),
        }
    }
}
impl Error for SudokuPuzzleCollectionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        use SudokuPuzzleCollectionError::*;
        match self {
            FileRead { err } => Some(err),

            TooManyCells { .. }    => None,
            IllegalCellChar { .. } => None,
        }
    }
}

/// Describes what can happen when loading [Simple Sudoku (New Style)](FileType::SimpleSudokuNew) [`Sudoku`]s.
#[derive(Debug)]
pub enum SimpleSudokuNewError {
    /// Failed to read the input file.
    FileRead { err: std::io::Error },

    /// Marker row was incorrect.
    IllegalSeparatorRow { line: usize, got: String },
    /// Marker column was incorrect.
    IllegalSeparatorCol { line: usize, got: String },
    /// Got too many columns.
    TooManyCols { line: usize },
    /// Got too many rows.
    TooManyRows { line: usize },
    /// Got an illegal character for a cell.
    IllegalCellChar { line: usize, col: usize, got: String },
}
impl Display for SimpleSudokuNewError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use SimpleSudokuNewError::*;
        match self {
            FileRead { .. } => write!(f, "Failed to read input file"),

            IllegalSeparatorRow { line, got }  => write!(f, "Got row separator '{got}' on line {line}, expected '-----------'"),
            IllegalSeparatorCol { line, got }  => write!(f, "Got column separator '{got}' on line {line}, expected '|'"),
            TooManyCols { line }               => write!(f, "Too many cells on line {line}"),
            TooManyRows { line }               => write!(f, "Line {line} adds too many rows to sudoku"),
            IllegalCellChar { line, col, got } => write!(f, "Encountered illegal cell character '{got}' in line {line}, cell {col}"),
        }
    }
}
impl Error for SimpleSudokuNewError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        use SimpleSudokuNewError::*;
        match self {
            FileRead { err } => Some(err),

            IllegalSeparatorRow { .. } => None,
            IllegalSeparatorCol { .. } => None,
            TooManyCols { .. }         => None,
            TooManyRows { .. }         => None,
            IllegalCellChar { .. }     => None,
        }
    }
}





/***** HELPER FUNCTIONS *****/
/// Parses the [Sudoku Puzzle](FileType::SudokuPuzzle) format.
/// 
/// # Arguments
/// - `handle`: A [`File`]-handle from which we read the puzzle.
/// 
/// # Returns
/// A new [`Sudoku`] read from the given `handle.`
/// 
/// # Errors
/// This function may error if the given `handle` did not contain valid SudokuPuzzle contents.
fn parse_sudoku_puzzle(mut handle: File) -> Result<Vec<Sudoku>, SudokuPuzzleError> {
    // Read the whole file
    let mut raw: String = String::new();
    if let Err(err) = handle.read_to_string(&mut raw) {
        return Err(SudokuPuzzleError::FileRead{ err });
    }
    drop(handle);

    // Read line-by-line to collect the cells
    let mut y: usize = 0;
    let mut rows: [ [ Option<u8>; 9 ]; 9 ] = [ [ None; 9 ]; 9 ];
    for (l, line) in raw.split('\n').enumerate() {
        // Ignore whitespace-only lines
        if line.trim().len() == 0 { continue; }

        // Split into logical graphemes
        let line_chars: Vec<&str> = line.graphemes(true).collect();

        // If the line stars with a comment, it's metadata
        if line_chars.len() >= 2 && line_chars[0] == "#" {
            // Analyse the first letter
            match line_chars[1] {
                // Valid characters
                "A" |
                "D" |
                "C" |
                "B" |
                "S" |
                "L" |
                "U" => { continue; },

                // The rest is unknown
                marker => { return Err(SudokuPuzzleError::UnknownMetadata { line: l + 1, marker: marker.into() }); },
            }
        }

        // Otherwise, parse as exactly 9 numbers or dots
        if line_chars.len() != 9 { return Err(SudokuPuzzleError::IncorrectLength { line: l + 1, got: line_chars.len() }); }
        let mut row: [ Option<u8>; 9 ] = [ None; 9 ];
        for (x, c) in line_chars.into_iter().enumerate() {
            // Mark it if it's a number
            if c.len() == 1 && c.chars().next().unwrap() >= '0' && c.chars().next().unwrap() <= '9' {
                row[x] = Some(u8::from_str(c).unwrap());
            } else if c != "." {
                return Err(SudokuPuzzleError::IllegalCellChar { line: l + 1, col: x + 1, got: c.into() });
            }
        }

        // Add to the rows
        if y >= 9 { return Err(SudokuPuzzleError::TooManyRows { line: l + 1 }); }
        rows[y] = row;
        y += 1;
    }

    // Done!
    Ok(vec![ Sudoku::with_values(rows) ])
}

/// Parses the [Sudoku Puzzle Progress](FileType::SudokuPuzzleProgress) format.
/// 
/// # Arguments
/// - `handle`: A [`File`]-handle from which we read the puzzle.
/// 
/// # Returns
/// A new [`Sudoku`] read from the given `handle.`
/// 
/// # Errors
/// This function may error if the given `handle` did not contain valid SudokuPuzzleProgress contents.
fn parse_sudoku_puzzle_progress(mut handle: File) -> Result<Vec<Sudoku>, SudokuPuzzleProgressError> {
    // Read the whole file
    let mut raw: String = String::new();
    if let Err(err) = handle.read_to_string(&mut raw) {
        return Err(SudokuPuzzleProgressError::FileRead{ err });
    }
    drop(handle);

    // Reads the lines, separated by spaces
    let mut y: usize = 0;
    let mut rows: [ [ Option<u8>; 9 ]; 9 ] = [ [ None; 9 ]; 9 ];
    for (l, line) in raw.split('\n').enumerate() {
        // Ignore whitespace-only lines
        if line.trim().len() == 0 { continue; }

        // Parse 9 cells
        let mut row: [ Option<u8>; 9 ] = [ None; 9 ];
        for (x, c) in line.split(' ').enumerate() {
            // Split into logical graphemes
            let c_chars: Vec<&str> = c.graphemes(true).collect();
            if c_chars.is_empty() { return Err(SudokuPuzzleProgressError::EmptyCell { line: l + 1, cell: x + 1 }); }

            // Parse it as viable numbers
            let mut ns: Vec<u8> = vec![];
            for (i, digit) in c_chars.into_iter().enumerate() {
                // Parse the number digit otherwise
                if digit.len() == 1 && digit.chars().next().unwrap() >= '0' && digit.chars().next().unwrap() <= '9' {
                    ns.push(u8::from_str(digit).unwrap());
                } else if i > 0 || digit != "u" {
                    return Err(SudokuPuzzleProgressError::IllegalCellChar { line: l + 1, col: x + 1, got: digit.into() });
                }
            }

            // If only one possibility remains, it's set; otherwise, mark as empty
            if x >= 9 {  }
            if ns.len() == 1 {
                row[x] = Some(ns[0]);
            } else if !ns.is_empty() {
                row[x] = None;
            } else {
                return Err(SudokuPuzzleProgressError::EmptyCell { line: l + 1, cell: x + 1 });
            }
        }

        // Add to the rows
        if y >= 9 { return Err(SudokuPuzzleProgressError::TooManyRows { line: l + 1 }); }
        rows[y] = row;
        y += 1;
    }

    // Done!
    Ok(vec![ Sudoku::with_values(rows) ])
}

/// Parses the [Sudoku Puzzle Collection](FileType::SudokuPuzzleCollection) format.
/// 
/// This format may contain multiple sudoku's.
/// 
/// # Arguments
/// - `handle`: A [`File`]-handle from which we read the puzzle.
/// 
/// # Returns
/// A new [`Sudoku`] read from the given `handle.`
/// 
/// # Errors
/// This function may error if the given `handle` did not contain valid SudokuPuzzleProgress contents.
fn parse_sudoku_puzzle_collection(mut handle: File) -> Result<Vec<Sudoku>, SudokuPuzzleCollectionError> {
    // Read the whole file
    let mut raw: String = String::new();
    if let Err(err) = handle.read_to_string(&mut raw) {
        return Err(SudokuPuzzleCollectionError::FileRead{ err });
    }
    drop(handle);

    // Read the lines (one Sudoku per line)
    let mut sudokus: Vec<Sudoku> = vec![];
    for (l, line) in raw.split('\n').enumerate() {
        // Read exactly 81 characters
        let cells: Vec<&str> = line.graphemes(true).collect();
        if cells.len() != 81 { return Err(SudokuPuzzleCollectionError::TooManyCells { line: l + 1, got: cells.len() }); }

        // Parse all as single-digit numbers
        let mut rows: [ [ Option<u8>; 9 ]; 9 ] = [ [ None; 9 ]; 9 ];
        for (i, cell) in cells.into_iter().enumerate() {
            if cell.len() == 1 && cell.chars().next().unwrap() >= '0' && cell.chars().next().unwrap() <= '9' {
                let value: u8 = u8::from_str(cell).unwrap();
                if value > 0 {
                    rows[i / 9][i % 9] = Some(value);
                } else {
                    rows[i / 9][i % 9] = None;
                }
            } else {
                return Err(SudokuPuzzleCollectionError::IllegalCellChar { line: l + 1, cell: i + 1, got: cell.into() });
            }
        }

        // Store the sudoku
        sudokus.push(Sudoku::with_values(rows));
    }

    // Ok done!
    Ok(sudokus)
}

/// Parses the [Simple Sudoku (New Style)](FileType::SimpleSudokuNew) format.
/// 
/// # Arguments
/// - `handle`: A [`File`]-handle from which we read the puzzle.
/// 
/// # Returns
/// A new [`Sudoku`] read from the given `handle.`
/// 
/// # Errors
/// This function may error if the given `handle` did not contain valid SudokuPuzzleProgress contents.
fn parse_simple_sudoku_new(mut handle: File) -> Result<Vec<Sudoku>, SimpleSudokuNewError> {
    // Read the whole file
    let mut raw: String = String::new();
    if let Err(err) = handle.read_to_string(&mut raw) {
        return Err(SimpleSudokuNewError::FileRead{ err });
    }
    drop(handle);

    // Read the lines
    let mut y: usize = 0;
    let mut rows: [ [ Option<u8>; 9 ]; 9 ] = [ [ None; 9 ]; 9 ];
    for (l, line) in raw.split('\n').enumerate() {
        // Parse only markings on the fourth and eights lines
        if l == 3 || l == 7 {
            if line != "-----------" { return Err(SimpleSudokuNewError::IllegalSeparatorRow { line: l + 1, got: line.into() }); }
            continue;
        }

        // Otherwise, parse three cells, separator, three cells, separator, three cells
        let mut row: [ Option<u8>; 9 ] = [ None; 9 ];
        let line_chars: Vec<&str> = line.graphemes(true).collect();
        for (mut x, c) in line_chars.into_iter().enumerate() {
            if x >= 11 { return Err(SimpleSudokuNewError::TooManyCols { line: l + 1 }); }

            // Parse separators on column 4 & 8
            if x == 3 || x == 7 {
                if c != "|" { return Err(SimpleSudokuNewError::IllegalSeparatorCol { line: l + 1, got: c.into() }); }
                continue;
            }

            // Compensate the X-coordinate for the separators
            if x >= 7 { x -= 1; }
            if x >= 3 { x -= 1; }

            // Otherwise, parse as digit
            if c.len() == 1 && c.chars().next().unwrap() >= '0' && c.chars().next().unwrap() <= '9' {
                row[x] = Some(u8::from_str(c).unwrap());
            } else if c != "." {
                return Err(SimpleSudokuNewError::IllegalCellChar { line: l + 1, col: x + 1, got: c.into() });
            }
        }

        // Add the row
        if y >= 9 { return Err(SimpleSudokuNewError::TooManyRows { line: l + 1 }); }
        rows[y] = row;
        y += 1;
    }

    // Done!
    Ok(vec![ Sudoku::with_values(rows) ])
}





/***** FORMATTERS *****/
/// Formats an error and all its dependencies.
pub struct PrettyErrorFormatter<'e, E: ?Sized> {
    /// The error to format.
    err : &'e E,
}
impl<'e, E: Error> Debug for PrettyErrorFormatter<'e, E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        // Always print the thing
        write!(f, "{:?}", self.err)?;

        // Print any deps if any
        if let Some(source) = self.err.source() {
            // Write the thingy
            write!(f, "\n\nCaused by:")?;

            let mut source: Option<&dyn Error> = Some(source);
            while let Some(err) = source.take() {
                // Print it
                write!(f, "\n - {err:?}")?;
                source = err.source();
            }
        }

        // Done!
        Ok(())
    }
}
impl<'e, E: Error> Display for PrettyErrorFormatter<'e, E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        // Always print the thing
        write!(f, "{}", self.err)?;

        // Print any deps if any
        if let Some(source) = self.err.source() {
            // Write the thingy
            write!(f, "\n\nCaused by:")?;

            let mut source: Option<&dyn Error> = Some(source);
            while let Some(err) = source.take() {
                // Print it
                write!(f, "\n - {err}")?;
                source = err.source();
            }
        }

        // Done!
        Ok(())
    }
}





/***** LIBRARY FUNCTIONS *****/
/// Helper function that loads a Sudoku file, automatically deducing its type from the file extension.
/// 
/// # Arguments
/// - `path`: The path to the Sudoku file to open.
/// 
/// # Returns
/// One or more [`Sudoku`]s parsed from the file.
/// 
/// # Errors
/// This function may error if we failed to read or correctly parse the file.
pub fn load_sudoku(path: impl AsRef<Path>) -> Result<Vec<Sudoku>, LoadError> {
    let path: &Path = path.as_ref();

    // Analyse the method of opening
    let ftype: FileType = if let Some(ext) = path.extension() {
        match FileType::from_ext(ext) {
            Some(ftype) => ftype,
            None        => { return Err(LoadError::UnknownExtension { path: path.into(), ext: ext.into() }); },
        }
    } else {
        return Err(LoadError::NoExtension { path: path.into() });
    };

    // Pass to type-set parsing
    load_sudoku_of_type(path, ftype)
}

/// Helper function that loads a Sudoku file of given type.
/// 
/// # Arguments
/// - `path`: The path to the Sudoku file to open.
/// - `ftype`: The type of the file. This determines how to parse its contents.
/// 
/// # Returns
/// One or more [`Sudoku`]s parsed from the file.
/// 
/// # Errors
/// This function may error if we failed to read or correctly parse the file.
pub fn load_sudoku_of_type(path: impl AsRef<Path>, ftype: FileType) -> Result<Vec<Sudoku>, LoadError> {
    let path: &Path = path.as_ref();

    // Open the file
    let handle: File = match File::open(path) {
        Ok(handle) => handle,
        Err(err)   => { return Err(LoadError::FileOpen { path: path.into(), err }); },
    };

    // Parse it according to the type
    match ftype {
        // Simple serde
        FileType::Json => match serde_json::from_reader(handle) {
            Ok(sudoku) => Ok(vec![ sudoku ]),
            Err(err)   => Err(LoadError::FileParse { ftype, path: path.into(), err: Box::new(err) }),
        },

        // Specialized formats
        FileType::SudokuPuzzle => match parse_sudoku_puzzle(handle) {
            Ok(sudoku) => Ok(sudoku),
            Err(err)   => Err(LoadError::FileParse { ftype, path: path.into(), err: Box::new(err) }),
        },
        FileType::SudokuPuzzleProgress => match parse_sudoku_puzzle_progress(handle) {
            Ok(sudoku) => Ok(sudoku),
            Err(err)   => Err(LoadError::FileParse { ftype, path: path.into(), err: Box::new(err) }),
        },
        FileType::SudokuPuzzleCollection => match parse_sudoku_puzzle_collection(handle) {
            Ok(sudoku) => Ok(sudoku),
            Err(err)   => Err(LoadError::FileParse { ftype, path: path.into(), err: Box::new(err) }),
        },
        FileType::SimpleSudoku => {
            todo!();
        },
        FileType::SimpleSudokuNew => match parse_simple_sudoku_new(handle) {
            Ok(sudoku) => Ok(sudoku),
            Err(err)   => Err(LoadError::FileParse { ftype, path: path.into(), err: Box::new(err) }),
        },
        FileType::SimpleSudokuOld => {
            todo!();
        },
    }
}





/***** LIBRARY *****/
/// Implements functions for printing any [`Error`] very neatly.
pub trait PrettyError: Error {
    /// Returns a formatter for showing this Error and all its [source](Error::source())s.
    /// 
    /// # Returns
    /// A new [`PrettyErrorFormatter`] that can do the job.
    fn pretty(&self) -> PrettyErrorFormatter<Self>;
}
impl<T: Error> PrettyError for T {
    fn pretty(&self) -> PrettyErrorFormatter<Self> {
        PrettyErrorFormatter { err: self }
    }
}
