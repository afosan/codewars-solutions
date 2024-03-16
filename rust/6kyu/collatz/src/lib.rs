//! https://www.codewars.com/kata/5286b2e162056fd0cb000c20/train/rust

pub fn collatz(n: u32) -> String {
    let mut n = n;
    let mut v = vec![n.to_string()];

    while n > 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }
        v.push(n.to_string());
    }
    
    v.join("->")
}

#[cfg(test)]
mod tests {
    use super::collatz;

    #[test]
    fn test() {
        assert_eq!(collatz(3), "3->10->5->16->8->4->2->1")
    }
}
