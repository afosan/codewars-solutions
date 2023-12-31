//! https://www.codewars.com/kata/513e08acc600c94f01000001/train/rust

fn clip(n: i32) -> u8 {
    n.max(u8::MIN as i32).min(u8::MAX as i32) as u8
}

pub fn rgb(r: i32, g: i32, b: i32) -> String {
    format!("{:0>2X}{:0>2X}{:0>2X}", clip(r), clip(g), clip(b))
}
  
macro_rules! compare {
    ( $got : expr, $expected : expr ) => {
        if $got != $expected { panic!("Got: {}\nExpected: {}\n", $got, $expected); }
    };
}

#[cfg(test)]
mod sample_tests {
    use self::super::*;

    #[test]
    fn tests() {
        compare!(rgb(0, 0, 0), "000000");
        compare!(rgb(1, 2, 3), "010203");
        compare!(rgb(255, 255, 255), "FFFFFF");
        compare!(rgb(254, 253, 252), "FEFDFC");
        compare!(rgb(-20, 275, 125), "00FF7D");
    }
}
