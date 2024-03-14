use std::collections::HashMap;
use std::cmp::Reverse;

pub fn solve(vec: &[i32]) -> Vec<i32> {
    let mut c = HashMap::<i32, usize>::new();
    
    vec.iter().for_each(|n| *c.entry(*n).or_insert(0) += 1);
    
    let mut v = vec.to_vec();
    v.sort_unstable_by_key(|n| (Reverse(c.get(n).unwrap()), n.clone()));
    
    v
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn basic_tests() {
        assert_eq!(solve(&vec![2,3,5,3,7,9,5,3,7]), vec![3,3,3,5,5,7,7,2,9]);
        assert_eq!(solve(&vec![1,2,3,0,5,0,1,6,8,8,6,9,1]), vec![1,1,1,0,0,6,6,8,8,2,3,5,9]);
        assert_eq!(solve(&vec![5,9,6,9,6,5,9,9,4,4]), vec![9,9,9,9,4,4,5,5,6,6]);
        assert_eq!(solve(&vec![4,4,2,5,1,1,3,3,2,8]), vec![1,1,2,2,3,3,4,4,5,8]);
        assert_eq!(solve(&vec![4,9,5,0,7,3,8,4,9,0]), vec![0,0,4,4,9,9,3,5,7,8]);
    }
}
