pub mod token {
    use std::str::FromStr;

    use strum::{EnumDiscriminants, EnumString};

    #[derive(Debug, PartialEq, EnumDiscriminants)]
    #[strum_discriminants(derive(EnumString))]
    #[strum_discriminants(name(TokenType))]
    #[strum_discriminants(strum(ascii_case_insensitive))]
    pub enum Token {
        // Symbols
        LeftParen {
            offset: usize,
            line: usize,
        },
        RightParen {
            offset: usize,
            line: usize,
        },
        LeftBrace {
            offset: usize,
            line: usize,
        },
        RightBrace {
            offset: usize,
            line: usize,
        },
        Comma {
            offset: usize,
            line: usize,
        },
        Dot {
            offset: usize,
            line: usize,
        },
        Minus {
            offset: usize,
            line: usize,
        },
        Plus {
            offset: usize,
            line: usize,
        },
        Semicolon {
            offset: usize,
            line: usize,
        },
        Slash {
            offset: usize,
            line: usize,
        },
        Star {
            offset: usize,
            line: usize,
        },

        Bang {
            offset: usize,
            line: usize,
        },
        BangEqual {
            offset: usize,
            line: usize,
        },
        Equal {
            offset: usize,
            line: usize,
        },
        EqualEqual {
            offset: usize,
            line: usize,
        },
        Greater {
            offset: usize,
            line: usize,
        },
        GreaterEqual {
            offset: usize,
            line: usize,
        },
        Less {
            offset: usize,
            line: usize,
        },
        LessEqual {
            offset: usize,
            line: usize,
        },
        BitAnd {
            offset: usize,
            line: usize,
        },
        BitOr {
            offset: usize,
            line: usize,
        },
        And {
            offset: usize,
            line: usize,
        },
        Or {
            offset: usize,
            line: usize,
        },

        // Keywords
        Eof {
            offset: usize,
            line: usize,
        },
        Class {
            offset: usize,
            line: usize,
        },
        If {
            offset: usize,
            line: usize,
        },
        Else {
            offset: usize,
            line: usize,
        },
        True {
            offset: usize,
            line: usize,
        },
        False {
            offset: usize,
            line: usize,
        },
        Fn {
            offset: usize,
            line: usize,
        },
        For {
            offset: usize,
            line: usize,
        },
        Mut {
            offset: usize,
            line: usize,
        },
        While {
            offset: usize,
            line: usize,
        },
        Loop {
            offset: usize,
            line: usize,
        },
        Break {
            offset: usize,
            line: usize,
        },
        Return {
            offset: usize,
            line: usize,
        },
        This {
            offset: usize,
            line: usize,
        },
        Super {
            offset: usize,
            line: usize,
        },
        Let {
            offset: usize,
            line: usize,
        },

        // Literals
        Identifier {
            offset: usize,
            length: usize,
            line: usize,
            value: String,
        },
        String {
            offset: usize,
            length: usize,
            line: usize,
            value: String,
        },
        Float {
            offset: usize,
            length: usize,
            line: usize,
            value: f32,
        },
        Integer {
            offset: usize,
            length: usize,
            line: usize,
            value: i32,
        },
    }

    pub fn is_keyword(text: &str) -> Option<TokenType> {
        let a = TokenType::from_str(text);
        unimplemented!();
    }
}
