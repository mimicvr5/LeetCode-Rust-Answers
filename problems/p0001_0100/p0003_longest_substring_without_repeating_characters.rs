use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut map = HashMap::new();
        let mut left: usize = 0;
        let mut max_len = 0;

        for (right, c) in s.chars().enumerate() {
            if let Some(&prev_index) = map.get(&c) {
                if prev_index >= left {
                    left = prev_index + 1;
                }
            }

            map.insert(c, right);
            max_len = max_len.max(right - left + 1);
        }

        max_len as i32
    }
}
