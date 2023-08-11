//  ENGINE.rs
//    by Lut99
// 
//  Created:
//    10 Aug 2023, 23:23:58
//  Last edited:
//    11 Aug 2023, 15:04:06
//  Auto updated?
//    Yes
// 
//  Description:
//!   Implements the driving part of the solver.
// 

use std::error;
use std::fmt::{Display, Formatter, Result as FResult};
use std::io::{self, Stdout};
use std::time::Duration;

use crossterm::execute;
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use log::warn;
use ratatui::{Frame, Terminal};
use ratatui::backend::CrosstermBackend;
use ratatui::widgets::Paragraph;

use crate::solvers::Solver;
use crate::sudoku::Sudoku;


/***** ERRORS *****/
/// Defines errors that relate to the UI.
#[derive(Debug)]
pub enum Error {
    /// Key detection failed.
    KeyDetect { err: std::io::Error },

    /// Failed to initialize terminal raw mode.
    RawModeEnable { err: std::io::Error },
    /// Failed to enter alternate screen mode for the terminal.
    AlternateScreenEnable { err: std::io::Error },
    /// Failed to create a new terminal.
    TerminalInitialize { err: std::io::Error },
    /// Failed to draw the next frame.
    FrameDraw { err: std::io::Error },
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use Error::*;
        match self {
            KeyDetect { .. } => write!(f, "Failed to detect keypress"),

            RawModeEnable { .. }         => write!(f, "Failed to move terminal to raw mode"),
            AlternateScreenEnable { .. } => write!(f, "Failed to move terminal to alternate screen mode"),
            TerminalInitialize { .. }    => write!(f, "Failed to initialize new terminal"),
            FrameDraw { .. }             => write!(f, "Failed to draw next terminal frame"),
        }
    }
}
impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        use Error::*;
        match self {
            KeyDetect { err } => Some(err),

            RawModeEnable { err }         => Some(err),
            AlternateScreenEnable { err } => Some(err),
            TerminalInitialize { err }    => Some(err),
            FrameDraw { err }             => Some(err),
        }
    }
}





/***** HELPER FUNCTIONS *****/
/// Checks if 'Q' was pressed.
/// 
/// # Arguments
/// - `timeout`: The time to wait until the user presses.
/// 
/// # Returns
/// True if it was, false if it wasn't.
/// 
/// # Errors
/// This function may error if we failed to poll for a key press.
fn should_quit(timeout: Duration) -> Result<bool, Error> {
    // We timeout every 250 ms to make sure we keep on doing work
    if event::poll(timeout).map_err(|err| Error::KeyDetect { err })? {
        if let Event::Key(key) = event::read().map_err(|err| Error::KeyDetect { err })? {
            return Ok(KeyCode::Char('q') == key.code);
        }
    }
    Ok(false)
}





/***** LIBRARY *****/
/// Our own wrapper around ratatui's [Terminal](RTerminal) that automatically restores the terminal when it goes out-of-scope.
pub struct Engine<S> {
    /// The solver to run Sudoku's with.
    solver  : S,
    /// The time to wait in between compute steps.
    timeout : Duration,

    /// The nested ratatui's terminal
    term : Terminal<CrosstermBackend<Stdout>>,
}

impl<S> Engine<S> {
    /// Constructor for the TerminalUI.
    /// 
    /// # Arguments
    /// - `solver`: The [`Solver`] to solve [`Sudoku`]s with.
    /// - `step_time`: The timeout in between steps to press 'Q'.
    /// 
    /// # Returns
    /// A new instance of Self.
    /// 
    /// # Errors
    /// This function may error if we failed to setup a new terminal UI.
    pub fn new(solver: S, step_time: Duration) -> Result<Self, Error> {
        // Enable terminal raw mode
        if let Err(err) = enable_raw_mode() {
            return Err(Error::RawModeEnable { err });
        }
        // Set the terminal into the alternate screen
        let mut stdout: Stdout = io::stdout();
        if let Err(err) = execute!(stdout, EnterAlternateScreen) {
            if let Err(err) = disable_raw_mode() { warn!("Failed to disable terminal raw mode: {err}"); }
            return Err(Error::AlternateScreenEnable { err });
        }

        // Create a new terminal
        let term: Terminal<_> = match Terminal::new(CrosstermBackend::new(stdout)) {
            Ok(term) => term,
            Err(err) => {
                if let Err(err) = disable_raw_mode() { warn!("Failed to disable terminal raw mode: {err}"); }
                if let Err(err) = execute!(io::stdout(), LeaveAlternateScreen) { warn!("Failed to leave alternate screen mode: {err}"); }
                return Err(Error::TerminalInitialize { err });
            },
        };

        // We can finally construct ourselves!
        Ok(Self {
            timeout : step_time,
            solver,

            term,
        })
    }
}
impl<S> Drop for Engine<S> {
    fn drop(&mut self) {
        // Reverse the raw mode
        if let Err(err) = disable_raw_mode() { warn!("Failed to disable terminal raw mode: {err}"); }
        if let Err(err) = execute!(io::stdout(), LeaveAlternateScreen) { warn!("Failed to leave alternate screen mode: {err}"); }
        if let Err(err) = self.term.show_cursor() { warn!("Failed to show terminal cursor: {err}"); }
    }
}

impl<S: Solver> Engine<S> {
    /// Solves a Sudoku, showing each step in the UI
    /// 
    /// # Arguments
    /// - `sudokus`: Any sudokus to solve, as a list of `(<name>, <sudoku>)` pairs. If the list is empty, will query the user instead.
    /// 
    /// # Returns
    /// The solved sudoku's of the input (or else best-effort). Matches the input indices. May be shorter than the input list if the process was cancelled halfway through.
    /// 
    /// # Errors
    /// This function may error if there was some error while running.
    pub fn solve(&mut self, sudokus: impl AsRef<[(String, Sudoku)]>) -> Result<Vec<Sudoku>, Error> {
        let sudokus: &[(String, Sudoku)] = sudokus.as_ref();

        // The game loop, as it were
        let mut solutions: Vec<Sudoku> = Vec::with_capacity(sudokus.len());
        for (name, sudoku) in sudokus {
            // Run the solver, updating the UI at the end of every run
            let solution: Option<Sudoku> = self.solver.run_with_callback(*sudoku, |sudoku: &Sudoku| -> Result<bool, Error> {
                // Draw the current state
                if let Err(err) = self.term.draw(|frame: &mut Frame<CrosstermBackend<Stdout>>| {
                    let title = Paragraph::new(format!("Solving sudoku '{name}'...\n(Press 'Q' to cancel)\n\n{sudoku}"));
                    frame.render_widget(title, frame.size());
                    // frame.render_widget(title, Rect { x: 0, y: 0, width: frame.size().width, height: frame.size().height / 8 });
                    // frame.render_widget(sudoku.render(), Rect { x: 0, y: frame.size().height / 8, width: frame.size().width, height: frame.size().height - frame.size().height / 8 });
                }) {
                    return Err(Error::FrameDraw { err });
                };
    
                // Check for key presses
                Ok(!should_quit(self.timeout)?)
            })?;

            // Add it if we have any, else quit
            match solution {
                Some(solution) => { solutions.push(solution); },
                None           => { return Ok(solutions); },
            }
        }

        // Done!
        Ok(solutions)
    }
}
