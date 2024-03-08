//! https://www.codewars.com/kata/64060d8ab2dd990058b7f8ee/train/rust

pub fn queens(n: i64) -> i64 {
    if n < 3 { 0 } else { n * (n - 3) + 2 }
}

#[test]
fn test_queens() {
    assert_eq!(queens(0), 0);
    assert_eq!(queens(1), 0);
    assert_eq!(queens(2), 0);
    assert_eq!(queens(3), 2);
    assert_eq!(queens(4), 6);
    assert_eq!(queens(5), 12);
    assert_eq!(queens(6), 20);
    assert_eq!(queens(20), 342);
    assert_eq!(queens(33), 992);
    assert_eq!(queens(50), 2352);
    assert_eq!(queens(-312350), 0);
    assert_eq!(queens(-64550), 0);
    assert_eq!(queens(-330), 0);
    assert_eq!(queens(3123214), 9754456320156);
}
