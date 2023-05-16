use crate::{expr::{ExprVisitor, Expr}, token::{Literal, TokenType}, error::LoxError};

pub struct Interpreter {}

impl ExprVisitor<Literal> for Interpreter {
    fn visit_binary_expr(&self, expr: &crate::expr::BinaryExpr) -> Result<Literal, crate::error::LoxError> {
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;
        let op = expr.operator.get_token_type();

        let result = match (left, right) {
            (Literal::Number(left), Literal::Number(right)) => match op {
                TokenType::Minus => Literal::Number(left - right),
                TokenType::Slash => Literal::Number(left / right),
                TokenType::Star => Literal::Number(left * right),
                TokenType::Plus => Literal::Number(left + right),
                TokenType::Greater => Literal::Bool(left > right),
                TokenType::GreaterEqual => Literal::Bool(left >= right),
                TokenType::Less => Literal::Bool(left < right),
                TokenType::LessEqual => Literal::Bool(left <= right),
                TokenType::BangEqual => Literal::Bool(left != right),
                TokenType::Equal => Literal::Bool(left == right),
                _ => {
                    todo!("need to work on your code dude");
                }
            },
            (Literal::Number(left), Literal::String(right)) => match op {
                TokenType::Plus => Literal::String(format!("{left}{right}")),
                _ => Literal::ArithmeticError,
            },
            (Literal::String(left), Literal::Number(right)) => match op {
                TokenType::Plus => Literal::String(format!("{left}{right}")),
                _ => Literal::ArithmeticError,
            },
            (Literal::String(left), Literal::String(right)) => match op {
                TokenType::Plus => Literal::String(format!("{left}{right}")),
                TokenType::BangEqual => Literal::Bool(left != right),
                TokenType::Equal => Literal::Bool(left == right),
                _ => Literal::ArithmeticError,
            },
            (Literal::Bool(left), Literal::Bool(right)) => match op {
                TokenType::BangEqual => Literal::Bool(left != right),
                TokenType::Equal => Literal::Bool(left == right),
                _ => Literal::ArithmeticError,
            },
            (Literal::Nil, Literal::Nil) => match op {
                TokenType::BangEqual => Literal::Bool(false),
                TokenType::Equal => Literal::Bool(true),
                _ => Literal::ArithmeticError,
            },
            (Literal::Nil, _) => match op {
                TokenType::Equal => Literal::Bool(false),
                TokenType::BangEqual => Literal::Bool(true),
                _ => Literal::ArithmeticError,
            },
            _ => Literal::ArithmeticError,
        };

        if result == Literal::ArithmeticError {
            Err(LoxError::new_with_token(
                expr.operator.clone(),
                "Illegal expression".to_string(),
            ))
        } else {
            Ok(result)
        }
    }

    fn visit_grouping_expr(&self, expr: &crate::expr::GroupingExpr) -> Result<Literal, crate::error::LoxError> {
        Ok(self.evaluate(&expr.expression)?)
    }

    fn visit_literal_expr(&self, expr: &crate::expr::LiteralExpr) -> Result<Literal, crate::error::LoxError> {
        Ok(expr.value.clone().unwrap())
    }

    fn visit_unary_expr(&self, expr: &crate::expr::UnaryExpr) -> Result<Literal, crate::error::LoxError> {
        let right = self.evaluate(&expr.right)?;

        match expr.operator.get_token_type() {
            TokenType::Minus => match right {
                Literal::Number(n) => return Ok(Literal::Number(-n)),
                _ => return Ok(Literal::Nil),
            },
            TokenType::Bang => Ok(Literal::Bool(!self.is_truthy(&right))),
            _ => Err(LoxError::new_with_token(expr.operator.clone(), "Unreachable according to Nystrom".to_string(),))
        }
    }
}

impl Interpreter {
    fn evaluate(&self, expr: &Expr) -> Result<Literal, LoxError> {
        expr.accept(self)
    }

    fn is_truthy(&self, literal: &Literal) -> bool {
        !matches!(literal, Literal::Nil | Literal::Bool(false))
    }

    pub fn interpret(&self, expr: &Expr) -> bool {
        match self.evaluate(&expr) {
            Ok(v) => {
                println!("{}", v);
                true
            }
            Err(e) => {
                false
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{token::*, expr::{LiteralExpr, UnaryExpr, BinaryExpr}};

    fn make_literal(o: Literal) -> Box<Expr> {
        Box::new(Expr::Literal(LiteralExpr { value: Some(o) }))
    }

    fn make_literal_string(s: &str) -> Box<Expr> {
        make_literal(Literal::String(s.to_string()))
    }

    #[test]
    fn test_unary_minus() {
        let terp = Interpreter {};
        let unary_expr = UnaryExpr {
            operator: Token::new(TokenType::Minus, "-".to_string(), None, 123),
            right: make_literal(Literal::Number(123.0)),
        };
        let result = terp.visit_unary_expr(&unary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Literal::Number(-123.0)));
    }

    #[test]
    fn test_equals_string() {
        let terp = Interpreter {};
        let binary_expr = BinaryExpr {
            left: make_literal_string("world"),
            operator: Token::new(TokenType::Equal, "==".to_string(), None, 123),
            right: make_literal_string("world"),
        };
        let result = terp.visit_binary_expr(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Literal::Bool(true)));
    }
}