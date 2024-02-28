//! https://www.codewars.com/kata/596ddaccdd42c1cf0e00005c/train/rust

pub fn calc(array: Vec<i32>) -> i32 {
    array.into_iter().enumerate().map(|(i, a)| {
        let mut res = a;

        if res > 0 {
            res *= res;
        }
        if i % 3 == 2 {
            res *= 3;
        }
        if i % 5 == 4 {
            res *= -1;
        }
        
        res
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod tests {
        use super::*;
    
        #[test]
        fn tests() {
            assert_eq!(calc(vec![0, 2, 1, -6, -3, 3]), 31);
            assert_eq!(calc(vec![0]), 0);
            assert_eq!(calc(vec![1, 1, 1, 1, 1]), 5);
            assert_eq!(calc(vec![1, 1, -9, 9, 16, -15, -45, -73, 26]), 1665);
            assert_eq!(calc(vec![1, -1, 10, -9, 16, 15, 45, -73, -26]), 2584);
            assert_eq!(calc(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), 0);
            assert_eq!(calc(vec![-5, -5, -5, -5, -5, -5, -5]), -45);
        }    
    }
}
