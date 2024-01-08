//! https://www.codewars.com/kata/56a4872cbb65f3a610000026/train/rust

pub fn max_rot(n: u64) -> u64 {
    let mut num_digits = if n == 0 { 1 } else { 0 };
    let mut num = n;
    while num > 0 {
        num_digits += 1;
        num /= 10;
    }

    (0..num_digits).fold((n, n), |(num, max_so_far), i| {
        let ten_power1 = 10u64.pow(num_digits - i);
        let ten_power2 = 10u64.pow(num_digits - 1 - i);
        let left = num / ten_power1 * ten_power1;
        let rest = num % ten_power1;
        let middle = rest / ten_power2;
        let right = rest % ten_power2;
        let new_num = left + 10 * right + middle;
        let new_max_so_far = max_so_far.max(new_num);
        (new_num, new_max_so_far)
    }).1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(max_rot(56789), 68957);
        assert_eq!(max_rot(38458215), 85821534);
        assert_eq!(max_rot(195881031), 988103115);
        assert_eq!(max_rot(896219342), 962193428);
        assert_eq!(max_rot(69418307), 94183076);
    }
}
