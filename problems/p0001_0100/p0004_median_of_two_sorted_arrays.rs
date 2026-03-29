impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (a, b) = if nums1.len() <= nums2.len() {
            (nums1, nums2)
        } else {
            (nums2, nums1)
        };

        let m = a.len();
        let n = b.len();
        let half_len = (m + n + 1) / 2;

        let mut left = 0;
        let mut right = m;

        while left <= right {
            let i = (left + right) / 2;
            let j = half_len - i;

            let a_left = if i == 0 { std::i32::MIN } else { a[i - 1] };
            let a_right = if i == m { std::i32::MAX } else { a[i] };
            let b_left = if j == 0 { std::i32::MIN } else { b[j - 1] };
            let b_right = if j == n { std::i32::MAX } else { b[j] };

            if a_left <= b_right && b_left <= a_right {
                let max_left = a_left.max(b_left) as f64;
                if (m + n) % 2 == 1 {
                    return max_left;
                }
                let min_right = a_right.min(b_right) as f64;
                return (max_left + min_right) / 2.0;
            } else if a_left > b_right {
                right = i - 1;
            } else {
                left = i + 1;
            }
        }

        unreachable!("Input arraus are guaranteed to be valid");
    }
}
