//! https://www.codewars.com/kata/541629460b198da04e000bb9/train/rust

pub fn last<T: Clone>(slice: &[T]) -> T {
    slice.iter().last().expect("expected non-empty input").clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work_for_non_empty_list_of_integers() {
      assert_eq!(last(&[1, 2, 3]), 3);
    }
    
    #[test]
    fn should_work_for_non_empty_list_of_strings() {
      assert_eq!(last(&["a", "b"]), "b");
    }
}
