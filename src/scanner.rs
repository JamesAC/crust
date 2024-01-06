use crust_grammar::token::{try_as_keyword, SourceToken, Token};
use std::str::FromStr;

use crate::util::{CrustCoreErr, CrustCoreResult};

pub struct Scanner<'a> {
    source: &'a str,

    start: usize,
    current: usize,
    line: usize,

    tokens: Vec<SourceToken>,
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

    pub fn scan_tokens(mut self) -> CrustCoreResult<Vec<SourceToken>> {
        let mut errors: Vec<CrustCoreErr> = vec![];
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token(&mut errors);
        }

        self.tokens
            .push(SourceToken::new(Token::Eof, self.current, self.line, 0));

        if !errors.is_empty() {
            Err(CrustCoreErr::Multi { errors })
        } else {
            Ok(self.tokens)
        }
    }

    fn scan_token(&mut self, errors: &mut Vec<CrustCoreErr>) {
        let char = self.advance();
        match char {
            '(' => self.push_token(Token::LeftParen),
            ')' => self.push_token(Token::RightParen),
            '{' => self.push_token(Token::LeftBrace),
            '}' => self.push_token(Token::RightBrace),
            ',' => self.push_token(Token::Comma),
            '.' => self.push_token(Token::Dot),
            '-' => self.push_token(Token::Minus),
            '+' => self.push_token(Token::Plus),
            ';' => self.push_token(Token::Semicolon),
            '*' => self.push_token(Token::Star),
            '!' if self.advance_if('=') => {
                self.push_token(Token::BangEqual);
            }
            '!' => {
                self.push_token(Token::Bang);
            }
            '=' if self.advance_if('=') => {
                self.push_token(Token::EqualEqual);
            }
            '=' => {
                self.push_token(Token::Equal);
            }
            '<' if self.advance_if('=') => {
                self.push_token(Token::LessEqual);
            }
            '<' => {
                self.push_token(Token::Less);
            }
            '>' if self.advance_if('=') => {
                self.push_token(Token::GreaterEqual);
            }
            '>' => {
                self.push_token(Token::Greater);
            }
            '/' => {
                if self.advance_if('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.push_token(Token::Slash);
                }
            }
            '0'..='9' => {
                if let Err(e) = self.take_number_literal() {
                    errors.push(e);
                }
            }
            'A'..='z' => {
                if let Err(e) = self.take_identifier() {
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

    fn push_token(&mut self, token: Token) {
        self.tokens.push(SourceToken::new(
            token,
            self.start,
            self.line,
            self.current - self.start,
        ))
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

        self.push_token(Token::String(
            self.source[self.start + 1..self.current - 1].to_string(),
        ));

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
                self.push_token(Token::Float(val));
            } else {
                return Err(CrustCoreErr::Scan {
                    line: self.line,
                    message: "Invalid float value".to_string(),
                });
            }
        } else if let Ok(val) = i32::from_str(literal) {
            self.push_token(Token::Integer(val));
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

    fn take_identifier(&mut self) -> CrustCoreResult<()> {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }
        let text = &self.source[self.start..self.current];

        if let Some(keyword) = try_as_keyword(text) {
            self.push_token(keyword);
        } else {
            self.push_token(Token::Identifier(text.to_string()));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scan_basic_symbols() {
        let symbols = vec![
            Token::LeftParen,
            Token::RightParen,
            Token::LeftBrace,
            Token::RightBrace,
            Token::Comma,
            Token::Dot,
            Token::Minus,
            Token::Plus,
            Token::Semicolon,
            Token::Star,
        ];
        let scanner = Scanner::new("(){},.-+;*");
        let tokens = scanner.scan_tokens();

        tokens
            .unwrap()
            .iter()
            .map(|st| &st.token)
            .zip(symbols)
            .for_each(|(token, symbol)| assert_eq!(*token, symbol))
    }

    #[test]
    fn scan_two_char_symbols() {
        let symbols = vec![
            Token::EqualEqual,
            Token::BangEqual,
            Token::Equal,
            Token::Bang,
            Token::LessEqual,
            Token::Less,
            Token::GreaterEqual,
            Token::Greater,
        ];
        let scanner = Scanner::new("==!==!<=<>=>");
        let tokens = scanner.scan_tokens();

        tokens
            .unwrap()
            .iter()
            .map(|st| &st.token)
            .zip(symbols)
            .for_each(|(token, symbol)| assert_eq!(*token, symbol))
    }

    #[test]
    fn scan_whitespace() {
        let symbols = vec![
            Token::LeftParen,
            Token::RightParen,
            Token::LeftBrace,
            Token::RightBrace,
            Token::Comma,
            Token::Dot,
            Token::Minus,
            Token::Plus,
            Token::Semicolon,
            Token::Star,
        ];
        let scanner = Scanner::new("() {}\n,.-\t+;*");
        let tokens = scanner.scan_tokens();

        tokens
            .unwrap()
            .iter()
            .map(|st| &st.token)
            .zip(symbols)
            .for_each(|(token, symbol)| assert_eq!(*token, symbol))
    }

    #[test]
    fn scan_comment() {
        let symbols = vec![Token::LeftParen, Token::RightParen, Token::Slash];
        let scanner = Scanner::new("(// this is ignored)\n)/");
        let tokens = scanner.scan_tokens();

        tokens
            .unwrap()
            .iter()
            .map(|st| &st.token)
            .zip(symbols)
            .for_each(|(token, symbol)| assert_eq!(*token, symbol))
    }

    #[test]
    fn scan_float_literal_with_access() {
        let symbols = vec![
            Token::LeftParen,
            Token::Float(1.3),
            Token::Dot,
            Token::RightParen,
            Token::Integer(25),
            Token::Dot,
        ];
        let scanner = Scanner::new("(1.3.)25.");
        let tokens = scanner.scan_tokens();

        tokens
            .unwrap()
            .iter()
            .map(|st| &st.token)
            .zip(symbols)
            .for_each(|(token, symbol)| assert_eq!(*token, symbol))
    }

    #[test]
    fn scan_number_literal() {
        let symbols = vec![
            Token::LeftParen,
            Token::Float(1.3),
            Token::RightParen,
            Token::Integer(25),
        ];
        let scanner = Scanner::new("(1.3)25");
        let tokens = scanner.scan_tokens();

        tokens
            .unwrap()
            .iter()
            .map(|st| &st.token)
            .zip(symbols)
            .for_each(|(token, symbol)| assert_eq!(*token, symbol))
    }

    #[test]
    fn scan_string_literal() {
        let symbols = vec![
            SourceToken::new(Token::LeftParen, 0, 1, 1),
            SourceToken::new(Token::String("This is a string".to_string()), 1, 1, 18),
            SourceToken::new(Token::RightParen, 19, 1, 1),
        ];
        let scanner = Scanner::new("(\"This is a string\")");
        let tokens = scanner.scan_tokens();

        tokens
            .unwrap()
            .iter()
            .zip(symbols)
            .for_each(|(token, symbol)| assert_eq!(*token, symbol))
    }

    #[test]
    fn scan_identifiers() {
        let symbols = vec![
            Token::If,
            Token::Else,
            Token::For,
            Token::Class,
            Token::Super,
            Token::Fn,
            Token::Identifier("some_name_1".to_string()),
            Token::True,
            Token::False,
            Token::Mut,
            Token::While,
            Token::Loop,
            Token::Break,
            Token::Return,
            Token::This,
            Token::Let,
        ];
        let scanner = Scanner::new("if else for class super fn some_name_1 true false mut while loop break return this let");
        let tokens = scanner.scan_tokens();

        tokens
            .unwrap()
            .iter()
            .map(|st| &st.token)
            .zip(symbols)
            .for_each(|(token, symbol)| assert_eq!(*token, symbol))
    }
}
