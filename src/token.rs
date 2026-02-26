#[derive(Clone, Debug, PartialEq)]
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
    String(String),

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
    line: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_owned(),
            start: 0,
            current: 0,
            line: 1
        }
    }

    pub fn report(errors: &Vec<String>) -> String {
        format!("Scanner errors:\n{}", errors.join("\n"))
    }

    pub fn scan(&mut self) -> Result<Vec<Token>, Vec<String>> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut errors: Vec<String> = Vec::new();
        loop {
            match self.scan_token() {
                Ok(token) => {
                    let is_eof = matches!(token, Token::EOF);
                    tokens.push(token);
                    if is_eof {
                        break;
                    }
                },
                Err(message) => errors.push(message),
            }
        }
        
        if errors.is_empty() {
            Ok(tokens)
        } else {
            Err(errors)
        }
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
            '"' => self.string(),
            '\n' | ';' => {
                self.advance();
                if ch == '\n' {
                    self.line += 1;
                }
                Ok(Token::EndStmt)
            },
            ' ' | '\r' | '\t' => { self.advance(); self.scan_token() } // Skip whitespace
            _ if ch.is_ascii_digit() || ch == '.' => self.number(),
            _ if ch.is_alphabetic() => self.identifier(),
            _ => {
                self.advance();
                Err(format!("[Line {}] Unexpected character: {}", self.line, ch))
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
        let value = lexeme.parse::<f64>()
            .map_err(|e|
                format!("[Line {}] Failed to parse '{}' as a number: {}", self.line, lexeme, e)
            )?;
        Ok(Token::Number(value))
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
            _ => {
                self.advance();
                Err(format!("[Line {}] Expected a |-> (MapsTo) symbol", self.line))
            }
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
            _ => {
                self.advance();
                Err(format!("[Line {}] Expected a := (Binding) symbol", self.line))
            }
        }
    }
    
    fn string(&mut self) -> Result<Token, String> {
        self.advance(); // skip the opening "
        let mut is_terminated = false;
        while self.current().is_some() {
            let ch = self.current().unwrap();
            self.advance();
            if ch == '\"' {
                is_terminated = true;
                break;
            }
        }

        if !is_terminated {
            return Err(format!("[Line {}] Unterminated string literal", self.line));
        }

        let str_start = self.start + 1; // after the opening "
        let str_end = self.current - 1; // the closing "
        let lexeme = &self.source[str_start..str_end];
        Ok(Token::String(lexeme.to_string()))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use Token::*;

    // testing helper
    fn assert_scan(input: &str, expected: Vec<Token>) {
        let actual = Scanner::new(input).scan().unwrap();
        assert_eq!(actual, expected, "Failed on input: {input}");
    }

    #[test]
    fn test_single_stmt() {
        assert_scan("f := x |-> 2 * x;", vec![
            Identifier("f".to_string()),
            Binding,
            Identifier("x".to_string()),
            MapsTo,
            Number(2.0),
            Star,
            Identifier("x".to_string()),
            EndStmt,
            EOF,
        ]);
    }

    #[test]
    fn test_multiple_stmt() {
        assert_scan("x := 5.0; y \n", vec![
            Identifier("x".to_string()),
            Binding,
            Number(5.0),
            EndStmt,
            Identifier("y".to_string()),
            EndStmt,
            EOF,
        ]);
    }


    #[test]
    fn test_single_symbols() {
        assert_scan(
            "+ - * / < > ( ) ;",
            vec![
                Plus, Minus, Star, Slash, LessThan, GreaterThan, 
                LeftParen, RightParen, EndStmt, EOF
            ]
        );
    }

    #[test]
    fn test_multi_char_symbols() {
        assert_scan(
            ":= |->",
            vec![Binding, MapsTo, EOF]
        );
    }

    #[test]
    fn test_numbers() {
        assert_scan(
            "123 45.67 .5 -0.5",
            vec![
                Number(123.0),
                Number(45.67),
                Number(0.5),
                Minus,
                Number(0.5),
                EOF
            ]
        );
    }

    #[test]
    fn test_keywords_and_identifiers() {
        assert_scan(
            "if then else iffy then_else",
            vec![
                If, Then, Else,
                Identifier("iffy".to_string()),
                Identifier("then_else".to_string()),
                EOF
            ]
        );
    }

    #[test]
    fn test_complex_expression_no_whitespace() {
        assert_scan(
            "a:=b|->if(n>0)then x else y;",
            vec![
                Identifier("a".to_string()),
                Binding,
                Identifier("b".to_string()),
                MapsTo,
                If,
                LeftParen,
                Identifier("n".to_string()),
                GreaterThan,
                Number(0.0),
                RightParen,
                Then,
                Identifier("x".to_string()),
                Else,
                Identifier("y".to_string()),
                EndStmt,
                EOF
            ]
        );
    }

    #[test]
    fn test_string() {
        assert_scan("msg := \"hello\"", vec![
            Identifier("msg".to_string()),
            Binding,
            Token::String("hello".to_string()),
            EOF
        ]);
    }

    #[test]
    fn test_whitespace_and_newlines() {
        assert_scan(
            "   x     :=   \t   10  \n",
            vec![Identifier("x".to_string()), Binding, Number(10.0), EndStmt, EOF]
        );
    }

    #[test]
    fn test_empty() {
        assert_scan("", vec![EOF]);
    }
}
