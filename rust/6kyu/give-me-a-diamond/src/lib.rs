//! https://www.codewars.com/kata/5503013e34137eeeaa001648/train/rust

pub fn print(n: i32) -> Option<String> {
    if n < 0 || n % 2 == 0 {
        None
    } else if n == 1 {
        Some("*\n".to_string())
    } else {
        let k = ((n - 1) / 2) as usize;

        let v = (0..k).map(|i| format!("{}{}", " ".repeat(k - i), "*".repeat(2 * i + 1))).collect::<Vec<_>>();

        Some(
            vec![
                v.join("\n"),
                "*".repeat(n.try_into().unwrap()).to_string(),
                v.iter().rev().cloned().collect::<Vec<_>>().join("\n"),
                "".to_string(),
            ].join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
      assert_eq!(print(3), Some(" *\n***\n *\n".to_string()) );
      assert_eq!(print(5), Some("  *\n ***\n*****\n ***\n  *\n".to_string()) );
      assert_eq!(print(-3),None);
      assert_eq!(print(2),None);
      assert_eq!(print(0),None);
      assert_eq!(print(1), Some("*\n".to_string()) );
    }
}
