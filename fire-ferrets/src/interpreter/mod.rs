pub mod env;
pub mod value;

use env::*;
use value::*;

use crate::lexer::*;
use crate::parser::*;

type ValueResult = Result<Value, String>;
type StmtResult = Result<(), String>;

pub trait Visitor {
    fn visit_expr(&mut self, expr: &Expr) -> ValueResult;
    fn visit_stmt(&mut self, stmt: &Stmt) -> StmtResult;
}

pub struct Interpreter {
    env: Env,
}

impl Interpreter {
    pub fn new() -> Self {
        Self { env: Env::new() }
    }

    pub fn run(&mut self, stmts: &[Stmt]) -> StmtResult {
        for stmt in stmts {
            self.visit_stmt(stmt)?;
        }
        Ok(())
    }

    fn visit_literal(lit: &Lit) -> Value {
        match lit {
            Lit::Int(x) => Value::Int(*x),
            Lit::Str(x) => Value::Str(x.to_string()),
            Lit::Bool(x) => Value::Bool(*x),
        }
    }

    fn visit_binary_op(&mut self, op: &TokenKind, lhs: &Expr, rhs: &Expr) -> ValueResult {
        let lhs = self.visit_expr(lhs).unwrap();
        self.eval_infix_expression(op, lhs, rhs)
    }

    fn eval_infix_expression(&mut self, op: &TokenKind, lhs: Value, rhs: &Expr) -> ValueResult {
        match lhs {
            Value::Int(_) => {
                if let TokenKind::And | TokenKind::Or = op {
                    self.eval_infix_short_circuiting(op, lhs, rhs)
                } else {
                    self.eval_infix_integer(op, lhs, rhs)
                }
            }
            Value::Str(_) => todo!(),
            Value::Comment(_) => todo!(),
            Value::Bool(_) => self.eval_infix_short_circuiting(op, lhs, rhs),
        }
    }

    fn eval_infix_integer(&mut self, op: &TokenKind, lhs: Value, rhs: &Expr) -> ValueResult {
        let rhs = self.visit_expr(rhs).unwrap();
        Ok(match op {
            TokenKind::Plus => Value::Int(lhs.to_int()? + rhs.to_int()?),
            TokenKind::Minus => Value::Int(lhs.to_int()? - rhs.to_int()?),
            TokenKind::Multiply => Value::Int(lhs.to_int()? * rhs.to_int()?),
            TokenKind::Divide => Value::Int(lhs.to_int()? / rhs.to_int()?),
            TokenKind::Equals => Value::Bool(lhs.to_int()? == rhs.to_int()?),
            TokenKind::NotEq => Value::Bool(lhs.to_int()? != rhs.to_int()?),
            TokenKind::Less => Value::Bool(lhs.to_int()? < rhs.to_int()?),
            TokenKind::LessEq => Value::Bool(lhs.to_int()? <= rhs.to_int()?),
            TokenKind::Greater => Value::Bool(lhs.to_int()? > rhs.to_int()?),
            TokenKind::GreaterEq => Value::Bool(lhs.to_int()? >= rhs.to_int()?),
            _ => unreachable!(),
        })
    }

    fn eval_infix_short_circuiting(
        &mut self,
        op: &TokenKind,
        lhs: Value,
        rhs: &Expr,
    ) -> ValueResult {
        Ok(match op {
            TokenKind::And => Value::Bool(bool::from(lhs) && bool::from(self.visit_expr(rhs)?)),
            TokenKind::Or => Value::Bool(bool::from(lhs) || bool::from(self.visit_expr(rhs)?)),
            _ => unreachable!(),
        })
    }

    fn visit_ident(&mut self, name: &str) -> ValueResult {
        if name == "pop" {
            self.env.pop()
        } else {
            self.env.get(name)
        }
    }

    fn visit_unary_op(&mut self, op: &TokenKind, rhs: &Expr) -> ValueResult {
        let value = self.visit_expr(rhs).unwrap();
        match value {
            Value::Int(_) | Value::Bool(_) => Self::eval_prefix_op(op, value),
            Value::Str(_) => todo!(),
            Value::Comment(_) => todo!(),
        }
    }

    fn eval_prefix_op(op: &TokenKind, rhs: Value) -> ValueResult {
        Ok(match op {
            TokenKind::Minus => Value::Int(-rhs.to_int()?),
            TokenKind::Not => Value::Bool(!bool::from(rhs)),
            _ => todo!(),
        })
    }

    fn visit_set(&mut self, name: &str, expr: &Expr) -> StmtResult {
        let name = name.to_string();
        if &name == "pop" {
            return Err("Error: Cannot assign to reserved identifier 'pop'".to_string());
        }
        let value = self.visit_expr(expr)?;
        self.env.set(name, value);
        Ok(())
    }

    fn visit_push(&mut self, expr: &Expr) -> StmtResult {
        let value = self.visit_expr(expr)?;
        self.env.push(value);
        Ok(())
    }

    fn visit_check(&mut self, expr: &Expr) -> StmtResult {
        let value = bool::from(self.visit_expr(expr)?);
        self.env.push(Value::Bool(value));
        Ok(())
    }

    fn visit_pop(&mut self) -> StmtResult {
        self.env.pop()?;
        Ok(())
    }

    fn visit_print(&mut self, expr: &Expr) -> StmtResult {
        let value = self.visit_expr(expr)?;
        println!("{}", value);
        Ok(())
    }
}

impl Visitor for Interpreter {
    fn visit_expr(&mut self, expr: &Expr) -> ValueResult {
        match expr {
            Expr::Literal(lit) => Ok(Self::visit_literal(lit)),
            Expr::Ident(name) => self.visit_ident(name),
            Expr::BinaryOp(op, lhs, rhs) => self.visit_binary_op(op, lhs, rhs),
            Expr::UnaryOp(op, rhs) => self.visit_unary_op(op, rhs),
        }
    }

    fn visit_stmt(&mut self, stmt: &Stmt) -> StmtResult {
        match stmt {
            Stmt::Set(name, expr) => self.visit_set(name, expr),
            Stmt::Push(expr) => self.visit_push(expr),
            Stmt::Check(expr) => self.visit_check(expr),
            Stmt::Pop => self.visit_pop(),
            Stmt::Print(expr) => self.visit_print(expr),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        interpreter::{Interpreter, Visitor},
        lexer::TokenKind,
        parser::{Expr, Lit},
    };

    use super::value::Value;

    #[test]
    fn test_integer_literal() {
        let input = Expr::Literal(Lit::Int(5));
        let expected = Value::Int(5);
        test_expr(input, expected);
    }

    #[test]
    fn test_boolean_literal() {
        let input = Expr::Literal(Lit::Bool(true));
        let expected = Value::Bool(true);
        test_expr(input, expected);
        let input = Expr::Literal(Lit::Bool(false));
        let expected = Value::Bool(false);
        test_expr(input, expected);
    }

    #[test]
    fn test_string_literal() {
        let input = Expr::Literal(Lit::Str("hello".to_string()));
        let expected = Value::Str("hello".to_string());
        test_expr(input, expected);
    }

    #[test]
    fn test_binary_op() {
        let input = Expr::BinaryOp(
            TokenKind::Plus,
            Box::new(Expr::Literal(Lit::Int(5))),
            Box::new(Expr::Literal(Lit::Int(10))),
        );
        let expected = Value::Int(15);
        test_expr(input, expected);
    }

    #[test]
    fn test_unary_op() {
        let input = Expr::UnaryOp(TokenKind::Minus, Box::new(Expr::Literal(Lit::Int(5))));
        let expected = Value::Int(-5);
        test_expr(input, expected);
    }

    fn test_expr(input: Expr, expected: Value) {
        let mut interpreter = Interpreter::new();
        let actual = interpreter.visit_expr(&input);
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), expected);
    }

    #[test]
    fn test_integer_equals() {
        let expected = Value::Int(5);
        let actual = Value::Int(5);
        assert_eq!(actual, expected);
        assert_ne!(actual, Value::Int(8));
    }

    #[test]
    fn test_boolean_equals() {
        let expected = Value::Bool(true);
        let actual = Value::Bool(true);
        assert_eq!(actual, expected);
        assert_ne!(actual, Value::Bool(false));
    }

    #[test]
    fn test_and_or() {
        let input = Expr::BinaryOp(
            TokenKind::Or,
            Box::new(Expr::BinaryOp(
                TokenKind::And,
                Box::new(Expr::Literal(Lit::Bool(true))),
                Box::new(Expr::Literal(Lit::Bool(true))),
            )),
            Box::new(Expr::Literal(Lit::Bool(false))),
        );
        let expected = Value::Bool(true);
        test_expr(input, expected);
    }
}
