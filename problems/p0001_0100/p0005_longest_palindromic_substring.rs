impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let chars: Vec<char> = s.chars().collect();
        let n = chars.len();
        if n == 0 { return "".to_string(); }

        let mut start = 0;
        let mut end = 0;

        for i in 0..n {
            let len1 = Self::expand(&chars, i as isize, i as isize);
            let len2 = Self::expand(&chars, i as isize, i as isize + 1);
            let len = len1.max(len2);

            if len as usize > end - start {
                start = i.saturating_sub((len as usize - 1) / 2);
                end = i + len / 2;
            }
        }

        chars[start..=end].iter().collect()
    }

    fn expand(chars: &[char], mut left: isize, mut right: isize) -> usize {
        let n = chars.len() as isize;
        while left >= 0 && right < n && chars[left as usize] == chars[right as usize] {
            left -= 1;
            right += 1;
        }
        (right - left - 1) as usize
    }
}
