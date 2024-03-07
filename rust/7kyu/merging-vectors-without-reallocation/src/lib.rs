//! https://www.codewars.com/kata/617dcb2f242452004a77c653/train/rust

pub fn merge<'a: 'c, 'b: 'c, 'c: 'a + 'b>(xs: &'a Vec<usize>, ys: &'b Vec<usize>) -> Vec<&'c usize> {
    xs.iter().chain(ys).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        assert_eq!(merge(&vec![1], &vec![2]), vec![&1,&2]);
        assert_eq!(merge(&vec![1,2,3], &vec![2,4,5]), vec![&1,&2,&3,&2,&4,&5]);
        assert_eq!(merge(&Vec::<usize>::new(), &Vec::<usize>::new()), Vec::<&usize>::new());
        assert_eq!(merge(&Vec::<usize>::new(), &vec![1]), vec![&1]);
    }
}
