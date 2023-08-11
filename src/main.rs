//  MAIN.rs
//    by Lut99
// 
//  Created:
//    10 Aug 2023, 23:01:37
//  Last edited:
//    11 Aug 2023, 23:38:21
//  Auto updated?
//    Yes
// 
//  Description:
//!   Entrypoint to the sudoku solver.
// 

use std::path::PathBuf;
use std::time::Duration;

use clap::Parser;
use console::Style;
use humanlog::{DebugMode, HumanLogger};
use log::error;

use sudoku_solver::engine::Engine;
use sudoku_solver::solvers::{BruteForceSolver, Solver as _};
use sudoku_solver::spec::FileType;
use sudoku_solver::sudoku::Sudoku;
use sudoku_solver::utils::{load_sudoku, load_sudoku_of_type, PrettyError as _};


/***** ARGUMENTS *****/
/// Defines the arguments for the sudoku solver.
#[derive(Debug, Parser)]
#[clap(name = "sudoku_solver", about = "A solver for Sudoku's.")]
struct Arguments {
    /// Whether to load from a file or not.
    #[clap(name="FILES", help="If given, loads the Sudoku from the given file instead of querying the user. Check '--file-type' to change the default file type.")]
    files : Vec<PathBuf>,

    /// If given, does not show the final version but instead shows only the solutions to the `n` first cells.
    #[clap(long, help="If given, does not show the final version but instead shows only the solutions to the given number of first empty cells.")]
    hint       : Option<u8>,
    /// Determines the type of the loaded file.
    #[clap(short='t', long, help="Overrides deriving the input file type with this fixed type instead. Note that this applies to ALL input files. Will be ignored if no file is given.")]
    input_type : Option<FileType>,
    /// Determines the timout in between steps (in ms).
    #[clap(short='T', long, default_value="50", help="The timeout in between compute steps, for visualisation purposes.")]
    timeout    : u64,
    /// Runs the solver without UI. Note that you cannot select files this way.
    #[clap(long, help="If given, runs without UI at maximum speed. Note that you cannot insert a Sudoku yourself this way.")]
    headless   : bool,
}





/***** ENTRYPOINT *****/
fn main() {
    // Parse the arguments
    let args: Arguments = Arguments::parse();

    // Enable the logger
    if let Err(err) = HumanLogger::terminal(DebugMode::HumanFriendly).init() {
        eprintln!("WARNING: Failed to setup logger: {err} (no logging enabled for this session)");
    }

    // Load the Sudokus, if any
    let mut sudokus: Vec<(String, Sudoku)> = Vec::with_capacity(args.files.len());
    for sudoku_path in args.files {
        // Attempt to load it according to our method
        println!("Loading Sudoku '{}'...", sudoku_path.display());
        let mut fsudokus: Vec<Sudoku> = if let Some(ftype) = args.input_type {
            match load_sudoku_of_type(&sudoku_path, ftype) {
                Ok(sudoku) => sudoku,
                Err(err)   => { error!("Failed to load sudoku file '{}' as {}: {}", sudoku_path.display(), ftype, err); std::process::exit(1); },
            }
        } else {
            match load_sudoku(&sudoku_path) {
                Ok(sudoku) => sudoku,
                Err(err)   => { error!("Failed to load sudoku file '{}': {}", sudoku_path.display(), err); std::process::exit(1); },
            }
        };

        // Add it to the list
        if fsudokus.len() == 1 {
            sudokus.push((sudoku_path.display().to_string(), fsudokus.swap_remove(0)));
        } else {
            sudokus.extend(fsudokus.into_iter().enumerate().map(|(i, s)| (format!("{} ({})", sudoku_path.display(), i + 1), s)));
        }
    }
    println!();

    // Now either run with UI or without.
    if !args.headless {
        /* With UI */

        // Start the terminal UI
        let mut ui: Engine<_> = match Engine::new(sudoku_solver::solvers::BruteForceSolver::new(), Duration::from_millis(args.timeout)) {
            Ok(ui)   => ui,
            Err(err) => { error!("{}", err.pretty()); std::process::exit(1); },
        };

        // Query for sudoku's if not given
        /* TODO */

        // Run the program
        let solutions: Vec<Sudoku> = match ui.solve(&sudokus) {
            Ok(sudokus) => sudokus,
            Err(err)    => {
                error!("Failed to solve Sudokus: {}", err.pretty());
                std::process::exit(1);
            }
        };

        // Show the results of the Sudokus

    } else {
        /* Without UI */

        // Assert we have sudokus
        if sudokus.is_empty() {
            println!("No Sudokus given; nothing to do.");
            std::process::exit(0);
        }

        // Start the solver
        let mut solver: BruteForceSolver = BruteForceSolver::new();
        let solutions: Vec<Sudoku> = sudokus.iter().map(|s| {
            println!("Solving Sudoku '{}'...", s.0);
            solver.run(s.1)
        }).collect();
        println!();
    
        // Write it to the terminal
        if let Some(n_hints) = args.hint {
            for (i, solution) in solutions.into_iter().enumerate() {
                println!("Hint to Sudoku '{}':", sudokus[i].0);

                // Find the first N slots in the mask and add the solutions from the solved sudoku
                let mut hint: Sudoku = sudokus[i].1;
                let mut j: usize = 0;
                'main: for y in 0..9 {
                    for x in 0..9 {
                        // Quit if we exceeded the number of requested hints
                        if j >= n_hints as usize { break 'main; }

                        // If the hint is empty, add in the thing
                        if hint.rows[y][x].is_none() {
                            hint.rows[y][x] = solution.rows[y][x];
                            j += 1;
                        }
                    }
                }

                // Show the hint
                println!("{}", hint.masked(&sudokus[i].1).colour(Style::new().green().bold()));
            }
        } else {
            for (i, solution) in solutions.into_iter().enumerate() {
                println!("Solution to Sudoku '{}':", sudokus[i].0);
                println!("{}", solution.masked(&sudokus[i].1));
            }
        }
    }

    // Done!
}
