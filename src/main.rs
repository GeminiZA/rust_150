struct Solution;

use std::collections::VecDeque;
impl Solution {
    
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: VecDeque<i32> = VecDeque::new();
        for token in tokens {
            if let Ok(num) = token.parse::<i32>() {
                stack.push_front(num.clone());
            } else {
                if token == "+" {
                    let a = stack.pop_front().unwrap();
                    let b = stack.pop_front().unwrap();
                    stack.push_front(a + b);
                } else if token == "-" {
                    let a = stack.pop_front().unwrap();
                    let b = stack.pop_front().unwrap();
                    stack.push_front(b - a);
                } else if token == "*" {
                    let a = stack.pop_front().unwrap();
                    let b = stack.pop_front().unwrap();
                    stack.push_front(a * b);
                } else if token == "/" {
                    let a = stack.pop_front().unwrap();
                    let b = stack.pop_front().unwrap();
                    stack.push_front(b / a);
                }
            }
        }
        return stack.pop_front().unwrap();
    }
}

fn main() {
    let s: Vec<String> = vec!["4","13","5","/","+"].iter().map(|val| val.to_string()).collect();
    let result = Solution::eval_rpn(s);
    println!("{:?}", result);
}
