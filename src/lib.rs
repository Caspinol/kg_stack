/*

Rust test

 */

mod kg_stack_test;
use std::vec::Vec;
use std::result::Result;

pub struct KGStack<T> {
    list: Vec<T>,
}

trait StackStuff<T> {
    fn push(&mut self, x: T);
    fn pop(&mut self) -> Result<T,i8>;
    fn size(&self) -> usize;
}

impl<T> StackStuff<T> for KGStack<T> {

    fn push(&mut self, x: T) {
        self.list.push(x);
    }

    fn pop(&mut self) -> Result<T,i8> {
        match self.list.is_empty() {
            true => Err(-1),
            false => Ok(self.list.pop().unwrap())
        }
    }
    
    fn size(&self) -> usize {   
        return self.list.len();
    }
}

impl<T> KGStack<T> {
    pub fn new() -> KGStack<T> {
        KGStack {
            list: Vec::new()
        }
    }
}
