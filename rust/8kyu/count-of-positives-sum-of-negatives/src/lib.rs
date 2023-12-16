//! https://www.codewars.com/kata/576bb71bbbcf0951d5000044/train/rust

pub fn count_positives_sum_negatives(input: Vec<i32>) -> Vec<i32> {
    if input.len() == 0 {
        return vec![];
    }

    let (p_count, n_sum) = input.iter().fold((0i32, 0i32), |(acc0, acc1), &n| {
        if n < 0 {
            (acc0, acc1 + n)
        } else if n > 0 {
            (acc0 + 1, acc1)
        } else {
            (acc0, acc1)
        }
    });

    vec![p_count, n_sum]
}

#[cfg(test)]
mod tests {
    use super::count_positives_sum_negatives;
        
    fn dotest(a: &[i32], expected: &[i32]) {
        let actual = count_positives_sum_negatives(a.to_vec());
        assert!(actual == expected, 
            "With input = {a:?}\nExpected {expected:?} but got {actual:?}")
    }
    #[test]
    fn fixed_tests() {
        dotest(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, -11, -12, -13, -14, -15], &[10, -65]);
        dotest(&[], &[]);
        dotest(&[0, 2, 3, 0, 5, 6, 7, 8, 9, 10, -11, -12, -13, -14], &[8, -50]);
        dotest(&[0,1,2,3,4,5], &[5, 0]);
        dotest(&[1,2,3,4,5], &[5, 0]);
        dotest(&[0,-1,-2,-3,-4,-5], &[0, -15]);
        dotest(&[-1,-2,-3,-4,-5], &[0, -15]);
        dotest(&[0,0,0,0], &[0,0]);
        dotest(&[0], &[0,0]);
    }
}
