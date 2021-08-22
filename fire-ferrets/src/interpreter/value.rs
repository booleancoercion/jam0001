use std::fmt;

#[derive(Clone, Debug)]
pub enum Value {
    Int(i64),
    Str(String),
    Comment(String),
    Bool(bool),
}

impl Value {
    pub fn to_int(&self) -> Result<i64, String> {
        match self {
            Value::Int(n) => Ok(*n),
            _ => Err("Error: Expected numerical expression".to_string()),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Value::Int(v) => v.to_string(),
                Value::Str(v) => v.to_string(),
                Value::Comment(v) => v.to_string(),
                Value::Bool(v) => v.to_string(),
            }
        )
    }
}

impl PartialEq<Value> for Value {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Value::Int(i) => {
                if let Value::Int(j) = other {
                    i == j
                } else {
                    false
                }
            }
            Value::Str(a) => {
                if let Value::Str(b) = other {
                    a == b
                } else {
                    false
                }
            }
            Value::Comment(_) => todo!(),
            Value::Bool(a) => {
                if let Value::Bool(b) = other {
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
            Value::Bool(b) => b,
            Value::Int(n) => n != 0_i64,
            Value::Str(s) => s.is_empty(),
            Value::Comment(_) => todo!(),
        }
    }
}
