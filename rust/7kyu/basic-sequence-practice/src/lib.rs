//! https://www.codewars.com/kata/5436f26c4e3d6c40e5000282/train/rust

pub fn sum_of_n(n: i32) -> Vec<i32> {
    let num = n.abs();
    let mut out = Vec::with_capacity(num as usize + 1);
    let mut acc = 0;

    for i in 0..=num {
        acc += i;
        out.push(acc);
    }
    
    if n < 0 {
        out.iter().map(|i| -i).collect()
    } else {
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_tests() {
        assert_eq!(sum_of_n(3), vec![0, 1, 3, 6]);
        assert_eq!(sum_of_n(-4), vec![0, -1, -3, -6, -10]);
        assert_eq!(sum_of_n(1), vec![0, 1]);
        assert_eq!(sum_of_n(0), vec![0]);
        assert_eq!(sum_of_n(10), vec![0, 1, 3, 6, 10, 15, 21, 28, 36, 45, 55]);
    }
}
