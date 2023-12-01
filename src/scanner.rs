use crust_grammar::token::Token;

use crate::util::{CrustCoreErr, CrustCoreResult};

pub struct Scanner<'a> {
    source: &'a str,

    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(mut self) -> CrustCoreResult<Vec<Token>> {
        let mut tokens: Vec<Token> = vec![];
        let mut errors: Vec<CrustCoreErr> = vec![];
        while !self.is_at_end() {
            let char = self.advance();
            match char {
                "(" => tokens.push(Token::LeftParen {
                    offset: self.current - 1,
                    line: self.line,
                }),
                ")" => tokens.push(Token::RightParen {
                    offset: self.current - 1,
                    line: self.line,
                }),
                "{" => tokens.push(Token::LeftBrace {
                    offset: self.current - 1,
                    line: self.line,
                }),
                "}" => tokens.push(Token::RightBrace {
                    offset: self.current - 1,
                    line: self.line,
                }),
                "," => tokens.push(Token::Comma {
                    offset: self.current - 1,
                    line: self.line,
                }),
                "." => tokens.push(Token::Dot {
                    offset: self.current - 1,
                    line: self.line,
                }),
                "-" => tokens.push(Token::Minus {
                    offset: self.current - 1,
                    line: self.line,
                }),
                "+" => tokens.push(Token::Plus {
                    offset: self.current - 1,
                    line: self.line,
                }),
                ";" => tokens.push(Token::Semicolon {
                    offset: self.current - 1,
                    line: self.line,
                }),
                "*" => tokens.push(Token::Star {
                    offset: self.current - 1,
                    line: self.line,
                }),
                "!" => {
                    if self.advance_if("=") {
                        tokens.push(Token::BangEqual {
                            offset: self.current - 2,
                            line: self.line,
                        });
                    } else {
                        tokens.push(Token::Bang {
                            offset: self.current - 1,
                            line: self.line,
                        });
                    }
                }
                "=" => {
                    if self.advance_if("=") {
                        tokens.push(Token::EqualEqual {
                            offset: self.current - 2,
                            line: self.line,
                        });
                    } else {
                        tokens.push(Token::Equal {
                            offset: self.current - 1,
                            line: self.line,
                        });
                    }
                }
                "<" => {
                    if self.advance_if("=") {
                        tokens.push(Token::LessEqual {
                            offset: self.current - 2,
                            line: self.line,
                        });
                    } else {
                        tokens.push(Token::Less {
                            offset: self.current - 1,
                            line: self.line,
                        });
                    }
                }
                ">" => {
                    if self.advance_if("=") {
                        tokens.push(Token::GreaterEqual {
                            offset: self.current - 2,
                            line: self.line,
                        });
                    } else {
                        tokens.push(Token::Greater {
                            offset: self.current - 1,
                            line: self.line,
                        });
                    }
                }
                " " | "\t" | "\r" => {}
                "\n" => self.line += 1,
                _ => errors.push(CrustCoreErr::Scan {
                    line: self.line,
                    message: "Unexpected character".to_string(),
                }),
            }
        }
        tokens.push(Token::Eof {
            offset: self.current,
            line: self.line,
        });

        if !errors.is_empty() {
            Err(CrustCoreErr::Multi { errors })
        } else {
            Ok(tokens)
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> &str {
        self.current += 1;
        self.char_at(self.current - 1)
    }

    fn advance_if(&mut self, pattern: &str) -> bool {
        if self.is_at_end() || self.char_at(self.current) != pattern {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn char_at(&self, index: usize) -> &str {
        &self.source[index..index + 1]
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
}
