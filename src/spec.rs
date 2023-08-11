//  SPEC.rs
//    by Lut99
// 
//  Created:
//    10 Aug 2023, 23:02:38
//  Last edited:
//    11 Aug 2023, 15:55:54
//  Auto updated?
//    Yes
// 
//  Description:
//!   Defines how we represent a Sudoku.
// 

use std::error::Error;
use std::ffi::OsStr;
use std::fmt::{Display, Formatter, Result as FResult};
use std::str::FromStr;

use enum_debug::EnumDebug;


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





/***** LIBRARY *****/
/// Defines possible Sudoku file types.
#[derive(Clone, Copy, Debug, EnumDebug, Eq, Hash, PartialEq)]
pub enum FileType {
    // General types
    /// Load it as a direct JSON representation of the [`Sudoku`](crate::sudoku::Sudoku)-struct.
    /// 
    /// # Example
    /// ```json
    /// {
    ///     "rows": [
    ///         [ 4, 3, 5,   2, 6, 9,   7, 8, 1 ],
    ///         [ 6, 8, 2,   5, 7, 1,   4, 9, 3 ],
    ///         [ 1, 9, 7,   8, 3, 4,   5, 6, 2 ],
    /// 
    ///         [ 8, 2, 6,   1, 9, 5,   3, 4, 7 ],
    ///         [ 3, 7, 4,   6, 8, 2,   9, 1, 5 ],
    ///         [ 9, 5, 1,   7, 4, 3,   6, 2, 8 ],
    /// 
    ///         [ 5, 1, 9,   3, 2, 6,   8, 7, 4 ],
    ///         [ 2, 4, 8,   9, 5, 7,   1, 3, 6 ],
    ///         [ 7, 6, 3,   4, 1, 8,   2, 5, 9 ]
    ///     ]
    /// }
    /// ```
    Json,

    // Specialized types
    /// The Sudoku Puzzle (.sdk) format, which can contain Sudoku metadata (but which we will ignore).
    /// 
    /// See <http://www.sudocue.net/fileformats.php>.
    /// 
    /// # Example
    /// ```plain
    /// #ARuud
    /// #DA random puzzle created by SudoCue
    /// #CJust start plugging in the numbers
    /// #B03-08-2006
    /// #SSudoCue
    /// #LEasy
    /// #Uhttp://www.sudocue.net/
    /// 2..1.5..3
    /// .54...71.
    /// .1.2.3.8.
    /// 6.28.73.4
    /// .........
    /// 1.53.98.6
    /// .2.7.1.6.
    /// .81...24.
    /// 7..4.2..1
    /// ```
    SudokuPuzzle,
    /// The Sudoku Puzzle Progress (.sdx) format, which represents a sudoku as possible options (i.e., small numbers stuff).
    /// 
    /// See <http://www.sudocue.net/fileformats.php>.
    /// 
    /// # Example
    /// ```plain
    /// 2 679 6789 1 46789 5 469 9 3
    /// 389 5 4 69 689 68 7 1 29
    /// 9 1 679 2 4679 3 4569 8 59
    /// 6 9 2 8 u1 7 3 59 4
    /// 3489 3479 3789 56 2456 46 u1 2579 2579
    /// 1 47 5 3 24 9 8 27 6
    /// 3459 2 39 7 3589 1 59 6 589
    /// 359 8 1 569 3569 6 2 4 579
    /// 7 369 369 4 35689 2 59 359 1
    /// ```
    SudokuPuzzleProgress,
    /// The Sudoku Puzzle Collection (.sdm) format, which holds multiple sudokus in one file.
    /// 
    /// See <http://www.sudocue.net/fileformats.php>.
    /// 
    /// # Example
    /// ```plain
    /// 016400000200009000400000062070230100100000003003087040960000005000800007000006820
    /// 049008605003007000000000030000400800060815020001009000010000000000600400804500390
    /// 760500000000060008000000403200400800080000030005001007809000000600010000000003041
    /// 000605000003020800045090270500000001062000540400000007098060450006040700000203000
    /// 409000705000010000006207800200000009003704200800000004002801500000060000905000406
    /// 000010030040070501002008006680000003000302000300000045200500800801040020090020000
    /// 080070030260050018000000400000602000390010086000709000004000800810040052050090070
    /// 000093006000800900020006100000080053006000200370050000002500040001009000700130000
    /// ```
    SudokuPuzzleCollection,
    /// The Simple Sudoku (.ss) format, which can parse both the new- and old-style format.
    /// 
    /// This is the older version of the two.
    /// 
    /// See <http://www.sudocue.net/fileformats.php>.
    /// 
    /// # Example
    /// See the [new-style](FileType::SimpleSudokuNew) or [old-style](FileType::SimpleSudokuOld) representations for examples.
    SimpleSudoku,
    /// The Simple Sudoku (.ss) format, which is very human-readable.
    /// 
    /// This is the newer version of the two.
    /// 
    /// See <http://www.sudocue.net/fileformats.php>.
    /// 
    /// # Example
    /// ```plain
    /// 1..|...|7..
    /// .2.|...|5..
    /// 6..|38.|...
    /// -----------
    /// .78|...|...
    /// ...|6.9|...
    /// ...|...|14.
    /// -----------
    /// ...|.25|..9
    /// ..3|...|.6.
    /// ..4|...|..2
    /// ```
    SimpleSudokuNew,
    /// The Simple Sudoku (.ss) format, which is very human-readable.
    /// 
    /// This is the older version of the two.
    /// 
    /// See <http://www.sudocue.net/fileformats.php>.
    /// 
    /// # Example
    /// ```plain
    /// X6X1X4X5X
    /// XX83X56XX
    /// 2XXXXXXX1
    /// 8XX4X7XX6
    /// XX6XXX3XX
    /// 7XX9X1XX4
    /// 5XXXXXXX2
    /// XX72X69XX
    /// X4X5X8X7X
    /// ```
    SimpleSudokuOld,
}
impl FileType {
    /// Attempts to deduce the file type from the given extension.
    /// 
    /// # Arguments
    /// - `ext`: The [`Extension`] in a filepath to analyse.
    /// 
    /// # Returns
    /// The corresponding FileType if it was known, or [`None`] otherwise.
    pub fn from_ext(ext: &OsStr) -> Option<Self> {
        // Check if it is a valid extension
        if ext == OsStr::new("json") {
            Some(Self::Json)
        } else if ext == OsStr::new("sdk") {
            Some(Self::SudokuPuzzle)
        } else if ext == OsStr::new("sdx") {
            Some(Self::SudokuPuzzleProgress)
        } else if ext == OsStr::new("sdm") {
            Some(Self::SudokuPuzzleCollection)
        } else if ext == OsStr::new("ss") {
            Some(Self::SimpleSudoku)
        } else {
            None
        }
    }
}

impl Display for FileType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use FileType::*;
        match self {
            Json   => write!(f, "JSON"),

            SudokuPuzzle           => write!(f, "Sudoku Puzzle"),
            SudokuPuzzleProgress   => write!(f, "Sudoku Puzzle Progress"),
            SudokuPuzzleCollection => write!(f, "Sudoku Puzzle Collection"),
            SimpleSudoku           => write!(f, "Simple Sudoku"),
            SimpleSudokuNew        => write!(f, "Simple Sudoku (new style)"),
            SimpleSudokuOld        => write!(f, "Simple Sudoku (old style)"),
        }
    }
}
impl FromStr for FileType {
    type Err = FileTypeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(Self::Json),

            "sudoku_puzzle"            | "sdk" => Ok(Self::SudokuPuzzle),
            "sudoku_puzzle_progress"   | "sdx" => Ok(Self::SudokuPuzzleProgress),
            "sudoku_puzzle_collection" | "sdm" => Ok(Self::SudokuPuzzleCollection),
            "simple_sudoku" | "ss"             => Ok(Self::SimpleSudoku),
            "simple_sudoku_new" | "ss_new"     => Ok(Self::SimpleSudokuNew),
            "simple_sudoku_old" | "ss_old"     => Ok(Self::SimpleSudokuOld),

            _ => Err(FileTypeParseError::Unknown { raw: s.into() }),
        }
    }
}
