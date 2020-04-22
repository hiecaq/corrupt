use std::io::{stdin, stdout, Write};

use corrupt::cli::{Opt, StructOpt};
use corrupt::error::Result;
use corrupt::stylizer::Stylizer;
use corrupt::token::TokenStream;

fn main() -> Result<()> {
    let opt = Opt::from_args();

    let stdin = stdin();
    let tokens = TokenStream::new(stdin.lock(), opt.regs);

    let stylizer = Stylizer::new(opt.style);

    let mut stdout = stdout();
    let mut cnt = 0;
    for token in tokens {
        cnt += stylizer.forward(token, &mut stdout)?;

        if cnt >= opt.flush_thd {
            cnt = 0;
            stdout.flush()?;
        }
    }
    stdout.flush()?;

    Ok(())
}
