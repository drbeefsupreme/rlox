use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug)]
pub struct ValueArray {
    values: Vec<Value>,
}

impl ValueArray {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }

    pub fn write(&mut self, value: Value) -> usize {
        let count = self.values.len();
        self.values.push(value);
        count
    }

    pub fn free(&mut self) {
        self.values = Vec::new();
    }

    pub fn print_value(&self, which: usize) {
        print!("{}", self.values[which]);
    }

    pub fn read_value(&self, which: usize) -> &Value {
        &self.values[which]
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Value {
    Number(f64),
    Bool(bool),
    Nil,
    Str(String),
}

impl Value {
    pub fn is_number(&self) -> bool {
        if let Value::Number(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_falsey(&self) -> bool {
        matches!(self, Value::Nil | Value::Bool(false))
    }
}

impl Clone for Value {
    fn clone(&self) -> Self {
        match self {
            Value::Number(n) => Value::Number(*n),
            Value::Bool(b)   => Value::Bool(*b),
            Value::Nil       => Value::Nil,
            Value::Str(s)    => Value::Str(s.clone()),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Value::Bool(b) => write!(f, "{b}"),
            Value::Number(n) => write!(f, "{n}"),
            Value::Nil => write!(f, "nil"),
            Value::Str(s) => write!(f, "{s}"),
        }
    }
}

impl Add for Value {
    type Output = Value;

    fn add(self, other: Value) -> Value {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a + b),
            (Value::Str(a), Value::Str(b)) => Value::Str(a + &b),
            _ => panic!("Operands must be two numbers or two strings"),
        }
    }
}

impl Sub for Value {
    type Output = Value;

    fn sub(self, other: Value) -> Value {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a - b),
            _ => panic!("Operands must be two numbers"),
        }
    }
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, other: Value) -> Value {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a * b),
            _ => panic!("Operands must be two numbers"),
        }
    }
}

impl Div for Value {
    type Output = Value;

    fn div(self, other: Value) -> Value {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a / b),
            _ => panic!("Operands must be two numbers"),
        }
    }
}

impl Neg for Value {
    type Output = Value;

    fn neg(self) -> Value {
        match self {
            Value::Number(a) => Value::Number(-a),
            _ => panic!("Operand must be a number"),
        }
    }
}
