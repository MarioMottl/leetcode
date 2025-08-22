struct MinStack {
    stack: Vec<i32>,
    mins: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(unused)]
impl MinStack {
    fn new() -> Self {
        Self {
            stack: Vec::new(),
            mins: Vec::new(),
        }
    }

    fn push(&mut self, val: i32) {
        self.stack.push(val);
        if let Some(&m) = self.mins.last() {
            self.mins.push(m.min(val));
        } else {
            self.mins.push(val);
        }
    }

    fn pop(&mut self) {
        self.stack.pop();
        self.mins.pop();
    }

    fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        *self.mins.last().unwrap()
    }
}

fn main() {}
