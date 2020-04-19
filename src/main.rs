use std::io::{stdin, stdout, Write};
use crossterm::style::{Colorize, Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor, PrintStyledContent};
use crossterm::QueueableCommand;
use crossterm::Result;
use regex::Regex;
use structopt::StructOpt;

use corrupt::token::{Token, TokenStream};
use corrupt::colorizer::{StyleBundle, Colorizer};

/// A minimal stdin/out filter that highlight regex-matched substrings that use colors with round-robin fashion.
#[derive(StructOpt, Debug)]
#[structopt(name = "corrupt")]
struct Opt {
    #[structopt(short, long, default_value="[Fg(Red)]")]
    style: Vec<StyleBundle>,
    /// Regexps that are stylized.
    #[structopt(name = "REGEX", parse(try_from_str))]
    regs: Vec<Regex>,
}

const FLUSH_THD: usize = 512;

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let stdin = stdin();
    let tokens = TokenStream::new(stdin.lock(), opt.regs);
    let colorizer = Colorizer::new(opt.style);

    let mut stdout = stdout();
    let mut cnt = 0;
    for token in tokens {
        cnt += colorizer.forward(token, &mut stdout)?;

        if cnt >= FLUSH_THD {
            cnt = 0;
            stdout.flush()?;
        }
    }
    stdout.flush()?;

    Ok(())
}
