#![forbid(unsafe_code)]

use std::collections::VecDeque;

#[derive(Default)]
pub struct MinQueue<T> {
    queue: VecDeque<T>,
    min: VecDeque<T>,
}

impl<T: Clone + Ord> MinQueue<T> {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            min: VecDeque::new(),
        }
    }

    pub fn push(&mut self, val: T) {
        while let Some(min) = self.min.back() {
            if val >= *min {
                break;
            }
            self.min.pop_back();
        }

        self.queue.push_back(val.clone());
        self.min.push_back(val);
    }

    pub fn pop(&mut self) -> Option<T> {
        let popped = self.queue.pop_front();

        if popped.as_ref() == self.min.front() {
            self.min.pop_front();
        }

        popped
    }

    pub fn front(&self) -> Option<&T> {
        self.queue.front()
    }

    pub fn min(&self) -> Option<&T> {
        self.min.front()
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}
