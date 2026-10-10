use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub kind: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    // Single-character tokens
    Plus,
    Minus,
    Star,
    Slash,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftSquareBracket,
    RightSquareBracket,
    Comma,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Data tokens
    Identifier(String),
    Number(f64),
    String(String),

    // Keywords
    If,
    Then,
    Else,
    Match,

    // Special symbols
    MapsTo,
    FatArrow,
    Binding,
    EndStmt,

    // Last token
    Eof,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Plus => f.write_str("'+'"),
            Self::Minus => f.write_str("'-'"),
            Self::Star => f.write_str("'*'"),
            Self::Slash => f.write_str("'/'"),
            Self::LeftParen => f.write_str("'('"),
            Self::RightParen => f.write_str("')'"),
            Self::LeftBrace => f.write_str("'{'"),
            Self::RightBrace => f.write_str("'}'"),
            Self::LeftSquareBracket => f.write_str("'['"),
            Self::RightSquareBracket => f.write_str("']'"),
            Self::Comma => f.write_str("','"),

            Self::Bang => f.write_str("'!'"),
            Self::BangEqual => f.write_str("'!='"),
            Self::Equal => f.write_str("'='"),
            Self::EqualEqual => f.write_str("'=='"),
            Self::Greater => f.write_str("'>'"),
            Self::GreaterEqual => f.write_str("'>='"),
            Self::Less => f.write_str("'<'"),
            Self::LessEqual => f.write_str("'<='"),

            Self::Identifier(data) => write!(f, "'{data}'"),
            Self::Number(data) => write!(f, "{data}"),
            Self::String(data) => write!(f, "\"{data}\""),

            Self::If => f.write_str("'if'"),
            Self::Then => f.write_str("'then'"),
            Self::Else => f.write_str("'else'"),
            Self::Match => f.write_str("'match'"),

            Self::MapsTo => f.write_str("'|->'"),
            Self::FatArrow => f.write_str("'=>'"),
            Self::Binding => f.write_str("':='"),
            Self::EndStmt => f.write_str("';'"),

            Self::Eof => f.write_str("EOF"),
        }
    }
}
