//! https://www.codewars.com/kata/5121303128ef4b495f000001/train/rust

struct Person<'a> {
    name: &'a str,
}

impl<'a> Person<'a> {
    fn new(name: &'a str) -> Self {
        Person { name }
    }

    fn greet(&self, other: &str) -> String {
        format!("Hello {}, my name is {}", other, self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::Person;

    #[test]
    fn greet() {
        let alice = Person::new("Alice");
        assert_eq!(alice.name, "Alice");
        assert_eq!(alice.greet("Bob"), "Hello Bob, my name is Alice");
    }
    
    #[test]
    fn name() {
        let charlie = Person::new("Charlie");
        assert_eq!(charlie.name, "Charlie");
    }
}
