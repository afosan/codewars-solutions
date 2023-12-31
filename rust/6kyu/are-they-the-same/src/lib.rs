//! https://www.codewars.com/kata/550498447451fbbd7600041c/train/rust

pub fn comp(a: Vec<i64>, b: Vec<i64>) -> bool {
    let mut a_squared = a.iter().map(|n| n * n).collect::<Vec<i64>>();
    let mut b = b;
    
    a_squared.sort();
    b.sort();
    
    a_squared == b
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing(a: Vec<i64>, b: Vec<i64>, exp: bool) -> () {
        assert_eq!(comp(a, b), exp)
    }

    #[test]
    fn tests_comp() {
    
        let a1 = vec![121, 144, 19, 161, 19, 144, 19, 11];
        let a2 = vec![11*11, 121*121, 144*144, 19*19, 161*161, 19*19, 144*144, 19*19];
        testing(a1, a2, true);
        let a1 = vec![121, 144, 19, 161, 19, 144, 19, 11];
        let a2 = vec![11*21, 121*121, 144*144, 19*19, 161*161, 19*19, 144*144, 19*19];
        testing(a1, a2, false);
    
    }
}
