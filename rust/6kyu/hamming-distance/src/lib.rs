//! https://www.codewars.com/kata/5410c0e6a0e736cf5b000e69/train/rust

pub fn hamming(a: &str, b: &str) -> usize {
    a.chars().zip(b.chars()).filter(|(ac, bc)| ac != bc).count()
}

#[cfg(test)]
mod tests {
    use super::*;
 
    #[test]
    fn simple_hamming_tests() {
        // Translated from the JavaScript test cases.
        assert_eq!(hamming("", ""), 0);
        assert_eq!(hamming("I like turtles", "I like turkeys"), 3);
        assert_eq!(hamming("Hello World", "Hello World"), 0);
        assert_eq!(hamming("a man a plan a canal", "a man a plan sobanal"), 3);
        assert_eq!(hamming("hamming and cheese", "Hamming and Cheese"), 2);
        assert_eq!(hamming("espresso", "Expresso"), 2);
        assert_eq!(
            hamming("old father, old artificer", "of my soul the uncreated "),
            24
        );
    }
}
