//! https://www.codewars.com/kata/610ab162bd1be70025d72261/train/rust

pub fn ideal_trader(prices: &[f64]) -> f64 {
    let mut i = 0;
    let mut p = 1_f64;
    let mut b = None;
    
    while i < prices.len() {
        if i == prices.len() - 1 {
            if b.is_some() && b.unwrap() < prices[i] {
                p *= prices[i] / b.unwrap();
            }
        } else if prices[i] < prices[i+1] {
            if b.is_none() {
                b = Some(prices[i]);
            }
        } else {
            if b.is_some() {
                p *= prices[i] / b.unwrap();
                b = None;   
            }
        }
        
        i += 1;
    }

    p
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn cant_make_any_profit_in_flat_market() {
        let prices = [1.09, 1.09, 1.09, 1.09];
        assert_approx_eq(ideal_trader(&prices), 1.0);
    }

    #[test]
    fn minimal_example_of_profit() {
        let prices = [2.0, 3.0];
        assert_approx_eq(ideal_trader(&prices), 1.5);
    }

    #[test]
    fn example_from_the_description() {
        let prices = [1.0, 1.0, 1.2, 0.8, 0.9, 1.0];
        assert_approx_eq(ideal_trader(&prices), 1.5);
    }

    // Translated from Python test framework
    // in order to compare floats with an absolute and relative error margin.
    fn assert_approx_eq(actual: f64, expected: f64) {
        const MARGIN: f64 = 0.001;
        let div: f64 = f64::max(f64::max(actual.abs(), expected.abs()), 1.0);
        let ok: bool = ((actual - expected) / div).abs() < MARGIN;
        assert!(ok,
"actual:   {actual}
expected: {expected}
(+/- absolute or relative margin of {MARGIN})
"
        );
    }
}
