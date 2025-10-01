struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];
        for c in s.chars() {
            if c == '(' || c == '[' || c == '{' {
                stack.push(c);
                continue;
            }
            if let Some(c2) = stack.pop() {
                if !((c2 == '(' && c == ')') || (c2 == '[' && c == ']') || (c2 == '{' && c == '}'))
                {
                    return false;
                }
            } else {
                return false;
            }
        }
        stack.is_empty()
    }
}

fn main() {
    println!("Hello, world!");
}
