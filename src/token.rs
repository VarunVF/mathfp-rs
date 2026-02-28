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
    Eof,
}

pub struct Scanner {
    source: String,
    start: usize,
    current: usize,
    line: usize,
    column: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_owned(),
            start: 0,
            current: 0,
            line: 1,
            column: 0,
        }
    }

    pub fn report(errors: &[String]) -> String {
        format!("Scanner errors:\n{}", errors.join("\n"))
    }

    fn make_token(&self, kind: TokenType, lexeme: &str) -> Result<Token, String> {
        Ok(Token {
            kind,
            lexeme: String::from(lexeme),
            line: self.line,
            column: self.column,
        })
    }

    pub fn scan(&mut self) -> Result<Vec<Token>, Vec<String>> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut errors: Vec<String> = Vec::new();
        loop {
            match self.scan_token() {
                Ok(token) => {
                    let is_eof = matches!(token.kind, TokenType::Eof);
                    tokens.push(token);
                    if is_eof {
                        break;
                    }
                }
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
            None => return self.make_token(TokenType::Eof, ""),
        };

        self.column += 1;

        match ch {
            '+' => {
                self.advance();
                self.make_token(TokenType::Plus, "+")
            }
            '-' => {
                self.advance();
                self.make_token(TokenType::Minus, "-")
            }
            '*' => {
                self.advance();
                self.make_token(TokenType::Star, "*")
            }
            '/' => {
                self.advance();
                self.make_token(TokenType::Slash, "/")
            }
            '<' => {
                self.advance();
                self.make_token(TokenType::LessThan, "<")
            }
            '>' => {
                self.advance();
                self.make_token(TokenType::GreaterThan, ">")
            }
            '(' => {
                self.advance();
                self.make_token(TokenType::LeftParen, "(")
            }
            ')' => {
                self.advance();
                self.make_token(TokenType::RightParen, ")")
            }
            '|' => self.maps_to(),
            ':' => self.binding(),
            '"' => self.string(),
            '\n' | ';' => {
                self.advance();
                if ch == '\n' {
                    self.line += 1;
                    self.column = 1;
                    self.make_token(TokenType::EndStmt, "\n")
                } else {
                    self.make_token(TokenType::EndStmt, ";")
                }
            }
            ' ' | '\r' | '\t' => {
                self.advance();
                self.scan_token()
            } // Skip whitespace
            _ if ch.is_ascii_digit() || ch == '.' => self.number(),
            _ if ch.is_alphabetic() => self.identifier(),
            _ => {
                self.advance();
                Err(format!(
                    "[Line {}, Col {}] Unexpected character: {}",
                    self.line, self.column, ch
                ))
            }
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
        let value = lexeme.parse::<f64>().map_err(|e| {
            format!(
                "[Line {}, Col {}] Failed to parse '{}' as a number: {}",
                self.line, self.column, lexeme, e
            )
        })?;
        self.make_token(TokenType::Number(value), lexeme)
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
            "if" => self.make_token(TokenType::If, lexeme),
            "then" => self.make_token(TokenType::Then, lexeme),
            "else" => self.make_token(TokenType::Else, lexeme),
            _ => self.make_token(TokenType::Identifier(lexeme.to_string()), lexeme),
        }
    }

    fn maps_to(&mut self) -> Result<Token, String> {
        // symbol |->
        let lexeme = &self.source[self.start..self.current + 3];
        match lexeme {
            "|->" => {
                self.advance_by(3);
                self.make_token(TokenType::MapsTo, "|->")
            }
            _ => {
                self.advance();
                Err(format!(
                    "[Line {}, Col {}] Expected a |-> (MapsTo) symbol",
                    self.line, self.column
                ))
            }
        }
    }

    fn binding(&mut self) -> Result<Token, String> {
        // symbol :=
        let symbol = &self.source[self.start..self.current + 2];
        match symbol {
            ":=" => {
                self.advance_by(2);
                self.make_token(TokenType::Binding, ":=")
            }
            _ => {
                self.advance();
                Err(format!(
                    "[Line {}, Col {}] Expected a := (Binding) symbol",
                    self.line, self.column
                ))
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
            return Err(format!(
                "[Line {}, Col {}] Unterminated string literal",
                self.line, self.column
            ));
        }

        let str_start = self.start + 1; // after the opening "
        let str_end = self.current - 1; // the closing "
        let lexeme = &self.source[str_start..str_end];
        self.make_token(TokenType::String(lexeme.to_string()), lexeme)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use TokenType::*;

    // testing helper
    fn assert_scan(input: &str, expected: Vec<Token>) {
        let actual = Scanner::new(input).scan().unwrap();

        // check length first
        assert_eq!(
            actual.len(),
            expected.len(),
            "Token count mismatch for input: {input}"
        );

        // compare only the kind field
        for (i, (act, exp)) in actual.iter().zip(expected.iter()).enumerate() {
            assert_eq!(
                act.kind, exp.kind,
                "Token mismatch at index {i} for input: {input}"
            );
        }
    }

    // simplified token helper
    fn make_token(kind: TokenType) -> Token {
        Token {
            kind,
            lexeme: std::string::String::new(),
            line: 1,   // placeholder
            column: 1, // placeholder
        }
    }

    #[test]
    fn test_single_stmt() {
        assert_scan(
            "f := x |-> 2 * x;",
            vec![
                make_token(Identifier("f".to_string())),
                make_token(Binding),
                make_token(Identifier("x".to_string())),
                make_token(MapsTo),
                make_token(Number(2.0)),
                make_token(Star),
                make_token(Identifier("x".to_string())),
                make_token(EndStmt),
                make_token(Eof),
            ],
        );
    }

    #[test]
    fn test_multiple_stmt() {
        assert_scan(
            "x := 5.0; y \n",
            vec![
                make_token(Identifier("x".to_string())),
                make_token(Binding),
                make_token(Number(5.0)),
                make_token(EndStmt),
                make_token(Identifier("y".to_string())),
                make_token(EndStmt),
                make_token(Eof),
            ],
        );
    }

    #[test]
    fn test_single_symbols() {
        assert_scan(
            "+ - * / < > ( ) ;",
            vec![
                make_token(Plus),
                make_token(Minus),
                make_token(Star),
                make_token(Slash),
                make_token(LessThan),
                make_token(GreaterThan),
                make_token(LeftParen),
                make_token(RightParen),
                make_token(EndStmt),
                make_token(Eof),
            ],
        );
    }

    #[test]
    fn test_multi_char_symbols() {
        assert_scan(
            ":= |->",
            vec![make_token(Binding), make_token(MapsTo), make_token(Eof)],
        );
    }

    #[test]
    fn test_numbers() {
        assert_scan(
            "123 45.67 .5 -0.5",
            vec![
                make_token(Number(123.0)),
                make_token(Number(45.67)),
                make_token(Number(0.5)),
                make_token(Minus),
                make_token(Number(0.5)),
                make_token(Eof),
            ],
        );
    }

    #[test]
    fn test_keywords_and_identifiers() {
        assert_scan(
            "if then else iffy then_else",
            vec![
                make_token(If),
                make_token(Then),
                make_token(Else),
                make_token(Identifier("iffy".to_string())),
                make_token(Identifier("then_else".to_string())),
                make_token(Eof),
            ],
        );
    }

    #[test]
    fn test_complex_expression_no_whitespace() {
        assert_scan(
            "a:=b|->if(n>0)then x else y;",
            vec![
                make_token(Identifier("a".to_string())),
                make_token(Binding),
                make_token(Identifier("b".to_string())),
                make_token(MapsTo),
                make_token(If),
                make_token(LeftParen),
                make_token(Identifier("n".to_string())),
                make_token(GreaterThan),
                make_token(Number(0.0)),
                make_token(RightParen),
                make_token(Then),
                make_token(Identifier("x".to_string())),
                make_token(Else),
                make_token(Identifier("y".to_string())),
                make_token(EndStmt),
                make_token(Eof),
            ],
        );
    }

    #[test]
    fn test_string() {
        assert_scan(
            "msg := \"hello\"",
            vec![
                make_token(Identifier("msg".to_string())),
                make_token(Binding),
                make_token(TokenType::String("hello".to_string())),
                make_token(Eof),
            ],
        );
    }

    #[test]
    fn test_whitespace_and_newlines() {
        assert_scan(
            "   x     :=   \t   10  \n",
            vec![
                make_token(Identifier("x".to_string())),
                make_token(Binding),
                make_token(Number(10.0)),
                make_token(EndStmt),
                make_token(Eof),
            ],
        );
    }

    #[test]
    fn test_empty() {
        assert_scan("", vec![make_token(Eof)]);
    }
}
