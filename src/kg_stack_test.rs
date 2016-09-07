#[cfg(test)]

use super::*;
use StackStuff;

#[test]
fn create_new_stack_object(){
    let stack = KGStack::<char>::new();
    assert!(stack.size() == 0);
}

#[test]
fn push_stack_test(){
    let mut stack = KGStack::<String>::new();
    stack.push("Dupa".to_string());
    stack.push("Sraka".to_string());

    assert!(stack.size() == 2);
    assert!(stack.pop() == Ok("Sraka".to_string()));
    assert!(stack.pop() == Ok("Dupa".to_string()));
    assert!(stack.pop() == Err(-1));
}
