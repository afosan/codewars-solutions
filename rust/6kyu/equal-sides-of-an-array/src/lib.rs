//! Equal Sides Of An Array

pub fn find_even_index(arr: &[i32]) -> Option<usize> {
    let s: i32 = arr.iter().sum();
    let mut t = 0;

    for (i, n) in arr.iter().enumerate() {
        if (s - n) / 2 == t {
            return Some(i);
        }
        t += n;
    }

    return None;
}

#[cfg(test)]
mod tests {
    use super::find_even_index;
    
    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";
   
    fn dotest(arr: &[i32], expected: Option<usize>) {
        assert_eq!(find_even_index(arr), expected, "{ERR_MSG} with arr = {arr:?}")   
    }

    #[test]
    fn fixed_tests() {
        dotest(&[1, 2, 3, 4, 3, 2, 1], Some(3));
        dotest(&[1, 100, 50, -51, 1, 1], Some(1));
        dotest(&[1, 2, 3, 4, 5, 6], None);
        dotest(&[20, 10, 30, 10, 10, 15, 35], Some(3));
        dotest(&[20, 10, -80, 10, 10, 15, 35], Some(0));
        dotest(&[10, -80, 10, 10, 15, 35, 20], Some(6));
        dotest(&(1..100).collect::<Vec<_>>(), None);
        dotest(&[0, 0, 0, 0, 0], Some(0));
        dotest(&[-1, -2, -3, -4, -3, -2, -1], Some(3));
        dotest(&(-100..-1).collect::<Vec<_>>(), None);
        dotest(&[8, 8], None);
        dotest(&[8, 0], Some(0));
        dotest(&[0, 8], Some(1));
        dotest(&[7, 3, -3], Some(0));
        dotest(&[8], Some(0));
        dotest(&[10, -10], None);
        dotest(&[-3, 2, 1, 0], Some(3));
        dotest(&[-15, 5, 11, 17, 19, -17, 20, -6, 17, -17, 19, 16, -15, -6, 20, 17], Some(8));
    }
}
