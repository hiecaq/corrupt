//! command line interface

use crate::style::StyleBundle;
use regex::Regex;
pub use structopt::StructOpt;

/// A minimal stdin/out filter that highlight regex-matched substrings that use colors with round-robin fashion.
#[derive(StructOpt, Debug)]
#[structopt(name = "corrupt")]
pub struct Opt {
    #[structopt(short, long, default_value = "[Fg(Red)]")]
    pub style: Vec<StyleBundle>,
    /// Regexps that are stylized.
    #[structopt(name = "REGEX", parse(try_from_str))]
    pub regs: Vec<Regex>,
    /// how many bytes to read before attempting to flush
    #[structopt(short, long, default_value = "512")]
    pub flush_thd: usize,
}
