//! https://www.codewars.com/kata/5a5cdb07fd56cbdd3c00005b/train/rust

use std::collections::HashSet;

pub fn find_dups_miss(arr: &[u32]) -> (u32, Vec<u32>) {
    let mut hs = HashSet::<u32>::new();
    let mut dups = vec![];
    
    arr.iter().for_each(|&a| {
        if !hs.insert(a) {
            dups.push(a);
        }
    });
    
    dups.sort_unstable();
    
    let minn = hs.iter().min().unwrap();
    let maxx = hs.iter().max().unwrap();
    let summ = hs.iter().sum::<u32>();
    let missing = (minn + maxx) * (maxx - minn + 1) / 2 - summ;
    
    (missing, dups)
}

#[cfg(test)]
mod tests {
    use super::find_dups_miss;
        
    fn dotest(a: &[u32], expected: (u32, Vec<u32>)) {
        let actual = find_dups_miss(a);
        assert!(actual == expected, 
            "With arr = {a:?}\nExpected {expected:?} but got {actual:?}")
    }

    #[test]
    fn fixed_tests() {
        dotest(&[10,9,8,9,6,1,2,4,3,2,5,5,3], (7, vec![2, 3, 5, 9]));
        dotest(&[20,19,6,9,7,17,16,17,12,5,6,8,9,10,14,13,11,14,15,19],(18, vec![6, 9, 14, 17, 19]));
        dotest(&[24,25,34,40,38,26,33,29,50,31,33,56,35,36,53,49,57,27,37,40,48,44,32,35,45,52,43,47,26,51,55,28,41,42,46,51,25,30,44,54],
            (39, vec![25, 26, 33, 35, 40, 44, 51]));
    }
}
