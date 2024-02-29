//! https://www.codewars.com/kata/595aa94353e43a8746000120/train/rust

pub fn find_deleted_number(list: &[u16], mixed_list: &[u16]) -> Option<u16> {
    let num = list.into_iter().map(|&a| a as u64).sum::<u64>() - mixed_list.into_iter().map(|&a| a as u64).sum::<u64>();
    
    if num == 0 { None } else { Some(num as u16) }
}

#[cfg(test)]
mod tests {
    use super::find_deleted_number;

    #[test]
    fn basic() {
        assert_eq!(
            find_deleted_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9], &[3, 2, 4, 6, 7, 8, 1, 9]),
            Some(5)
        );
        assert_eq!(
            find_deleted_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9], &[3, 2, 4, 6, 7, 8, 9, 5]),
            Some(1)
        );
        assert_eq!(
            find_deleted_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9], &[3, 2, 4, 1, 7, 8, 9, 5]),
            Some(6)
        );
    }
}
