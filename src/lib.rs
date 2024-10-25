use regex::{Regex, Match};
use thiserror::Error;

pub struct Lexer<'l, T> {
    rules: Vec<Rule<'l, T>>
}

pub type AddRuleResult<'l, 'a, T> = Result<&'a mut Lexer<'l, T>, regex::Error>;

impl<'l, T> Lexer<'l, T> {
    pub fn new() -> Lexer<'l, T> {
        Lexer { rules: Vec::new() }
    }

    pub fn rule(&mut self, pattern: impl AsRef<str>, rule: impl Fn(Match) -> T + 'l) -> AddRuleResult<'l, '_, T> {
        self.rules.push(Rule {
            pattern: Regex::new(&add_start_flag(pattern))?,
            func: Box::new(move |mat| Some(rule(mat)))
        });
        Ok(self)
    }

    pub fn rule_option(&mut self, pattern: impl AsRef<str>, rule: impl Fn(Match) -> Option<T> + 'l) -> AddRuleResult<'l, '_, T> {
        self.rules.push(Rule {
            pattern: Regex::new(&add_start_flag(pattern))?,
            func: Box::new(rule)
        });
        Ok(self)
    }

    pub fn rule_noop(&mut self, pattern: impl AsRef<str>) -> AddRuleResult<'l, '_, T> {
        self.rules.push(Rule {
            pattern: Regex::new(&add_start_flag(pattern))?,
            func: Box::new(|_| None)
        });
        Ok(self)
    }

    pub fn tokenize<'i>(&'i self, string: &'i impl AsRef<str>) -> LexerIter<'i, T> {
        LexerIter {
            rules: &self.rules,
            full_string: string.as_ref(),
            pos: 0,
            failed: false
        }
    }
}

impl<'l, T: 'l + Clone> Lexer<'l, T> {
    pub fn rule_simple(&mut self, pattern: impl AsRef<str>, token: T) -> AddRuleResult<'l, '_, T> {
        self.rules.push(Rule {
            pattern: Regex::new(&add_start_flag(pattern))?,
            func: Box::new(move |_| Some(token.clone()))
        });
        Ok(self)
    }
}

struct Rule<'r, T> {
    pattern: Regex,
    func: Box<dyn Fn(Match) -> Option<T> + 'r>
}

pub struct LexerIter<'i, T> {
    rules: &'i [Rule<'i, T>],
    full_string: &'i str,
    pos: usize,
    failed: bool
}

impl<'i, T> Iterator for LexerIter<'i, T> {
    type Item = TokenizeResult<T>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.failed || self.pos >= self.full_string.len() {
                return None;
            }

            let string = &self.full_string[self.pos..];
            let mut matched_noop = false;

            for rule in self.rules {
                if let Some(mat) = rule.pattern.find(string) {
                    #[cfg(debug_assertions)]
                    assert_eq!(mat.start(), 0);

                    self.pos += mat.end();

                    if let Some(token) = (rule.func)(mat) {
                        return Some(Ok(token));
                    } else {
                        matched_noop = true;
                    }
                    break;
                }
            }

            if !matched_noop {
                self.failed = true;
                return Some(Err(TokenizeError::UnexpectedChar {
                    ch: string.chars().next().unwrap(),
                    position: self.pos
                }));
            }
        }
    }
}

#[derive(Error, Debug)]
pub enum TokenizeError {
    #[error("no rules matched character '{ch:?}' at position {position:?}")]
    UnexpectedChar {
        ch: char,
        position: usize
    }
}

pub type TokenizeResult<T> = Result<T, TokenizeError>;




fn add_start_flag(pattern: impl AsRef<str>) -> String {
    format!("^{}", pattern.as_ref())
}