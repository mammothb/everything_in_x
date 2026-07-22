struct Solution;

impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut pos_and_spd: Vec<(i32, i32)> = position.into_iter().zip(speed.into_iter()).collect();
        pos_and_spd.sort_unstable_by_key(|(pos, _)| -pos);
        let mut stack = vec![];
        for (pos, spd) in pos_and_spd.into_iter() {
            let time = (target - pos) as f64 / spd as f64;
            if let Some(&last) = stack.last() && time <= last {
                continue;
            }
            stack.push(time);
        }
        stack.len() as i32
    }
}

fn main() {
}
