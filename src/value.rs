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
        print!("{:?}", self.values[which]);
    }

    pub fn read_value(&self, which: usize) -> Value {
        self.values[which]
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Value {
    Number(f64),
    Bool(bool),
    Nil,
}

impl Value {
    pub fn print(&self) {
        print!("{:?}", self);
    }
}
