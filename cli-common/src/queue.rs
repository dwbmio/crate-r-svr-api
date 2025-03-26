#[derive(Debug)]
pub struct Queue<T> {
    older: Vec<T>,
    newer: Vec<T>,
    is_circle: bool,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Self {
            newer: Vec::new(),
            older: Vec::new(),
            is_circle: false,
        }
    }

    pub fn enable_circle(&mut self) {
        self.is_circle = true;
    }

    pub fn push(&mut self, c: T) {
        self.newer.push(c);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_circle {
            if self.older.is_empty() {
                if self.newer.is_empty() {
                    return None;
                }
                // Bring the elements in younger over to older, and put them in
                // the promised order.
                use std::mem::swap;
                swap(&mut self.older, &mut self.newer);
                self.older.reverse(); //为什么要颠倒一下呢？ 因为push/pop是栈操作，和队列顺序正相反
            }
            return self.older.pop();
        }
        return self.newer.pop();
    }

    pub fn is_empty(&self) -> bool {
        if self.is_circle {
            self.older.is_empty() && self.newer.is_empty()
        } else {
            self.newer.is_empty()
        }
    }

    pub fn len(&mut self) -> usize {
        self.newer.len()
    }
}
