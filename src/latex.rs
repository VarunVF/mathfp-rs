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
                    let left = if let Expr::Grouping(inner) = &**left {
                        inner
                    } else {
                        left
                    };
                    let right = if let Expr::Grouping(inner) = &**right {
                        inner
                    } else {
                        right
                    };
                    Ok(format!(
                        "\\frac{{{}}}{{{}}}",
                        self.expr_to_latex(left)?,
                        self.expr_to_latex(right)?
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
