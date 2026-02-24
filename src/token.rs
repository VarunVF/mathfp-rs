#[derive(Debug)]
pub enum Token {
    // Single-character tokens
    Plus,
    Minus,
    Star,
    Slash,
    LessThan,
    GreaterThan,
    LeftParen,
    RightParen,

    // Data tokens
    Identifier(String),
    Number(f64),

    // Keywords
    If,
    Then,
    Else,

    // Special symbols
    MapsTo,
    Binding,
    EndStmt,

    // Last token
    EOF,
}

pub struct Scanner {
    source: String,
    start: usize,
    current: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Scanner {
        Scanner {
            source: source.to_owned(),
            start: 0,
            current: 0,
        }
    }

    pub fn scan(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        loop {
            match self.scan_token() {
                Ok(token) => {
                    let is_eof = matches!(token, Token::EOF);
                    tokens.push(token);
                    if is_eof {
                        break;
                    }
                },
                Err(message) => eprintln!("Scanner error: {}", message),
            }
        }
        tokens
    }

    fn scan_token(&mut self) -> Result<Token, String> {
        self.start = self.current;

        let ch = match self.current() {
            Some(value) => value,
            None => return Ok(Token::EOF),
        };

        match ch {
            '+' => { self.advance(); Ok(Token::Plus) },
            '-' => { self.advance(); Ok(Token::Minus) },
            '*' => { self.advance(); Ok(Token::Star) },
            '/' => { self.advance(); Ok(Token::Slash) },
            '<' => { self.advance(); Ok(Token::LessThan) },
            '>' => { self.advance(); Ok(Token::GreaterThan) },
            '(' => { self.advance(); Ok(Token::LeftParen) },
            ')' => { self.advance(); Ok(Token::RightParen) },
            '|' => self.maps_to(),
            ':' => self.binding(),
            '\n' | ';' => { self.advance(); Ok(Token::EndStmt) },
            ' ' | '\r' | '\t' => { self.advance(); self.scan_token() } // Skip whitespace
            _ if ch.is_ascii_digit() || ch == '.' => self.number(),
            _ if ch.is_alphabetic() => self.identifier(),
            _ => {
                self.advance();
                Err(format!("Unexpected character: {}", ch))
            },
        }
    }

    fn current(&self) -> Option<char> {
        self.source.chars().nth(self.current)
    }

    fn advance(&mut self) {
        self.current += 1;
    }

    fn advance_by(&mut self, amount: usize) {
        self.current += amount;
    }

    fn number(&mut self) -> Result<Token, String> {
        while self.current().is_some() {
            let ch = self.current().unwrap();
            if ch.is_numeric() || ch == '.' {
                self.advance();
            } else {
                break;
            }
        }

        let lexeme = &self.source[self.start..self.current];
        match lexeme.parse::<f64>() {
            Ok(value) => Ok(Token::Number(value)),
            Err(error) => Err(format!("Scanner error: {}", error)),
        }
    }

    fn identifier(&mut self) -> Result<Token, String> {
        while self.current().is_some() {
            let ch = self.current().unwrap();
            if ch.is_alphanumeric() || ch == '_' {
                self.advance();
            } else {
                break;
            }
        }

        let lexeme = &self.source[self.start..self.current];
        match lexeme {
            "if" => Ok(Token::If),
            "then" => Ok(Token::Then),
            "else" => Ok(Token::Else),
            _ => Ok(Token::Identifier(lexeme.to_string())),
        }
    }
    
    fn maps_to(&mut self) -> Result<Token, String> {
        // symbol |->
        let symbol = &self.source[self.start..self.current+3];
        match symbol {
            "|->" => {
                self.advance_by(3);
                Ok(Token::MapsTo)
            },
            _ => Err("Expected a |-> (MapsTo) symbol".to_string()),
        }
    }
    
    fn binding(&mut self) -> Result<Token, String> {
        // symbol :=
        let symbol = &self.source[self.start..self.current+2];
        match symbol {
            ":=" => {
                self.advance_by(2);
                Ok(Token::Binding)
            },
            _ => Err("Expected a := (Binding) symbol".to_string()),
        }
    }
}
