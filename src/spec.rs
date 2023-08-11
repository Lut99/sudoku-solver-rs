//  SPEC.rs
//    by Lut99
// 
//  Created:
//    10 Aug 2023, 23:02:38
//  Last edited:
//    11 Aug 2023, 14:31:46
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
    /// Load it as JSON
    Json,
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
        } else {
            None
        }
    }
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
