mod cat;
mod error;
mod exit_codes;
mod opt;
mod state;

use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use anyhow::{anyhow, Result};
use structopt::StructOpt;

use cat::{fast_print, print_insert, echo};
use error::print_err;
use exit_codes::ExitCode;
use opt::Opt;

fn try_main() -> Result<()> {
    let wants_to_quit = Arc::new(AtomicBool::new(false));
    let wq = Arc::clone(&wants_to_quit);
    ctrlc::set_handler(move || {
        process::exit(ExitCode::KilledBySigint.into());
        // if wq.load(Ordering::Relaxed) {
        //     // Ctrl-C has been pressed twice, exit NOW
        //     process::exit(ExitCode::KilledBySigint.into());
        // } else {
        //     wq.store(true, Ordering::Relaxed);
        // }
    })
    .map_err(|e| anyhow!("{}\n\nError setting ctrl-c handler.", e.to_string()))?;

    let opt = Opt::from_args();
    dbg!(&opt);
    

    let stdout = stdout();
    let stdin = stdin();
    let mut stdout_handle = stdout.lock();
    let mut stdin_handle = stdin.lock();

    if opt.files.is_empty() {
        echo(&mut stdin_handle, &mut stdout_handle, &opt, &wants_to_quit)?;
    } else {
        for file in &opt.files {
            if file == Path::new("-") {
                echo(&mut stdin_handle, &mut stdout_handle, &opt, &wants_to_quit)?;
            } else if opt.should_use_fast_print() {
                fast_print(file, &mut stdout_handle)?;
            } else {
                print_insert(file, &mut stdout_handle, &opt, &wants_to_quit)?;
            }
        }
    }

    Ok(())
}

fn main() {
    match try_main() {
        Ok(()) => process::exit(ExitCode::Success.into()),
        Err(e) => {
            print_err(&format!("{}", e));
            process::exit(ExitCode::GeneralError.into())
        }
    }
}
