use std::path::PathBuf;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
/// A cat clone written in rust
///
/// Concatenate FILES(s) to standard output.
///
/// With not FILE, or when FILE is -, read standard input.
pub struct Opt {
    /// equivalent to -vET
    #[structopt(short = "A", long = "show-all")]
    pub show_all: bool,

    /// number nonempty output lines, overrides -n
    #[structopt(short = "b", long = "number-nonblank")]
    pub number_nonblank: bool,

    /// equivalent to -vE
    #[structopt(short = "e")]
    pub show_ends_and_nonprinting: bool,

    /// display $ at end of each line
    #[structopt(short = "E", long = "show-ends")]
    pub show_ends: bool,

    /// number all output lines
    #[structopt(short, long)]
    pub number: bool,

    /// suppress repeated empty output lines
    #[structopt(short, long = "squeeze-blank")]
    pub squeeze_blank: bool,

    /// display TAB characters as ^I
    #[structopt(short = "T", long = "show-tabs")]
    pub show_tabs: bool,

    /// (ignored)
    #[structopt(short = "u")]
    pub ignored: bool,

    /// use ^ and M- notation, except for LFD and TAB
    #[structopt(short = "v", long = "show-nonprinting")]
    pub show_nonprinting: bool,

    #[structopt(name = "FILE")]
    #[structopt(parse(from_os_str))]
    pub files: Vec<PathBuf>,
}
