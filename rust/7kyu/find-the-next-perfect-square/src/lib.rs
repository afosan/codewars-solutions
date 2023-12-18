pub fn find_next_square(sq: u64) -> Option<u64> {
    let sq_sqrt = (sq as f64).sqrt();
    let sq_sqrt_floor = sq_sqrt.floor();
    
    if sq_sqrt_floor == sq_sqrt {
        Some(((sq_sqrt_floor as u64) + 1).pow(2))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::find_next_square;
    
    #[test]
    fn sample_tests() {
        assert_eq!(find_next_square(121), Some(144));
        assert_eq!(find_next_square(625), Some(676));
        assert_eq!(find_next_square(319_225), Some(320_356));
        assert_eq!(find_next_square(15_241_383_936), Some(15_241_630_849));
        assert_eq!(find_next_square(155), None);
        assert_eq!(find_next_square(342_786_627), None);
    }
}
