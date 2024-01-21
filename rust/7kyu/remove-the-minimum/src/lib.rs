//! https://www.codewars.com/kata/563cf89eb4747c5fb100001b/train/rust

pub fn remove_smallest(numbers: &[u32]) -> Vec<u32> {
    if numbers.len() == 0 {
        return vec![];
    }

    let min_index = numbers.iter().enumerate().fold((u32::MAX, 0), |(acc_min_value, acc_min_index), (i, v)| {
        if *v < acc_min_value {
            (*v, i)
        } else {
            (acc_min_value, acc_min_index)
        }
    }).1;

    numbers.iter().enumerate().filter(|(i, _)| *i != min_index).map(|(_, v)| *v).collect()
}

#[cfg(test)]
mod tests {
    use super::remove_smallest;
    
    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";
    
    fn dotest(a: &[u32], expected: &[u32]) {
        assert_eq!(remove_smallest(a), expected, "{ERR_MSG} with numbers = {a:?}")
    }

    #[test]
    fn fixed_tests() {
        dotest(&[1, 2, 3, 4, 5], &[2, 3, 4, 5]);
        dotest(&[1, 2, 3, 4], &[2, 3, 4]);
        dotest(&[5, 3, 2, 1, 4], &[5, 3, 2, 4]);
        dotest(&[1, 2, 3, 1, 1], &[2, 3, 1, 1]);
    }
}
