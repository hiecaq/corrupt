use std::io::BufRead;
use std::collections::VecDeque;
use regex::Regex;

pub enum Token {
    Matched(usize, String),
    Unmatched(String),
    Undecided(String),
}

pub struct TokenStream<R: BufRead> {
    input: R,
    cache: VecDeque<Token>,
    regs: Vec<Regex>,
}

impl<R: BufRead> TokenStream<R> {
    pub fn new(input: R, regs: Vec<Regex>) -> Self {
        Self {
            input,
            cache: VecDeque::new(),
            regs,
        }
    }

    fn earliest_match<'a>(&self, input: &'a str) -> Option<(usize, &'a str, &'a str, &'a str)> {
        let output = self.regs.iter().enumerate()
            .filter_map(|(n, re)| re.find(input).map(|m| (m.start(), m.end(), n)))
            .min();
        output.map(|(start, end, color)| {
            (color, &input[..start], &input[start..end], &input[end..])
        })
    }

    fn parse_string(&mut self, s: &str) -> Option<Token> {
        let output = self.earliest_match(s);
        if let Some((color, left, mid, right)) = output {
            if left.is_empty() {
                self.cache.push_back(Token::Undecided(right.to_string()));
                Some(Token::Matched(0, mid.to_string()))
            } else {
                self.cache.push_back(Token::Matched(color, mid.to_string()));
                self.cache.push_back(Token::Undecided(right.to_string()));
                Some(Token::Unmatched(left.to_string()))
            }
        } else {
            Some(Token::Unmatched(s.to_string()))
        }
    }
}

impl<R: BufRead> Iterator for TokenStream<R> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        let s = {
            match self.cache.pop_front() {
                t @ Some(Token::Matched(..)) | t @ Some(Token::Unmatched(_)) => return t,
                Some(Token::Undecided(s)) => s,
                None => {
                    let mut s: String = String::new();
                    match self.input.read_line(&mut s) {
                        Err(e) => Err(e).unwrap(),
                        Ok(0) => return None,
                        Ok(_) => (),
                    }
                    s
                }
            }
        };

        self.parse_string(&s)
    }
}
