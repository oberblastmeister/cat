use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::io::{StdoutLock, StdinLock};
use std::path::Path;

use anyhow::{Context, Result};

use super::Opt;

/// quickly prints the contents of a file
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
) -> Result<()> {
    let mut buf = String::new();
    let mut insert = if opt.number {
        Some(String::new())
    } else {
        None
    };

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
        add_to_string(&mut buf, &mut insert, line, &opt);
        handle.write_all(buf.as_bytes())?;
        buf.clear();

        if opt.number {
            line += 1;
        }
    }

    Ok(())
}

/// change string based on command line options
fn add_to_string(
    s: &mut String,
    insert: &mut Option<String>,
    line: u32,
    opt: &Opt,
) {
    if opt.show_ends {
        s.insert_str(s.len() - 1, "$");
    }

    if opt.show_tabs {
        if s.starts_with("\t") {
            *s = s.chars().skip(1).collect();
            s.insert_str(0, "^I");
        }
    }

    if opt.number {
        let insert = insert.as_mut().expect("Insert must be supplied if using number option");
        *insert = format!("{:>5}  ", line);
        s.insert_str(0, &insert);
    }
}

/// get input from stdin and the echo it back to stdout
/// it will add different strings to the output based on
/// command line options
pub fn echo(
    stdin_handle: &mut StdinLock,
    stdout_handle: &mut StdoutLock,
    opt: &Opt,
) -> Result<()> {
    let mut buf = String::new();
    let mut insert = if opt.number {
        Some(String::new())
    } else {
        None
    };
    let mut line = 1;

    loop {
        stdin_handle
            .read_line(&mut buf)
            .context("Failed to read a line from stdin")?;

        add_to_string(&mut buf, &mut insert, line, opt);

        stdout_handle
            .write_all(buf.as_bytes())
            .context("Failed to write to stdout")?;

        buf.clear();

        if opt.number {
            line += 1;
        }
    }
}
