use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::io::{StdinLock, StdoutLock};
use std::path::Path;
use std::process;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use anyhow::{Context, Result};

use crate::ExitCode;
use crate::Opt;
use crate::state::State;

/// quickly prints the contents of a file verbatim
pub fn fast_print<P: AsRef<Path>>(path: P, handle: &mut StdoutLock) -> Result<()> {
    let mut f =
        File::open(&path).context(format!("`{}` is not a file", path.as_ref().display()))?;
    io::copy(&mut f, handle)?;

    Ok(())
}

/// prints the content of a file and inserts different strings
/// depending on command line options
pub fn print_insert<P: AsRef<Path>>(
    path: P,
    handle: &mut StdoutLock,
    opt: &Opt,
    wants_to_quit: &Arc<AtomicBool>,
) -> Result<()> {
    let mut buf = String::new();
    let mut insert_buf = if opt.is_number_option() {
        Some(String::new())
    } else {
        None
    };
    let mut supress = false;

    let mut f = BufReader::new(
        File::open(&path).context(format!("`{}` is not a file", path.as_ref().display()))?,
    );
    let mut line = 1;
    while let Some(_) = f
        .read_line(&mut buf)
        .map(|u| if u == 0 { None } else { Some(u) })
        .context(format!(
            "Could not read line {} from {}",
            line,
            path.as_ref().display()
        ))?
    {
        change_string(&mut buf, &mut insert_buf, &mut supress, line, &opt);
        handle.write_all(buf.as_bytes())?;

        // must be before the buffer is cleared
        line = calculate_next_line(&buf, &opt, line);
        buf.clear();
    }

    Ok(())
}

/// change string based on command line options
fn change_string(s: &mut String, insert_buf: &mut Option<String>, suppress: &mut bool, line: u32, opt: &Opt) {
    if opt.show_ends {
        s.insert_str(s.len() - 1, "$");
    }

    if opt.show_tabs {
        if s.starts_with("\t") {
            *s = s.chars().skip(1).collect();
            s.insert_str(0, "^I");
        }
    }

    if opt.number_nonblank {
        if !s.trim().is_empty() {
            number(
                insert_buf
                    .as_mut()
                    .expect("Insert must be some if used with a number option"),
                s,
                line,
            );
        }
    } else if opt.number {
        number(
            insert_buf
                .as_mut()
                .expect("Insert must be some if used with a number option"),
            s,
            line,
        );
    } else if opt.squeeze_blank {
        if s.trim().is_empty() {
            if *suppress == true {
                s.clear();
                // *s = String::from("\n");
            }
            *suppress = true;
        } else {
            *suppress = false;
        }
    }
}

fn number(insert: &mut String, s: &mut String, line: u32) {
    *insert = format!("{:>5}  ", line);
    s.insert_str(0, &insert);
}

/// get input from stdin and the echo it back to stdout
/// it will add different strings to the output based on
/// command line options
pub fn echo(
    stdin_handle: &mut StdinLock,
    stdout_handle: &mut StdoutLock,
    opt: &Opt,
    wants_to_quit: &Arc<AtomicBool>,
) -> Result<()> {
    let mut buf = String::new();
    let mut insert_buf = if opt.is_number_option() {
        Some(String::new())
    } else {
        None
    };
    let mut suppress = false;
    let mut line = 1;

    loop {
        stdin_handle
            .read_line(&mut buf)
            .context("Failed to read a line from stdin")?;

        change_string(&mut buf, &mut insert_buf, &mut suppress, line, opt);

        stdout_handle
            .write_all(buf.as_bytes())
            .context("Failed to write to stdout")?;

        // do this before we clear the buffer so we can we can
        // check if it is really empty
        line = calculate_next_line(&buf, &opt, line);
        buf.clear();
    }
}

/// add one to the current line if number is true
/// if number_nonblank is true, it will only add numbers if
/// the line is not empty
fn calculate_next_line(s: &String, opt: &Opt, current_line: u32) -> u32 {
    if opt.number_nonblank {
        if !s.trim().is_empty() {
            current_line + 1
        } else {
            current_line
        }
    } else if opt.number {
        current_line + 1
    } else {
        current_line
    }
}
