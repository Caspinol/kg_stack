#[cfg(test)]

use super::*;

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
    assert!(stack.pop() == Some("Sraka".to_string()));
    assert!(stack.size() == 1);
    assert!(stack.pop() == Some("Dupa".to_string()));
    assert!(stack.size() == 0);
    assert!(stack.pop() == None);
}

#[test]
fn peek_stack_test(){
    let mut stack = KGStack::<String>::new();
    stack.push("Dupa".to_string());
    stack.push("Sraka".to_string());

    assert!(stack.size() == 2);
    assert!(stack.peek() == Some(&"Sraka".to_string()));
    assert!(stack.size() == 2);
}
