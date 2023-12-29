//! https://www.codewars.com/kata/55911ef14065454c75000062/train/rust

pub fn multiply(a: &str, b: &str) -> String {
    let a = a.chars().rev().map(|c| c.to_digit(10).expect("unexpected char") as u8).collect::<Vec<u8>>();
    let b = b.chars().rev().map(|c| c.to_digit(10).expect("unexpected char") as u8).collect::<Vec<u8>>();

    let muls = (0usize..).zip(b.iter()).map(|(i, d)| multiply_vec_with_digit(&a, *d, i)).collect::<Vec<_>>();

    let sum = muls.into_iter().reduce(|acc, i| sum_vec_with_vec(acc, i)).unwrap().iter().rev().skip_while(|u| **u == 0u8).map(|u| u.to_string()).collect::<String>();
    
    if sum == "".to_string() {
        "0".to_string()
    } else {
        sum
    }
}

fn multiply_vec_with_digit(ds: &Vec<u8>, n: u8, s: usize) -> Vec<u8> {
    let (mut out, carry) = ds.iter().fold((Vec::new(), 0u8), |(mut out, mut carry), d| {
        let m = d * n + carry;
        carry = m / 10;
        out.push(m % 10);
        (out, carry)
    });

    if carry > 0 {
        out.push(carry);
    }

    vec![0; s].into_iter().chain(out.into_iter()).collect()
}

fn sum_vec_with_vec(a: Vec<u8>, b: Vec<u8>) -> Vec<u8> {
    let mut out = Vec::with_capacity(a.len().max(b.len()) + 1);
    let mut carry = 0u8;
    let mut a_iter = a.iter();
    let mut b_iter = b.iter();

    loop {
        match (a_iter.next(), b_iter.next()) {
            (Some(aval), Some(bval)) => {
                let s = carry + aval + bval;
                carry = s / 10;
                let d = s % 10;
                out.push(d);
            },
            (Some(aval), None) => {
                let s = carry + aval;
                carry = s / 10;
                let d = s % 10;
                out.push(d);
            },
            (None, Some(bval)) => {
                let s = carry + bval;
                carry = s / 10;
                let d = s % 10;
                out.push(d);
            },
            (None, None) => {
                out.push(carry);
                break;
            },
        }
    }

    out
}

#[cfg(test)]
mod sample_tests {
    use super::multiply;
    
    fn do_test(a: &str, b: &str, expected: &str) {
        let actual = multiply(&a, &b);
        assert_eq!(actual, expected,
               "\n\nMultiplying a*b with\na = {a}\nb = {b}\nshould return: {expected}\ninstead got: {actual}");
    }

    #[test]
    fn simple_cases() {
        //        input       expected
        do_test("2",  "3",     "6");
        do_test("30", "69",    "2070");
        do_test("11", "85",    "935");
    }
    
    #[test]
    fn edge_cases() { 
        do_test("2", "0",       "0");
        do_test("0", "30",      "0");
        do_test("0000001", "3", "3");
        do_test("1009", "03",   "3027");
    }
    
    #[test]
    fn big_numbers() {
        do_test("98765", "56894", "5619135910");
        do_test("9007199254740991", "9007199254740991", "81129638414606663681390495662081");
        do_test("1020303004875647366210", "2774537626200857473632627613", 
                "2830869077153280552556547081187254342445169156730");
        do_test("58608473622772837728372827", "7586374672263726736374",
                "444625839871840560024489175424316205566214109298");
    }
}
