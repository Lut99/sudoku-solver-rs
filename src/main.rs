//  MAIN.rs
//    by Lut99
// 
//  Created:
//    10 Aug 2023, 23:01:37
//  Last edited:
//    11 Aug 2023, 00:43:01
//  Auto updated?
//    Yes
// 
//  Description:
//!   Entrypoint to the sudoku solver.
// 

use std::fs::File;
use std::path::PathBuf;

use clap::Parser;
use humanlog::{DebugMode, HumanLogger};
use log::error;

use sudoku_solver::engine::Engine;
use sudoku_solver::spec::{FileType, Sudoku};
use sudoku_solver::utils::PrettyError as _;


/***** ARGUMENTS *****/
/// Defines the arguments for the sudoku solver.
#[derive(Debug, Parser)]
#[clap(name = "sudoku_solver", about = "A solver for Sudoku's.")]
struct Arguments {
    /// Whether to load from a file or not.
    #[clap(name="FILES", help="If given, loads the Sudoku from the given file instead of querying the user. Check '--file-type' to change the default file type.")]
    files : Vec<PathBuf>,

    /// Determines the type of the loaded file.
    #[clap(short, long, default_value="json", help="Determines the type of the file to load. Will be ignored if no file is given.")]
    file_type : FileType,
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
        let sudoku: Sudoku = match args.file_type {
            FileType::Json => {
                // Open the file
                let handle: File = match File::open(&sudoku_path) {
                    Ok(handle) => handle,
                    Err(err)   => { error!("Failed to open sudoku file '{}' as {}: {}", sudoku_path.display(), args.file_type, err); std::process::exit(1); },
                };

                // Parse it
                let sudoku: Sudoku = match serde_json::from_reader(handle) {
                    Ok(sudoku) => sudoku,
                    Err(err)   => { error!("Failed to parse sudoku file '{}' as {}: {}", sudoku_path.display(), args.file_type, err); std::process::exit(1); },
                };

                // Discard it if not well-formed
                if !sudoku.is_well_formed() { error!("Sudoku in file '{}' is not properly formed", sudoku_path.display()); std::process::exit(1); }

                // Done
                println!("Loaded Sudoku file '{}' as {}", sudoku_path.display(), args.file_type);
                sudoku
            },
        };

        // Add it to the list
        sudokus.push((sudoku_path.display().to_string(), sudoku));
    }

    // Spin up the terminal UI
    let mut ui: Engine<_> = match Engine::new(sudoku_solver::solvers::BruteForceSolver::new()) {
        Ok(ui)   => ui,
        Err(err) => { error!("{}", err.pretty()); std::process::exit(1); },
    };

    // Query for sudoku's if not given
    /* TODO */

    // Run the program
    if let Err(err) = { ui.solve(sudokus) } {
        error!("Failed to solve Sudoku: {}", err.pretty());
        std::process::exit(1);
    };

    // Done!
}
