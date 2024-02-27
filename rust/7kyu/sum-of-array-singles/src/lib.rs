use std::collections::HashSet;

pub fn repeats(arr: &Vec<i32>) -> i32 {
    let mut seen = HashSet::<i32>::new();
    let mut sum = 0_i32;
    
    arr.iter().for_each(|a| if seen.insert(*a) {
        sum -= a;
    } else {
        sum += a;
    });
    
    -sum
}

#[cfg(test)]
mod tests {
    use super::repeats;

    #[test]
    fn basic_tests() {
        assert_eq!(repeats(&vec![4, 5, 7, 5, 4, 8]), 15);
        assert_eq!(repeats(&vec![9, 10, 19, 13, 19, 13]), 19);
        assert_eq!(repeats(&vec![16, 0, 11, 4, 8, 16, 0, 11]), 12);
        assert_eq!(repeats(&vec![5, 17, 18, 11, 13, 18, 11, 13]), 22);
        assert_eq!(repeats(&vec![5, 10, 19, 13, 10, 13]), 24);
    }
}
