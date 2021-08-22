use core::fmt;
use std::collections::HashMap;

use crate::lexer::*;
use crate::parser::*;

#[derive(Debug)]
pub enum Value {
    Integer(i64),
    String(String),
    Comment(String),
    Boolean(bool),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Value::Integer(v) => v.to_string(),
                Value::String(v) => v.to_string(),
                Value::Comment(v) => v.to_string(),
                Value::Boolean(v) => v.to_string(),
            }
        )
    }
}

impl PartialEq<Value> for Value {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Value::Integer(i) => {
                if let Value::Integer(j) = other {
                    i == j
                } else {
                    false
                }
            }
            Value::String(a) => {
                if let Value::String(b) = other {
                    a == b
                } else {
                    false
                }
            }
            Value::Comment(_) => todo!(),
            Value::Boolean(a) => {
                if let Value::Boolean(b) = other {
                    a == b
                } else {
                    false
                }
            }
        }
    }
}

impl From<Value> for bool {
    fn from(value: Value) -> Self {
        match value {
            Value::Boolean(b) => b,
            Value::Integer(n) => n != 0_i64,
            Value::String(s) => s.is_empty(),
            Value::Comment(_) => todo!(),
        }
    }
}

impl From<Value> for i64 {
    fn from(value: Value) -> Self {
        match value {
            Value::Integer(n) => n,
            _ => panic!("Expected numerical expression"),
        }
    }
}

pub struct Env {
    store: HashMap<String, Value>,
    stack: Vec<Value>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            stack: vec![],
            store: HashMap::new(),
        }
    }

    pub fn set(&mut self, name: String, value: Value) {
        self.store.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Result<&Value, String> {
        self.store.get(name).ok_or(format!("{} is undefined", name))
    }

    pub fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    pub fn pop(&mut self) -> Result<Value, String> {
        self.stack
            .pop()
            .ok_or_else(|| "The stack is empty".to_string())
    }
}

pub trait Visitor {
    fn visit_expr(&self, e: &Expr) -> Value;
}

pub struct Interpreter {
    env: Env,
}

impl Interpreter {
    pub fn new() -> Self {
        Self { env: Env::new() }
    }

    pub fn eval_expr(&self, input: &Expr) -> Option<Value> {
        Some(self.visit_expr(input))
    }

    fn walk_literal_expr(lit: &Lit) -> Value {
        match lit {
            Lit::Int(x) => Value::Integer(*x),
            Lit::Str(x) => Value::String(x.to_string()),
            Lit::Bool(x) => Value::Boolean(*x),
        }
    }

    fn walk_binary_expr(&self, token: &TokenKind, lhs: &Expr, rhs: &Expr) -> Value {
        // TODO: check for erros
        let left = self.eval_expr(lhs).unwrap();
        let right = self.eval_expr(rhs).unwrap();
        Interpreter::eval_infix_expression(token, left, right)
    }

    fn eval_infix_expression(token: &TokenKind, lhs: Value, rhs: Value) -> Value {
        match lhs {
            Value::Integer(_) => Self::eval_infix_integer(token, lhs, rhs),
            Value::String(_) => todo!(),
            Value::Comment(_) => todo!(),
            Value::Boolean(_) => todo!(),
        }
    }

    fn eval_infix_integer(token: &TokenKind, lhs: Value, rhs: Value) -> Value {
        match token {
            TokenKind::Plus => Value::Integer(i64::from(lhs) + i64::from(rhs)),
            TokenKind::Minus => Value::Integer(i64::from(lhs) - i64::from(rhs)),
            TokenKind::Multiply => Value::Integer(i64::from(lhs) * i64::from(rhs)),
            TokenKind::Divide => Value::Integer(i64::from(lhs) / i64::from(rhs)),
            TokenKind::Equals => Value::Boolean(i64::from(lhs) == i64::from(rhs)),
            TokenKind::NotEq => Value::Boolean(i64::from(lhs) != i64::from(rhs)),
            TokenKind::Less => Value::Boolean(i64::from(lhs) < i64::from(rhs)),
            TokenKind::LessEq => Value::Boolean(i64::from(lhs) <= i64::from(rhs)),
            TokenKind::Greater => Value::Boolean(i64::from(lhs) > i64::from(rhs)),
            TokenKind::GreaterEq => Value::Boolean(i64::from(lhs) >= i64::from(rhs)),
            _ => todo!(), // TODO: Unsopported operator error
        }
    }

    fn walk_ident_expr(name: &str) -> Value {
        Value::Integer(5)
    }

    fn walk_unary_op_expr(&self, token: &TokenKind, rhs: &Expr) -> Value {
        let value = self.eval_expr(rhs).unwrap();
        match value {
            Value::Integer(_) => Self::eval_prefix_integer(token, value),
            Value::String(_) => todo!(),
            Value::Comment(_) => todo!(),
            Value::Boolean(_) => Self::eval_prefix_bool(token, value),
        }
    }

    fn eval_prefix_integer(token: &TokenKind, rhs: Value) -> Value {
        match token {
            TokenKind::Minus => Value::Integer(-i64::from(rhs)),
            _ => todo!(), // TODO: error handling
        }
    }

    fn eval_prefix_bool(token: &TokenKind, rhs: Value) -> Value {
        match token {
            TokenKind::Not => Value::Boolean(!bool::from(rhs)),
            _ => todo!(),
        }
    }
}

impl Visitor for Interpreter {
    fn visit_expr(&self, e: &Expr) -> Value {
        match e {
            Expr::Literal(lit) => Interpreter::walk_literal_expr(lit),
            Expr::Ident(name) => Interpreter::walk_ident_expr(name),
            Expr::BinaryOp(t, lhs, rhs) => self.walk_binary_expr(t, lhs, rhs),
            Expr::UnaryOp(t, rsh) => self.walk_unary_op_expr(t, rsh),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_literal() {
        let input = Expr::Literal(Lit::Int(5));
        let expected = Value::Integer(5);
        test_expr(input, expected);
    }

    #[test]
    fn test_boolean_literal() {
        let input = Expr::Literal(Lit::Bool(true));
        let expected = Value::Boolean(true);
        test_expr(input, expected);
        let input = Expr::Literal(Lit::Bool(false));
        let expected = Value::Boolean(false);
        test_expr(input, expected);
    }

    #[test]
    fn test_string_literal() {
        let input = Expr::Literal(Lit::Str("hello".to_string()));
        let expected = Value::String("hello".to_string());
        test_expr(input, expected);
    }

    #[test]
    fn test_binary_op() {
        let input = Expr::BinaryOp(
            TokenKind::Plus,
            Box::new(Expr::Literal(Lit::Int(5))),
            Box::new(Expr::Literal(Lit::Int(10))),
        );
        let expected = Value::Integer(15);
        test_expr(input, expected);
    }

    #[test]
    fn test_unary_op() {
        let input = Expr::UnaryOp(TokenKind::Minus, Box::new(Expr::Literal(Lit::Int(5))));
        let expected = Value::Integer(-5);
        test_expr(input, expected);
    }

    fn test_expr(input: Expr, expected: Value) {
        let interpreter = Interpreter::new();
        let actual = interpreter.eval_expr(&input);
        assert!(actual.is_some());
        assert_eq!(actual.unwrap(), expected);
    }

    #[test]
    fn test_integer_equals() {
        let expected = Value::Integer(5);
        let actual = Value::Integer(5);
        assert_eq!(actual, expected);
        assert_ne!(actual, Value::Integer(8));
    }

    #[test]
    fn test_boolean_equals() {
        let expected = Value::Boolean(true);
        let actual = Value::Boolean(true);
        assert_eq!(actual, expected);
        assert_ne!(actual, Value::Boolean(false));
    }
}
