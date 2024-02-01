use std::collections::HashMap;

fn main() {
    let s = String::from("()");
    let result = is_valid(s);
    println!("{}",result);
}

pub fn is_valid(s: String) -> bool {
    let mut stack: Vec<char> = Vec::new();

    for c in s.chars() {
        match c {
            '(' | '{' | '[' => stack.push(c),
            ')' => if stack.pop() != Some('(') { return false },
            '}' => if stack.pop() != Some('{') { return false },
            ']' => if stack.pop() != Some('[') { return false },
            _ => {},
        }
    }
    stack.is_empty()
}