//! https://www.codewars.com/kata/6066ae080168ff0032c4107a/train/rust

pub fn max_sum_between_two_negatives(nums: &Vec<i32>) -> Option<i32> {
    let mut max_sum_so_far = None;
    let mut sum_so_far = 0;
    let mut is_init = false;
    
    for n in nums.iter().skip_while(|n| **n > 0) {
        if *n < 0 {
            if is_init {
                max_sum_so_far = Some(max_sum_so_far.unwrap_or(0).max(sum_so_far));
                sum_so_far = 0;
            } else {
                is_init = true;
            }
        } else {
            sum_so_far += *n;
        }
        
        println!("{n} {max_sum_so_far:?} {sum_so_far}");
    }
    
    max_sum_so_far
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(max_sum_between_two_negatives(&vec![4, -1, 6, -2, 3, 5, -7, 7]), Some(8), "testing with the example from description");
        assert_eq!(max_sum_between_two_negatives(&vec![1, 2, 3, 4, 5]), None, "testing with only positive numbers");
        assert_eq!(max_sum_between_two_negatives(&vec![]), None, "testing with an empty vector");
    }
}
