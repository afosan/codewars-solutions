//! https://www.codewars.com/kata/5a651865fd56cb55760000e0/train/rust

pub fn array_leaders(arr: &[i32]) -> Vec<i32> {
    let mut right_sums = Vec::with_capacity(arr.len());
    let mut acc = 0;

    for a in arr.iter().rev() {
        right_sums.push(acc);
        acc += a;
    }

    arr.iter().zip(right_sums.iter().rev()).filter(|(a, right_sum)| **a > **right_sum).map(|(a, _)| *a).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        // Positive values
        assert_eq!(array_leaders(&[1,2,3,4,0]), [4]);
        assert_eq!(array_leaders(&[16,17,4,3,5,2]), [17,5,2]);
        
        // Negative values
        assert_eq!(array_leaders(&[-1,-29,-26,-2]), [-1]);
        assert_eq!(array_leaders(&[-36,-12,-27]),  [-36,-12]);
        
        // Mixed values
        assert_eq!(array_leaders(&[5,-2,2]), [5,2]);
        assert_eq!(array_leaders(&[0,-1,-29,3,2]),  [0,-1,3,2]);
    }
}
