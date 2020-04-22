use crossterm::style::{
    Attribute, Color, Print, ResetColor, SetAttribute, SetBackgroundColor, SetForegroundColor,
};
use crossterm::QueueableCommand;

use crate::error::Result;

use crate::style::{Style, StyleBundle};
use crate::token::Token;

pub struct Stylizer {
    colors: Vec<StyleBundle>,
}

impl Stylizer {
    pub fn new(colors: Vec<StyleBundle>) -> Self {
        Self { colors }
    }

    fn stylize<T>(&self, color_index: usize, s: String, queue: &mut T) -> Result<usize>
    where
        T: std::io::Write,
    {
        if let Some(color) = self.colors.get(color_index % self.colors.len()) {
            let (mut colored, mut attributed) = (false, false);
            for style in &color.0 {
                match style {
                    Style::Fg(c) => {
                        queue.queue(SetForegroundColor(*c))?;
                        colored = true;
                    }
                    Style::Bg(c) => {
                        queue.queue(SetBackgroundColor(*c))?;
                        colored = true;
                    }
                    Style::Attr(a) => {
                        queue.queue(SetAttribute(*a))?;
                        attributed = true;
                    }
                };
            }

            let len = s.len();

            queue.queue(Print(s))?;

            if attributed {
                queue.queue(SetAttribute(Attribute::Reset))?;
            }

            if colored {
                queue.queue(ResetColor)?;
            }

            Ok(len)
        } else {
            unreachable!()
        }
    }

    pub fn forward<T>(&self, t: Token, queue: &mut T) -> Result<usize>
    where
        T: std::io::Write,
    {
        match t {
            Token::Matched(v, s) => self.stylize(v, s, queue),
            Token::Unmatched(s) => {
                let len = s.len();
                queue.queue(Print(s))?;
                Ok(len)
            }
            Token::Undecided(_) => unreachable!(),
        }
    }
}
