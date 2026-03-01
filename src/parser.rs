use crate::ast::{Expr, LiteralValue};
use crate::token::{Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn report(errors: &[String]) -> String {
        format!("Parser errors:\n{}", errors.join("\n"))
    }

    pub fn parse(&mut self) -> Result<Expr, Vec<String>> {
        self.program()
    }

    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn current_kind(&self) -> Option<TokenType> {
        self.tokens
            .get(self.current)
            .map(|token| token.kind.clone())
    }

    fn lookahead_kind(&self) -> Option<TokenType> {
        self.tokens
            .get(self.current + 1)
            .map(|token| token.kind.clone())
    }

    fn advance(&mut self) {
        self.current += 1;
    }

    fn make_error(&self, message: &str) -> Result<Expr, String> {
        let default = &Token {
            kind: TokenType::Eof,
            lexeme: String::new(),
            line: 1,
            column: 1,
        };
        let token = self.current().unwrap_or(default);
        Err(format!(
            "[Line {}, Col {}] {}",
            token.line, token.column, message
        ))
    }

    fn is_at_end(&self) -> bool {
        match self.current() {
            Some(token) => token.kind == TokenType::Eof,
            None => true,
        }
    }

    /// Skips to the beginning of the next statement.
    /// This is used when a parse error is found to avoid cascading errors.
    fn synchronise(&mut self) {
        while let Some(kind) = self.current_kind() {
            match kind {
                TokenType::EndStmt | TokenType::Eof => break,
                _ => self.advance(),
            }
        }
    }

    pub fn program(&mut self) -> Result<Expr, Vec<String>> {
        let mut statements = vec![];
        let mut errors = vec![];

        while !self.is_at_end() {
            match self.statement() {
                Ok(Expr::Empty) => continue,
                Ok(stmt) => statements.push(stmt),
                Err(message) => {
                    errors.push(message);
                    self.synchronise();
                }
            }
        }

        if errors.is_empty() {
            Ok(Expr::Program { statements })
        } else {
            Err(errors)
        }
    }

    fn statement(&mut self) -> Result<Expr, String> {
        let expr = self.expression()?;
        match expr {
            Expr::Empty => Ok(expr),
            _ => match self.current_kind() {
                Some(TokenType::EndStmt | TokenType::Eof) => Ok(expr),
                Some(kind) => self.make_error(&format!(
                    "Expected ; or newline after expression, found {:?}",
                    kind
                )),
                None => Err("Expected ; or newline after expression".to_string()),
            },
        }
    }

    fn expression(&mut self) -> Result<Expr, String> {
        match self.current_kind() {
            Some(TokenType::EndStmt) => self.empty_expr(),
            Some(TokenType::Eof) => unreachable!(),
            Some(_) => match self.lookahead_kind() {
                Some(TokenType::Binding) => self.binding(),
                _ => self.binary_expr(),
            },
            None => self.make_error("Expected an expression"),
        }
    }

    fn empty_expr(&mut self) -> Result<Expr, String> {
        self.advance();
        Ok(Expr::Empty)
    }

    fn binding(&mut self) -> Result<Expr, String> {
        let name = match self.primary()? {
            Expr::Variable(name) => name,
            _ => return self.make_error("Expected an identifier to bind a value"),
        };
        let expr = match self.current_kind() {
            Some(TokenType::Binding) => {
                self.advance();
                self.expression()?
            }
            Some(kind) => {
                self.make_error(&format!("Expected a binding expression, found {:?}", kind))?
            }
            None => self.make_error("Expected an expression")?,
        };
        Ok(Expr::Binding {
            name,
            expr: Box::new(expr),
        })
    }

    fn binary_expr(&mut self) -> Result<Expr, String> {
        let mut left = self.term()?;

        while matches!(
            self.current_kind(),
            Some(TokenType::Plus | TokenType::Minus)
        ) {
            let op = match self.current() {
                Some(op) => op.clone(),
                None => return self.make_error("Expected a binary operator"),
            };
            self.advance();
            let right = self.term()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn term(&mut self) -> Result<Expr, String> {
        let mut left = self.factor()?;

        while matches!(
            self.current_kind(),
            Some(TokenType::Star | TokenType::Slash)
        ) {
            let op = match self.current() {
                Some(op) => op.clone(),
                None => return self.make_error("Expected a binary operator"),
            };
            self.advance();
            let right = self.factor()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn factor(&mut self) -> Result<Expr, String> {
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, String> {
        match self.current_kind() {
            Some(TokenType::Number(value)) => {
                self.advance();
                Ok(Expr::Literal(LiteralValue::Number(value)))
            }
            Some(TokenType::Identifier(name)) => {
                self.advance();
                Ok(Expr::Variable(name))
            }
            Some(TokenType::String(message)) => {
                self.advance();
                Ok(Expr::Literal(LiteralValue::String(message.clone())))
            }
            Some(TokenType::LeftParen) => self.grouping(),
            Some(kind) => {
                self.make_error(&format!("Expected a primary expression, found {:?}", kind))
            }
            None => self.make_error("Expected an expression"),
        }
    }

    fn grouping(&mut self) -> Result<Expr, String> {
        self.advance(); // opening (
        let expr = self.expression()?;
        match self.current_kind() {
            Some(TokenType::RightParen) => {
                self.advance(); // closing )
                Ok(Expr::Grouping(Box::new(expr)))
            }
            Some(kind) => self.make_error(&format!(
                "Expected ) after parenthesised expression, found {:?}",
                kind
            )),
            None => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Expr::*;
    use TokenType::*;

    // testing helper
    fn assert_parse(input: Vec<Token>, expected: Expr) {
        let actual = Parser::new(input.clone()).parse().unwrap();
        assert_eq!(actual, expected, "Failed on input: {:?}", input);
    }

    // token helper
    fn make_token(kind: TokenType) -> Token {
        // hardcode char position and lexeme for testing purposes
        Token {
            kind,
            lexeme: std::string::String::new(),
            line: 1,
            column: 1,
        }
    }

    #[test]
    fn test_empty() {
        assert_parse(vec![make_token(Eof)], Program { statements: vec![] });
    }

    #[test]
    fn test_expr() {
        assert_parse(
            vec![
                make_token(Number(5.0)),
                make_token(Plus),
                make_token(Number(3.0)),
                make_token(Star),
                make_token(Number(1.0)),
                make_token(Eof),
            ],
            Program {
                statements: vec![Binary {
                    left: Box::new(Literal(LiteralValue::Number(5.0))),
                    op: make_token(Plus),
                    right: Box::new(Binary {
                        left: Box::new(Literal(LiteralValue::Number(3.0))),
                        op: make_token(Star),
                        right: Box::new(Literal(LiteralValue::Number(1.0))),
                    }),
                }],
            },
        );
    }

    #[test]
    fn test_stmt() {
        assert_parse(
            vec![
                make_token(Number(5.0)),
                make_token(Star),
                make_token(Number(3.0)),
                make_token(EndStmt),
                make_token(Eof),
            ],
            Program {
                statements: vec![Binary {
                    left: Box::new(Literal(LiteralValue::Number(5.0))),
                    op: make_token(Star),
                    right: Box::new(Literal(LiteralValue::Number(3.0))),
                }],
            },
        );
    }

    #[test]
    #[should_panic(expected = "Expected a primary expression")]
    fn test_invalid_expr() {
        Parser::new(vec![
            make_token(Number(5.0)),
            make_token(Plus),
            make_token(Star),
            make_token(Eof),
        ])
        .parse()
        .unwrap();
    }

    #[test]
    fn test_grouping() {
        // ((9)*(9))
        assert_parse(
            vec![
                make_token(LeftParen),
                make_token(LeftParen),
                make_token(Number(9.0)),
                make_token(RightParen),
                make_token(Star),
                make_token(LeftParen),
                make_token(Number(9.0)),
                make_token(RightParen),
                make_token(RightParen),
                make_token(Eof),
            ],
            Program {
                statements: vec![Grouping(Box::new(Binary {
                    left: Box::new(Grouping(Box::new(Literal(LiteralValue::Number(9.0))))),
                    op: make_token(Star),
                    right: Box::new(Grouping(Box::new(Literal(LiteralValue::Number(9.0))))),
                }))],
            },
        );
    }

    #[test]
    #[should_panic(expected = "Expected ) after parenthesised expression")]
    fn test_invalid_grouping_close() {
        // ((9)*(9
        Parser::new(vec![
            make_token(LeftParen),
            make_token(LeftParen),
            make_token(Number(9.0)),
            make_token(RightParen),
            make_token(Star),
            make_token(LeftParen),
            make_token(Number(9.0)),
            make_token(Eof),
        ])
        .parse()
        .unwrap();
    }

    #[test]
    #[should_panic(expected = "Expected ; or newline after expression")]
    fn test_invalid_grouping_open() {
        // 9)*(9))
        Parser::new(vec![
            make_token(Number(9.0)),
            make_token(RightParen),
            make_token(Star),
            make_token(LeftParen),
            make_token(Number(9.0)),
            make_token(RightParen),
            make_token(RightParen),
            make_token(Eof),
        ])
        .parse()
        .unwrap();
    }
}
