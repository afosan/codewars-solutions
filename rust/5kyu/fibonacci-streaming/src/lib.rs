use num::BigUint;

struct Fibonacci {
    curr: BigUint,
    next: BigUint,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci { curr: BigUint::from(1_u64), next: BigUint::from(1_u64) }
    }
}

impl Iterator for Fibonacci {
    type Item = BigUint;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr.clone();

        self.curr = self.next.clone();
        self.next = current.clone() + self.next.clone();

        Some(current)
    }
}

pub fn all_fibonacci_numbers() -> impl Iterator<Item = BigUint> {
    let f = Fibonacci::new();
    f
}

#[cfg(test)]
mod tests {
    use super::*;
    use num::BigUint;

    fn test_segment(start: u32, numbers: [u128; 10]){
        let mut fib_iterator = all_fibonacci_numbers();
        for _ in 0..start{
            fib_iterator.next();
        }
        for i in numbers{
            let ans = fib_iterator.next();
            let expected = Some(BigUint::from(i));
            assert_eq!(expected, ans,
                "\nYour result (left) \ndid not match the expected output (right)");
        }
    }
    
    #[test]
    fn tests_0_10() {
        test_segment(0, [1, 1, 2, 3, 5, 8, 13, 21, 34, 55]);
    }
    
    #[test]
    fn tests_10_20() {
        test_segment(10, [89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765]);
    }
    
    #[test]
    fn tests_100_110() {
        test_segment(100, [573147844013817084101, 927372692193078999176, 1500520536206896083277, 2427893228399975082453, 3928413764606871165730, 6356306993006846248183, 10284720757613717413913, 16641027750620563662096, 26925748508234281076009, 43566776258854844738105]);
    }
}
