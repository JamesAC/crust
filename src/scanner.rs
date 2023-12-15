use crust_grammar::token::Token;
use std::str::FromStr;

use crate::util::{CrustCoreErr, CrustCoreResult};

pub struct Scanner<'a> {
    source: &'a str,

    start: usize,
    current: usize,
    line: usize,

    tokens: Vec<Token>,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            start: 0,
            current: 0,
            line: 1,
            tokens: vec![],
        }
    }

    pub fn scan_tokens(mut self) -> CrustCoreResult<Vec<Token>> {
        let mut errors: Vec<CrustCoreErr> = vec![];
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token(&mut errors);
        }

        self.tokens.push(Token::Eof {
            offset: self.current,
            line: self.line,
        });

        if !errors.is_empty() {
            Err(CrustCoreErr::Multi { errors })
        } else {
            Ok(self.tokens)
        }
    }

    fn scan_token(&mut self, errors: &mut Vec<CrustCoreErr>) {
        let char = self.advance();
        match char {
            '(' => self.tokens.push(Token::LeftParen {
                offset: self.current - 1,
                line: self.line,
            }),
            ')' => self.tokens.push(Token::RightParen {
                offset: self.current - 1,
                line: self.line,
            }),
            '{' => self.tokens.push(Token::LeftBrace {
                offset: self.current - 1,
                line: self.line,
            }),
            '}' => self.tokens.push(Token::RightBrace {
                offset: self.current - 1,
                line: self.line,
            }),
            ',' => self.tokens.push(Token::Comma {
                offset: self.current - 1,
                line: self.line,
            }),
            '.' => self.tokens.push(Token::Dot {
                offset: self.current - 1,
                line: self.line,
            }),
            '-' => self.tokens.push(Token::Minus {
                offset: self.current - 1,
                line: self.line,
            }),
            '+' => self.tokens.push(Token::Plus {
                offset: self.current - 1,
                line: self.line,
            }),
            ';' => self.tokens.push(Token::Semicolon {
                offset: self.current - 1,
                line: self.line,
            }),
            '*' => self.tokens.push(Token::Star {
                offset: self.current - 1,
                line: self.line,
            }),
            '!' if self.advance_if('=') => {
                self.tokens.push(Token::BangEqual {
                    offset: self.current - 2,
                    line: self.line,
                });
            }
            '!' => {
                self.tokens.push(Token::Bang {
                    offset: self.current - 1,
                    line: self.line,
                });
            }
            '=' if self.advance_if('=') => {
                self.tokens.push(Token::EqualEqual {
                    offset: self.current - 2,
                    line: self.line,
                });
            }
            '=' => {
                self.tokens.push(Token::Equal {
                    offset: self.current - 1,
                    line: self.line,
                });
            }
            '<' if self.advance_if('=') => {
                self.tokens.push(Token::LessEqual {
                    offset: self.current - 2,
                    line: self.line,
                });
            }
            '<' => {
                self.tokens.push(Token::Less {
                    offset: self.current - 1,
                    line: self.line,
                });
            }
            '>' if self.advance_if('=') => {
                self.tokens.push(Token::GreaterEqual {
                    offset: self.current - 2,
                    line: self.line,
                });
            }
            '>' => {
                self.tokens.push(Token::Greater {
                    offset: self.current - 1,
                    line: self.line,
                });
            }
            '/' => {
                if self.advance_if('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.tokens.push(Token::Slash {
                        offset: self.current - 1,
                        line: self.line,
                    });
                }
            }
            '0'..='9' => {
                if let Err(e) = self.take_number_literal() {
                    errors.push(e);
                }
            }
            ' ' | '\t' | '\r' => {}
            '\n' => self.line += 1,
            '\"' => {
                if let Err(e) = self.take_string_literal() {
                    errors.push(e);
                }
            }
            _ => errors.push(CrustCoreErr::Scan {
                line: self.line,
                message: "Unexpected character".to_string(),
            }),
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.char_at(self.current - 1)
    }

    fn advance_if(&mut self, pattern: char) -> bool {
        if self.is_at_end() || self.char_at(self.current) != pattern {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.char_at(self.current)
        }
    }

    fn char_at(&self, index: usize) -> char {
        self.source[index..index + 1].chars().next().unwrap()
    }

    fn take_string_literal(&mut self) -> CrustCoreResult {
        while self.peek() != '\"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(CrustCoreErr::Scan {
                line: self.line,
                message: "Unterminated string literal".to_string(),
            });
        };

        self.advance();

        self.tokens.push(Token::String {
            offset: self.start,
            length: self.current - self.start,
            line: self.line,
            value: self.source[self.start + 1..self.current - 1].to_string(),
        });

        Ok(())
    }

    fn take_number_literal(&mut self) -> CrustCoreResult {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let literal = &self.source[self.start..self.current];
        if literal.contains('.') {
            if let Ok(val) = f32::from_str(literal) {
                self.tokens.push(Token::Float {
                    offset: self.start,
                    length: self.current - self.start,
                    line: self.line,
                    value: val,
                });
            } else {
                return Err(CrustCoreErr::Scan {
                    line: self.line,
                    message: "Invalid float value".to_string(),
                });
            }
        } else if let Ok(val) = i32::from_str(literal) {
            self.tokens.push(Token::Integer {
                offset: self.start,
                length: self.current - self.start,
                line: self.line,
                value: val,
            });
        } else {
            return Err(CrustCoreErr::Scan {
                line: self.line,
                message: "Invalid integer value".to_string(),
            });
        }

        Ok(())
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.char_at(self.current + 1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scan_basic_symbols() {
        let symbols = vec![
            Token::LeftParen { offset: 0, line: 1 },
            Token::RightParen { offset: 1, line: 1 },
            Token::LeftBrace { offset: 2, line: 1 },
            Token::RightBrace { offset: 3, line: 1 },
            Token::Comma { offset: 4, line: 1 },
            Token::Dot { offset: 5, line: 1 },
            Token::Minus { offset: 6, line: 1 },
            Token::Plus { offset: 7, line: 1 },
            Token::Semicolon { offset: 8, line: 1 },
            Token::Star { offset: 9, line: 1 },
        ];
        let scanner = Scanner::new("(){},.-+;*");
        let tokens = scanner.scan_tokens();

        tokens
            .unwrap()
            .iter()
            .zip(symbols)
            .for_each(|(token, symbol)| assert_eq!(*token, symbol))
    }

    #[test]
    fn scan_two_char_symbols() {
        let symbols = vec![
            Token::EqualEqual { offset: 0, line: 1 },
            Token::BangEqual { offset: 2, line: 1 },
            Token::Equal { offset: 4, line: 1 },
            Token::Bang { offset: 5, line: 1 },
            Token::LessEqual { offset: 6, line: 1 },
            Token::Less { offset: 8, line: 1 },
            Token::GreaterEqual { offset: 9, line: 1 },
            Token::Greater {
                offset: 11,
                line: 1,
            },
        ];
        let scanner = Scanner::new("==!==!<=<>=>");
        let tokens = scanner.scan_tokens();

        tokens
            .unwrap()
            .iter()
            .zip(symbols)
            .for_each(|(token, symbol)| assert_eq!(*token, symbol))
    }

    #[test]
    fn scan_whitespace() {
        let symbols = vec![
            Token::LeftParen { offset: 0, line: 1 },
            Token::RightParen { offset: 1, line: 1 },
            Token::LeftBrace { offset: 3, line: 1 },
            Token::RightBrace { offset: 4, line: 1 },
            Token::Comma { offset: 6, line: 2 },
            Token::Dot { offset: 7, line: 2 },
            Token::Minus { offset: 8, line: 2 },
            Token::Plus {
                offset: 10,
                line: 2,
            },
            Token::Semicolon {
                offset: 11,
                line: 2,
            },
            Token::Star {
                offset: 12,
                line: 2,
            },
        ];
        let scanner = Scanner::new("() {}\n,.-\t+;*");
        let tokens = scanner.scan_tokens();

        tokens
            .unwrap()
            .iter()
            .zip(symbols)
            .for_each(|(token, symbol)| assert_eq!(*token, symbol))
    }

    #[test]
    fn scan_comment() {
        let symbols = vec![
            Token::LeftParen { offset: 0, line: 1 },
            Token::RightParen {
                offset: 21,
                line: 2,
            },
            Token::Slash {
                offset: 22,
                line: 2,
            },
        ];
        let scanner = Scanner::new("(// this is ignored)\n)/");
        let tokens = scanner.scan_tokens();

        tokens
            .unwrap()
            .iter()
            .zip(symbols)
            .for_each(|(token, symbol)| assert_eq!(*token, symbol));
    }

    #[test]
    fn scan_float_literal_with_access() {
        let symbols = vec![
            Token::LeftParen { offset: 0, line: 1 },
            Token::Float {
                line: 1,
                offset: 1,
                length: 3,
                value: 1.3f32,
            },
            Token::Dot { line: 1, offset: 4 },
            Token::RightParen { offset: 5, line: 1 },
            Token::Integer {
                line: 1,
                offset: 6,
                length: 2,
                value: 25i32,
            },
            Token::Dot { line: 1, offset: 8 },
        ];
        let scanner = Scanner::new("(1.3.)25.");
        let tokens = scanner.scan_tokens();

        tokens
            .unwrap()
            .iter()
            .zip(symbols)
            .for_each(|(token, symbol)| assert_eq!(*token, symbol));
    }

    #[test]
    fn scan_number_literal() {
        let symbols = vec![
            Token::LeftParen { offset: 0, line: 1 },
            Token::Float {
                line: 1,
                offset: 1,
                length: 3,
                value: 1.3f32,
            },
            Token::RightParen { offset: 4, line: 1 },
            Token::Integer {
                line: 1,
                offset: 5,
                length: 2,
                value: 25i32,
            },
        ];
        let scanner = Scanner::new("(1.3)25");
        let tokens = scanner.scan_tokens();

        tokens
            .unwrap()
            .iter()
            .zip(symbols)
            .for_each(|(token, symbol)| assert_eq!(*token, symbol));
    }

    #[test]
    fn scan_string_literal() {
        let symbols = vec![
            Token::LeftParen { offset: 0, line: 1 },
            Token::String {
                line: 1,
                offset: 1,
                length: 18,
                value: "This is a string".to_string(),
            },
            Token::RightParen {
                offset: 19,
                line: 1,
            },
        ];
        let scanner = Scanner::new("(\"This is a string\")");
        let tokens = scanner.scan_tokens();

        tokens
            .unwrap()
            .iter()
            .zip(symbols)
            .for_each(|(token, symbol)| assert_eq!(*token, symbol));
    }
}
