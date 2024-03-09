//! https://www.codewars.com/kata/5b71af678adeae41df00008c/train/rust

pub fn shortest_distance(a: f64, b: f64, c: f64) -> f64 {
    vec![
        a.powi(2) + (b + c).powi(2),
        b.powi(2) + (a + c).powi(2),
        c.powi(2) + (a + b).powi(2),
    ].iter().min_by(|a, b| a.total_cmp(b)).unwrap().sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn should_approximate(actual: f64, expected: f64) {
        const EPSILON: f64 = 1e-9;
        float_eq::assert_float_eq!(actual, expected, r2nd <= EPSILON);
    }

    #[test]
    fn test_fixed() {
        should_approximate(shortest_distance(1.0, 2.0, 3.0), 4.242640687);
        should_approximate(shortest_distance(1.0, 1.0, 1.0), 2.236067978);
        should_approximate(shortest_distance(134.0, 191.5, 45.5), 262.4738082);
    }
}
