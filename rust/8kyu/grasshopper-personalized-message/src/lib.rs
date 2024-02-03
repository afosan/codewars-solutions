//! https://www.codewars.com/kata/5772da22b89313a4d50012f7/train/rust

pub fn greet(name: &str, owner: &str) -> String {
    format!("Hello {}", if name == owner { "boss" } else { "guest" })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        assert_eq!(greet("Daniel", "Daniel"), "Hello boss");
        assert_eq!(greet("Greg", "Daniel"), "Hello guest");
    }
}
