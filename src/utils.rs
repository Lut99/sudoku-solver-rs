//  UTILS.rs
//    by Lut99
// 
//  Created:
//    10 Aug 2023, 23:45:41
//  Last edited:
//    11 Aug 2023, 14:39:41
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
use std::path::{Path, PathBuf};

use crate::spec::FileType;
use crate::sudoku::Sudoku;


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
/// A [`Sudoku`] that should be well-formed and finished.
/// 
/// # Errors
/// This function may error if we failed to read or correctly parse the file.
pub fn load_sudoku(path: impl AsRef<Path>) -> Result<Sudoku, LoadError> {
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
/// A [`Sudoku`] that matches the contents of the file.
/// 
/// # Errors
/// This function may error if we failed to read or correctly parse the file.
pub fn load_sudoku_of_type(path: impl AsRef<Path>, ftype: FileType) -> Result<Sudoku, LoadError> {
    let path: &Path = path.as_ref();

    // Open the file
    let handle: File = match File::open(path) {
        Ok(handle) => handle,
        Err(err)   => { return Err(LoadError::FileOpen { path: path.into(), err }); },
    };

    // Parse it according to the type
    match ftype {
        FileType::Json => match serde_json::from_reader(handle) {
            Ok(sudoku) => Ok(sudoku),
            Err(err)   => Err(LoadError::FileParse { ftype, path: path.into(), err: Box::new(err) }),
        }
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
