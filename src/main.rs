mod cat;
mod error;
mod opt;

use std::io::{stdin, stdout};
use std::path::Path;

use anyhow::Result;
use structopt::StructOpt;

use cat::*;
use error::print_err;
use opt::Opt;

fn try_main() -> Result<()> {
    let opt = Opt::from_args();

    let stdout = stdout();
    let stdin = stdin();
    let mut stdout_handle = stdout.lock();
    let mut stdin_handle = stdin.lock();

    if opt.files.is_empty() {
        echo(
            &mut stdin_handle,
            &mut stdout_handle,
            &opt,
        )?;
    } else {
        for file in &opt.files {
            if file == Path::new("-") {
                echo(
                    &mut stdin_handle,
                    &mut stdout_handle,
                    &opt,
                )?;
            } else if !(opt.number || opt.show_ends || opt.show_tabs) {
                fast_print(file, &mut stdout_handle)?;
            } else {
                print_insert(
                    file,
                    &mut stdout_handle,
                    &opt,
                )?;
            }
        }
    }

    Ok(())
}

fn main() {
    match try_main() {
        Ok(()) => (),
        Err(e) => print_err(&format!("{}", e)),
    }
}
