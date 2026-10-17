use crate::{
    ast::{Expr, LiteralValue},
    parser::Parser,
    scanner::Scanner,
};

pub struct Transpiler {
    do_standalone: bool,
}

impl Transpiler {
    pub fn new(do_standalone: bool) -> Self {
        Self { do_standalone }
    }

    pub fn transpile(&self, input: &str) -> Result<String, String> {
        let tokens = Scanner::new(input)
            .scan()
            .map_err(|errors| Scanner::report(&errors))?;

        let expr = Parser::new(tokens)
            .parse()
            .map_err(|errors| Parser::report(&errors))?;

        self.expr_to_latex(&expr)
    }

    fn expr_to_latex(&self, expr: &Expr) -> Result<String, String> {
        let mut preamble = "";
        let mut closing = "";

        if self.do_standalone {
            preamble = "\\documentclass{article}\n\
                        \\usepackage{amsmath}\n\
                        \\usepackage{mathtools}\n\
                        \\begin{document}\n\n";
            closing = "\n\\end{document}\n";
        }

        match expr {
            Expr::Program { statements } => {
                let mut result = String::from(preamble);
                for stmt in statements {
                    result.push_str("\\[");
                    result.push_str(&self.expr_to_latex(stmt)?);
                    result.push_str("\\]\n");
                }
                result.push_str(closing);
                Ok(result)
            }
            Expr::Block { statements } => {
                let mut result = String::from("\\{");
                for stmt in statements {
                    result.push_str(&self.expr_to_latex(stmt)?);
                    result.push_str(";\n");
                }
                result.push_str("\\}");
                Ok(result)
            }
            Expr::Binary { left, op, right } => {
                if op.lexeme == "/" {
                    let mut left_inner: &Expr = left;
                    let mut right_inner: &Expr = right;
                    if let Expr::Grouping(inner) = &**left {
                        left_inner = inner;
                    }
                    if let Expr::Grouping(inner) = &**right {
                        right_inner = inner;
                    }
                    Ok(format!(
                        "\\frac{{{}}}{{{}}}",
                        self.expr_to_latex(left_inner)?,
                        self.expr_to_latex(right_inner)?
                    ))
                } else {
                    let op_str = match op.lexeme.as_str() {
                        "==" => "=",
                        "!=" => "\\ne",
                        "<=" => "\\le",
                        ">=" => "\\ge",
                        "*" => "\\cdot",
                        other => other,
                    };
                    Ok(format!(
                        "{} {op_str} {}",
                        self.expr_to_latex(left)?,
                        self.expr_to_latex(right)?
                    ))
                }
            }
            Expr::Unary { op, right } => Ok(format!("{}{}", op.lexeme, self.expr_to_latex(right)?)),
            Expr::Grouping(expr) => Ok(format!("({})", self.expr_to_latex(expr)?)),
            Expr::Variable(name) => match name.len() {
                1 => Ok(name.replace("_", "\\_")),
                _ => Ok(format!("\\mathrm{{{}}}", name.replace("_", "\\_"))),
            },
            Expr::Assign { name, expr } => Ok(format!(
                "{} \\leftarrow {}",
                name.replace("_", "\\_"),
                self.expr_to_latex(expr)?
            )),
            Expr::Binding { name, expr } => Ok(format!(
                "\\mathrm{{{}}} \\coloneq {}",
                name.replace("_", "\\_"),
                self.expr_to_latex(expr)?
            )),
            Expr::Literal(literal) => Ok(match literal {
                LiteralValue::Number(number) => number.to_string(),
                LiteralValue::String(string) => format!("\\text{{``{}''}}", string.escape_debug()),
                LiteralValue::Boolean(cond) => (if *cond { "true" } else { "false" }).to_string(),
                LiteralValue::Nil => "nil".to_string(),
            }),
            Expr::FunctionDef { param, body } => {
                let param = match param.len() {
                    1 => param.replace("_", "\\_"),
                    _ => format!("\\mathrm{{{}}}", param.replace("_", "\\_")),
                };
                Ok(format!("{param} \\mapsto {}", self.expr_to_latex(body)?))
            }
            Expr::FunctionCall { func, arg } => Ok(match &**func {
                Expr::Variable(name) => match name.as_str() {
                    "sqrt" => format!("\\{name}{{{}}}", self.expr_to_latex(arg)?),
                    "sin" | "cos" => format!("\\{name}({})", self.expr_to_latex(arg)?),
                    "floor" => format!("\\lfloor {} \\rfloor", self.expr_to_latex(arg)?),
                    "ceil" => format!("\\lceil {} \\rceil", self.expr_to_latex(arg)?),
                    _ => format!(
                        "{}({})",
                        self.expr_to_latex(func)?,
                        self.expr_to_latex(arg)?
                    ),
                },
                _ => format!(
                    "{}({})",
                    self.expr_to_latex(func)?,
                    self.expr_to_latex(arg)?
                ),
            }),
            Expr::If {
                cond_expr,
                then_expr,
                else_expr,
            } => Ok(format!(
                "\\begin{{cases}} {} & \\text{{if }} {}, \\\\ {} &\\text{{otherwise}} \\end{{cases}}",
                self.expr_to_latex(then_expr)?,
                self.expr_to_latex(cond_expr)?,
                self.expr_to_latex(else_expr)?,
            )),
            Expr::Match { arms } => {
                let mut result = String::from("\\begin{cases} ");
                for i in 0..arms.len() {
                    let arm = &arms[i];
                    result.push_str(&self.expr_to_latex(&arm.body)?);
                    if let Expr::Variable(name) = &*arm.pattern
                        && name == "true"
                    {
                        result.push_str(" & \\text{otherwise } ");
                    } else {
                        result.push_str(" & \\text{if } ");
                        result.push_str(&self.expr_to_latex(&arm.pattern)?);
                    }

                    if i == arms.len() - 1 {
                        result.push_str(" \\\\ ");
                    } else {
                        result.push_str(", \\\\ ");
                    }
                }
                result.push_str("\\end{cases}");
                Ok(result)
            }
            Expr::List { elements } => {
                let mut result = String::from("[");
                for i in 0..elements.len() {
                    result.push_str(&self.expr_to_latex(&elements[i])?);
                    if i != elements.len() - 1 {
                        result.push_str(", ");
                    }
                }
                result.push(']');
                Ok(result)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_latex(input: &str, expected: &str) {
        let result = Transpiler::new(false).transpile(input).unwrap();
        assert_eq!(result.trim(), expected.trim());
    }

    fn assert_document(input: &str, expected: &str) {
        let result = Transpiler::new(true).transpile(input).unwrap();
        assert_eq!(result.trim(), expected.trim());
    }

    #[test]
    fn test_document() {
        let input = "x := 0";
        let expected = "\
            \\documentclass{article}\n\
            \\usepackage{amsmath}\n\
            \\usepackage{mathtools}\n\
            \\begin{document}\n\
            \n\
            \\[\\mathrm{x} \\coloneq 0\\]\n\
            \n\
            \\end{document}\n";
        assert_document(input, expected);
    }

    #[test]
    fn test_quadratic_formula() {
        assert_latex(
            "(-b + sqrt(b*b - 4*a*c)) / (2*a)",
            r"\[\frac{-b + \sqrt{b \cdot b - 4 \cdot a \cdot c}}{2 \cdot a}\]",
        );
    }

    #[test]
    fn test_floor_ceil() {
        assert_latex(
            "floor(5.1) + ceil(7.2)",
            r"\[\lfloor 5.1 \rfloor + \lceil 7.2 \rceil\]",
        );
    }

    #[test]
    fn test_piecewise_match() {
        assert_latex(
            "f := x |-> match { x < 0 => 0, x >= 0 => x }",
            r"\[\mathrm{f} \coloneq x \mapsto \begin{cases} 0 & \text{if } x < 0, \\ x & \text{if } x \ge 0 \\ \end{cases}\]",
        );
    }
}
