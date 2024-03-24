//! https://www.codewars.com/kata/58c8af49fd407dea5f000042/train/rust

pub fn luxhouse(houses: &[u32]) -> Vec<u32> {
    let mut max_so_far = u32::MIN;
    let reversed = houses.iter().rev().map(|&n| {
        if n > max_so_far {
            max_so_far = n;
            0
        } else {
            max_so_far + 1 - n
        }
    }).collect::<Vec<_>>();
    
    reversed.into_iter().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::luxhouse;
        
    fn dotest(a: &[u32], expected: &[u32]) {
        let actual = luxhouse(a);
        assert!(actual == expected, 
            "With houses = {a:?}\nExpected {expected:?} but got {actual:?}")
    }

    #[test]
    fn fixed_tests() {
        dotest(&[1, 2, 3, 1, 2], &[3, 2, 0, 2, 0]);
        dotest(&[3, 2, 1, 4], &[2, 3, 4, 0]);
        dotest(&[1, 2, 3], &[3, 2, 0]);
        dotest(&[3, 2, 1], &[0, 0, 0]);
        dotest(&[1, 1, 1], &[1, 1, 0]);
    }
}
