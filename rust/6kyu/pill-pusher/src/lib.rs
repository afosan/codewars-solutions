//! https://www.codewars.com/kata/628e6f112324192c65cd8c97/train/rust

pub fn prescribe(d: u16, a: u16, b: u16) -> u16 {
    let mut mx = 0;
    
    for ai in 0..=d/a {
        for bi in 0..=d/b {
            let temp = ai * a + bi * b;
            if temp <= d && temp > mx {
                mx = temp;
            }
        }
    }
    
    mx
}

#[cfg(test)]
mod tests {
    use super::prescribe;

    #[test]
    fn example_problem() {
        assert_eq!(prescribe(99,25,60), 85, 
            "Expected 85.  85mg can be made with 1 * 25mg pill and 1* 60mg pill, no close dose can be made that is less than or equal to 99");
    }
    #[test]
    fn one_type_needed() {
        assert_eq!(prescribe(180,25,60), 180, 
            "Expected 180.  3 * 60mg pills is exactly the target dose of 180mg");
    }
    #[test]
    fn bigger_dose() {
        assert_eq!(prescribe(2575,150,400), 2550, 
            "Expected 2550.  2550mg can be made 3 different ways. No closer dose can be made without exceeding target");
    }
    #[test]
    fn many_pills() {
        assert_eq!(prescribe(4540, 9, 15), 4539, 
            "Expected 4539");
    }
}
