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
            Self::Plus => f.write_str("Plus('+')"),
            Self::Minus => f.write_str("Minus('-')"),
            Self::Star => f.write_str("Star('*')"),
            Self::Slash => f.write_str("Slash('/')"),
            Self::LeftParen => f.write_str("LeftParen('(')"),
            Self::RightParen => f.write_str("RightParen(')')"),
            Self::LeftBrace => f.write_str("LeftBrace('{')"),
            Self::RightBrace => f.write_str("RightBrace('}')"),
            Self::LeftSquareBracket => f.write_str("LeftSquareBracket('[')"),
            Self::RightSquareBracket => f.write_str("RightSquareBracket(']')"),
            Self::Comma => f.write_str("Comma(',')"),

            Self::Bang => f.write_str("Bang('!')"),
            Self::BangEqual => f.write_str("BangEqual('!=')"),
            Self::Equal => f.write_str("Equal('=')"),
            Self::EqualEqual => f.write_str("EqualEqual('==')"),
            Self::Greater => f.write_str("Greater('>')"),
            Self::GreaterEqual => f.write_str("GreaterEqual('>=')"),
            Self::Less => f.write_str("Less('<')"),
            Self::LessEqual => f.write_str("LessEqual('<=')"),

            Self::Identifier(data) => write!(f, "Identifier({data})"),
            Self::Number(data) => write!(f, "Number({data})"),
            Self::String(data) => write!(f, "String({data})"),

            Self::If => f.write_str("If"),
            Self::Then => f.write_str("Then"),
            Self::Else => f.write_str("Else"),
            Self::Match => f.write_str("Match"),

            Self::MapsTo => f.write_str("MapsTo('|->')"),
            Self::FatArrow => f.write_str("FatArrow('=>')"),
            Self::Binding => f.write_str("Binding(':=')"),
            Self::EndStmt => f.write_str("EndStmt(';')"),

            Self::Eof => f.write_str("Eof"),
        }
    }
}
