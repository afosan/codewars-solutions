//! https://www.codewars.com/kata/542ebbdb494db239f8000046/train/rust

pub fn next_item<T: PartialEq<T> + Clone>(slice: &[T], find: T) -> Option<T> {
    slice.into_iter().skip_while(|s| **s != find).skip(1).next().cloned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(next_item(&["Joe", "Bob", "Sally"], "Bob"), Some("Sally"));
        assert_eq!(next_item(&[0, 1], 0), Some(1));
        assert_eq!(next_item(&[0, 1], 1), None);
        assert_eq!(next_item((1..10).collect::<Vec<_>>().as_slice(), 7), Some(8));  
    }
}
