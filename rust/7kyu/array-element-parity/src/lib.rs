use std::collections::HashSet;

pub fn solve(arr: &Vec<i32>) -> i32 {
    let mut seen = HashSet::<i32>::new();
    
    arr.iter().for_each(|&a| {
        if !seen.remove(&-a) {
            seen.insert(a);
        }
    });
    
    *seen.iter().next().unwrap()
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn sample_tests() {
        assert_eq!(solve(&vec![1,-1,2,-2,3]), 3);
        assert_eq!(solve(&vec![-3,1,2,3,-1,-4,-2]), -4);
        assert_eq!(solve(&vec![1,-1,2,-2,3,3]), 3);
        assert_eq!(solve(&vec![-110,110,-38,-38,-62,62,-38,-38,-38]), -38);
        assert_eq!(solve(&vec![-9,-105,-9,-9,-9,-9,105]), -9);
    }
}
