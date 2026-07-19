struct MinStack {
    data: Vec<i32>,
    mins: Vec<i32>,
}

impl MinStack {
    pub fn new() -> Self {
        Self {
            data: vec![],
            mins: vec![],
        }
    }

    pub fn push(&mut self, val: i32) {
        self.data.push(val);
        let min = if let Some(&curr) = self.mins.last() {
            val.min(curr)
        } else {
            val
        };
        self.mins.push(min);
    }

    pub fn pop(&mut self) {
        self.data.pop();
        self.mins.pop();
    }

    pub fn top(&self) -> i32 {
        self.data.last().copied().unwrap()
    }

    pub fn get_min(&self) -> i32 {
        self.mins.last().copied().unwrap()
    }
}

fn main() {}
