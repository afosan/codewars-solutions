//! https://www.codewars.com/kata/59a96d71dbe3b06c0200009c/train/rust

pub fn generate_shape(n: i32) -> String {
    let n = n as usize;
    vec!["+".repeat(n); n].join("\n")
}

#[cfg(test)]
mod tests {
    use super::generate_shape;
        
    fn dotest(n: i32, expected: &str) {
        let actual = generate_shape(n);
        assert!(actual == expected, 
            "With n = {n}\nExpected \"{expected}\" but got \"{actual}\"")
    }
    
    fn reference_solution(n: i32) -> String {
        (0 .. n)
        .map(|_x| "+".repeat(n as usize)
        )
        .collect::<Vec<String>>().join("\n")
    }
    
    #[test]
    fn sample_test() {
      dotest(3, "+++\n+++\n+++")
    }
}
