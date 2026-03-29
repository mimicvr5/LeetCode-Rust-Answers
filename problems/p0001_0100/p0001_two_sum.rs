use std::collections::HashMap;

impl Solution {
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut h = HashMap::new();

        for (i, num) in nums.iter().enumerate() {
            let desired = target - num;

            if let Some(&j) = h.get(&desired) {
                return vec![j as i32, i as i32];
            }

            h.insert(*num, i);
        }

        vec![]
    }
}
