//  UTILS.rs
//    by Lut99
// 
//  Created:
//    10 Aug 2023, 23:45:41
//  Last edited:
//    10 Aug 2023, 23:51:41
//  Auto updated?
//    Yes
// 
//  Description:
//!   Provides some common utilities for the crate.
// 

use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FResult};


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
