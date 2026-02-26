use crate::ast::{Expr, LiteralValue};
use crate::token::Token;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Expr, String> {
        self.program()
    }

    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn advance(&mut self) {
        self.current += 1;
    }

    fn program(&mut self) -> Result<Expr, String> {
        assert!(
            matches!(self.tokens.last(), Some(Token::Eof)),
            "No EOF token was found"
        );

        let mut statements: Vec<Expr> = vec![];
        while !matches!(self.current(), Some(Token::Eof)) {
            match self.statement()? {
                Expr::Empty => continue,
                stmt => statements.push(stmt),
            };
        }

        Ok(Expr::Program { statements })
    }

    fn statement(&mut self) -> Result<Expr, String> {
        let expr = self.expression();
        match expr {
            Ok(Expr::Empty) => expr,
            Err(msg) => Err(format!("Parser error: {msg}")),
            _ => match self.current() {
                Some(Token::EndStmt | Token::Eof) => expr,
                Some(token) => Err(format!(
                    "Expected ; or newline after expression, found {:?}",
                    token
                )),
                None => unreachable!(),
            },
        }
    }

    fn expression(&mut self) -> Result<Expr, String> {
        if let Some(token) = self.current() {
            match token {
                Token::Number(_) => self.binary_expr(),
                Token::EndStmt => self.empty_expr(),
                Token::Eof => unreachable!(),
                _ => Err(format!("Unexpected token: {:?}", token)),
            }
        } else {
            Err("Expected an expression".to_string())
        }
    }

    fn empty_expr(&mut self) -> Result<Expr, String> {
        self.advance();
        Ok(Expr::Empty)
    }

    fn binary_expr(&mut self) -> Result<Expr, String> {
        let mut left = self.term()?;

        while matches!(self.current(), Some(Token::Plus) | Some(Token::Minus)) {
            let op = self
                .current()
                .ok_or("Expected a binary operator".to_string())?
                .clone();
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

        while matches!(self.current(), Some(Token::Star) | Some(Token::Slash)) {
            let op = self
                .current()
                .ok_or("Expected a binary operator".to_string())?
                .clone();
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
        self.number()
    }

    fn number(&mut self) -> Result<Expr, String> {
        if let Some(token) = self.current() {
            match *token {
                Token::Number(value) => {
                    self.advance();
                    Ok(Expr::Literal(LiteralValue::Number(value)))
                }
                _ => Err(format!("Expected a numeric literal, found {:?}", *token)),
            }
        } else {
            Err("Expected an expression".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Expr::*;
    use Token::*;

    // testing helper
    fn assert_parse(input: Vec<Token>, expected: Expr) {
        let actual = Parser::new(input.clone()).parse().unwrap();
        assert_eq!(actual, expected, "Failed on input: {:?}", input);
    }

    #[test]
    fn test_empty() {
        assert_parse(vec![Eof], Program { statements: vec![] });
    }

    #[test]
    fn test_expr() {
        assert_parse(
            vec![Number(5.0), Plus, Number(3.0), Star, Number(1.0), Eof],
            Program {
                statements: vec![Binary {
                    left: Box::new(Literal(LiteralValue::Number(5.0))),
                    op: Plus,
                    right: Box::new(Binary {
                        left: Box::new(Literal(LiteralValue::Number(3.0))),
                        op: Star,
                        right: Box::new(Literal(LiteralValue::Number(1.0))),
                    }),
                }],
            },
        );
    }

    #[test]
    fn test_stmt() {
        assert_parse(
            vec![Number(5.0), Star, Number(3.0), EndStmt, Eof],
            Program {
                statements: vec![Binary {
                    left: Box::new(Literal(LiteralValue::Number(5.0))),
                    op: Star,
                    right: Box::new(Literal(LiteralValue::Number(3.0))),
                }],
            },
        );
    }

    #[test]
    #[should_panic(expected = "Parser error: Expected a numeric literal, found Star")]
    fn test_invalid_expr() {
        Parser::new(vec![Number(5.0), Plus, Star, Eof])
            .parse()
            .unwrap();
    }
}
