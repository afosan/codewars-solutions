//! https://www.codewars.com/kata/54fb853b2c8785dd5e000957/train/rust

pub fn chain<F: Fn(i32)->i32>(input: i32, functions: &[F]) -> i32 {
    functions.iter().fold(input, |acc, f| f(acc))
}

#[cfg(test)]
mod sample_tests {
    use super::chain;
    
    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";
    
    fn add10(x: i32) -> i32 { x + 10 }
    fn mul30(x: i32) -> i32 { x * 30 }
    fn sub5(x: i32)  -> i32 { x - 5 }
    fn xor3(x: i32)  -> i32 { x ^ 3 }

    #[test]
    fn empty_chain() {
        assert_eq!(chain::<&dyn Fn(i32)->i32>(42, &[]), 42,
            "{ERR_MSG} for the given chain:\n(x), where x = 42"
        );
    }
    
    #[test]
    fn single_fn() {
        assert_eq!(chain(50, &[&mul30]), 1500,
            "{ERR_MSG} for the given chain:\n(x * 30), where x = 50"
        );
    }
    
    #[test]
    fn two_fns() {
        let fns: Vec<Box<dyn Fn(i32)->i32>> = vec![Box::new(mul30), Box::new(add10)];
        assert_eq!(chain(50, &fns), 1510,
            "{ERR_MSG} for the given chain:\n((x * 30) + 10), where x = 50"
        );
    }
    
    #[test]
    fn simple_chain() {
        let fns: Vec<Box<dyn Fn(i32)->i32>> = vec![
            Box::new(add10),
            Box::new(mul30), Box::new(xor3),
            Box::new(sub5)
        ];
        assert_eq!(chain(50, &fns), 1798,
            "{ERR_MSG} for the given chain:\n((((x + 10) * 30) ^ 3) - 5), where x = 50");
    }
}
