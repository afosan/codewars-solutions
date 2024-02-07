//! https://www.codewars.com/kata/5a1ebc2480171f29cf0000e5/train/rust

use either::Either;

fn area(shape: &Either<(f64, f64), f64>) -> f64 {
    match shape {
        Either::Left((width, length)) => width * length,
        Either::Right(r) => std::f64::consts::PI * r * r,
    }
}

pub fn sort_by_area(seq: &[Either<(f64, f64), f64>]) -> Vec<Either<(f64, f64), f64>> {
    let mut out = seq.to_vec();
    out.sort_by(|a, b| area(a).partial_cmp(&area(b)).unwrap());

    out
}


#[cfg(test)]
mod tests {
    use super::sort_by_area;
    use either::Either;
        
    fn dotest(seq: &[Either<(f64, f64), f64>], expected: &[Either<(f64, f64), f64>]) {
        let actual = sort_by_area(seq);
        assert!(actual == expected, "With seq = {seq:?}\nExpected {expected:?} but got {actual:?}")
    }

    #[test]
    fn fixed_tests() {
        dotest(&[Either::Left((4.23, 6.43)), Either::Right(1.23), Either::Right(3.444), Either::Left((1.342, 3.212))], &[Either::Left((1.342, 3.212)), Either::Right(1.23), Either::Left((4.23, 6.43)), Either::Right(3.444)]);
        dotest(&[Either::Left((2.0, 5.0)), Either::Right(6.0)], &[Either::Left((2.0, 5.0)), Either::Right(6.0) ]);
        dotest(&[], &[]);
    }
}
