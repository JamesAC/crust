pub mod token {
    use std::str::FromStr;

    use strum::{EnumDiscriminants, EnumString};

    #[derive(Debug, PartialEq, EnumDiscriminants)]
    #[strum_discriminants(derive(EnumString))]
    #[strum_discriminants(name(TokenType))]
    #[strum_discriminants(strum(ascii_case_insensitive))]
    pub enum Token {
        // Symbols
        LeftParen,
        RightParen,
        LeftBrace,
        RightBrace,
        Comma,
        Dot,
        Minus,
        Plus,
        Semicolon,
        Slash,
        Star,

        Bang,
        BangEqual,
        Equal,
        EqualEqual,
        Greater,
        GreaterEqual,
        Less,
        LessEqual,
        BitAnd,
        BitOr,
        And,
        Or,

        Eof,

        // Keywords
        Class,
        If,
        Else,
        True,
        False,
        Fn,
        For,
        Mut,
        While,
        Loop,
        Break,
        Return,
        This,
        Super,
        Let,

        // Literals
        Identifier(String),
        String(String),
        Float(f32),
        Integer(i32),
    }

    #[derive(Debug, PartialEq)]
    pub struct SourceToken {
        pub token: Token,
        pub offset: usize,
        pub line: usize,
        pub length: usize,
    }
    impl SourceToken {
        pub fn new(token: Token, offset: usize, line: usize, length: usize) -> Self {
            Self {
                token,
                offset,
                line,
                length,
            }
        }
    }

    pub fn try_as_keyword(text: &str) -> Option<Token> {
        match TokenType::from_str(text) {
            Ok(token_type) => match token_type {
                TokenType::Class => Some(Token::Class),
                TokenType::If => Some(Token::If),
                TokenType::Else => Some(Token::Else),
                TokenType::True => Some(Token::True),
                TokenType::False => Some(Token::False),
                TokenType::Fn => Some(Token::Fn),
                TokenType::For => Some(Token::For),
                TokenType::Mut => Some(Token::Mut),
                TokenType::While => Some(Token::While),
                TokenType::Loop => Some(Token::Loop),
                TokenType::Break => Some(Token::Break),
                TokenType::Return => Some(Token::Return),
                TokenType::This => Some(Token::This),
                TokenType::Super => Some(Token::Super),
                TokenType::Let => Some(Token::Let),
                _ => None,
            },
            Err(_) => None,
        }
    }
}
