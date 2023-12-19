//! https://www.codewars.com/kata/5541f58a944b85ce6d00006a/train/rust

pub fn product_fib(prod: u64) -> (u64, u64, bool) {
    let mut a0 = 0;
    let mut a1 = 1;
    
    while a0 * a1 < prod {
        (a0, a1) = (a1, a0 + a1);
    }
    
    (a0, a1, a0 * a1 == prod)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(prod: u64, exp: (u64, u64, bool)) -> () {
        assert_eq!(product_fib(prod), exp)
    }
    
    #[test]
    fn basics_product_fib() {
        dotest(4895, (55, 89, true));
        dotest(5895, (89, 144, false));
    }
}
