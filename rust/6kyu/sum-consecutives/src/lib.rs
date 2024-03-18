//! https://www.codewars.com/kata/55eeddff3f64c954c2000059/train/rust

/// Just to make code nicer
type Number = i32;
type Numbers = Vec<Number>;

/// Sums the numbers that are the same and consecutive.
pub fn sum_consecutives(numbers: &Numbers) -> Numbers {
    let mut last_number: Option<&Number> = None;
    let mut count = 0;
    let mut out: Numbers = vec![];
    
    numbers
        .iter()
        .for_each(|n| {
            if last_number.is_none() || last_number.unwrap() == n {
                count += 1;
            } else {
                out.push(count * last_number.unwrap());
                count = 1;
            }
            last_number = Some(n);
        });
    out.push(count * last_number.unwrap());

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        println!("Sample Tests");

        let input = vec![1, 4, 4, 4, 0, 4, 3, 3, 1];
        let expected = vec![1, 12, 0, 4, 6, 1];
        println!("Input: {:?}", input);
        assert_eq!(sum_consecutives(&input), expected);

        let input = vec![-5, -5, 7, 7, 12, 0];
        let expected = vec![-10, 14, 12, 0];
        println!("Input: {:?}", input);
        assert_eq!(sum_consecutives(&input), expected);
    }
}
