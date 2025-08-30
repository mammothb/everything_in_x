struct Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        *tokens
            .iter()
            .fold(Vec::<i32>::new(), |mut acc, c| {
                if let Ok(n) = c.parse() {
                    acc.push(n);
                } else {
                    let right = acc.pop().unwrap();
                    let left = acc.pop().unwrap();
                    acc.push(match c.as_str() {
                        "+" => left + right,
                        "-" => left - right,
                        "*" => left * right,
                        "/" => left / right,
                        _ => unreachable!(),
                    });
                }
                acc
            })
            .last()
            .unwrap()
    }
}

fn main() {
    println!(
        "{}",
        Solution::eval_rpn(
            vec![
                "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"
            ]
            .into_iter()
            .map(|c| c.into())
            .collect()
        )
    );
}
