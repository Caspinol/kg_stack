/*

Rust test

 */

mod kg_stack_test;
use std::vec::Vec;

pub struct KGStack<T> {
    list: Vec<T>,
}

pub trait StackStuff<T> {
    fn push(&mut self, x: T);
    fn pop(&mut self) -> Option<T>
;
    fn peek(&self) -> Option<&T>;
    fn size(&self) -> usize;
}

impl<T> StackStuff<T> for KGStack<T> {

    fn push(&mut self, x: T) {
        self.list.push(x);
    }

    fn pop(&mut self) -> Option<T> {
        self.list.pop()
    }

    fn peek(&self) -> Option<&T>{
        self.list.get(self.list.len()-1)
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
