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

impl Opt {
    pub fn new_with_equivalent_options() -> Self {
        let mut opt = Opt::from_args();
        opt.show_all_equivalent();
        opt.show_ends_and_nonprinting_equivalent();
        opt
    }

    pub fn should_use_fast_print(&self) -> bool {
        !(self.number
            || self.show_ends
            || self.show_tabs
            || self.show_nonprinting
            || self.show_ends_and_nonprinting
            || self.number_nonblank
            || self.show_all
            || self.squeeze_blank)
    }

    pub fn is_number_option(&self) -> bool {
        self.number || self.number_nonblank
    }

    fn show_all_equivalent(&mut self) {
        if self.show_all {
            self.show_nonprinting = true;
            self.show_ends = true;
            self.show_tabs = true;
        }
    }
    
    fn show_ends_and_nonprinting_equivalent(&mut self) {
        if self.show_ends_and_nonprinting {
            self.show_nonprinting = true;
            self.show_ends = true;
        }
    }
}
