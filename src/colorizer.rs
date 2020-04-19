use crossterm::style::{Color, Attribute, Print, ResetColor, SetBackgroundColor, SetForegroundColor, SetAttribute};
use crossterm::QueueableCommand;
use crossterm::Result;

use serde::{Deserialize, de::Deserializer};
use ron::de;

use crate::token::Token;

#[derive(Debug, Deserialize)]
pub enum Style {
    Fg(Color),
    Bg(Color),
    Attr(Attribute),
}

#[derive(Debug)]
pub struct StyleBundle(Vec<Style>);

impl<'de> Deserialize<'de> for StyleBundle {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Vec::deserialize(deserializer).map(|v| StyleBundle(v))
    }
}

impl std::str::FromStr for StyleBundle {
    type Err = usize;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        de::from_str(s).map_err(|e|{
            eprintln!("{}", e);
            1 as usize
        })
    }
}

pub struct Colorizer {
    colors: Vec<StyleBundle>,
}

impl Colorizer {
    pub fn new(colors: Vec<StyleBundle>) -> Self {
        Self {
            colors,
        }
    }

    fn stylize<T>(&self, color_index: usize, s: String, queue: &mut T) -> Result<usize>
    where T: std::io::Write {
        if let Some(color) = self.colors.get(color_index % self.colors.len()) {
            let (mut colored, mut attributed) = (false, false);
            for style in &color.0 {
                match style {
                    Style::Fg(c) => {
                        queue.queue(SetForegroundColor(*c))?;
                        colored = true;
                    },
                    Style::Bg(c) => {
                        queue.queue(SetBackgroundColor(*c))?;
                        colored = true;
                    },
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
    where T: std::io::Write {
        match t {
            Token::Matched(v, s) => self.stylize(v, s, queue),
            Token::Unmatched(s) => {
                let len = s.len();
                queue.queue(Print(s))?;
                Ok(len)
            },
            Token::Undecided(_) => unreachable!(),
        }
    }
}
