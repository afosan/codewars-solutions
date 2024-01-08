//! https://www.codewars.com/kata/598106cb34e205e074000031/train/rust

pub fn count_deaf_rats(town: &str) -> u8 {
    let s = town.chars().filter(|c| match c {
        '~' | 'O' | 'P' => true,
        _ => false,
    }).collect::<String>();
    let index = s.chars().enumerate().find(|(_, c)| *c == 'P').expect("there must be 'P' in the input").0;
    let left = s[..index].chars().collect::<Vec<char>>().chunks(2).filter(|pair| pair[0] == 'O' && pair[1] == '~').count();
    let right = s[index + 1..].chars().collect::<Vec<char>>().chunks(2).filter(|pair| pair[1] == 'O' && pair[0] == '~').count();

    (left + right) as u8
}

#[cfg(test)]
mod tests {
    use super::count_deaf_rats;

    #[test]
    fn ex1() {
        assert_eq!(count_deaf_rats("~O~O~O~O P"), 0);
    }
    #[test]
    fn ex2() {
        assert_eq!(count_deaf_rats("P O~ O~ ~O O~"), 1);
    }
    #[test]
    fn ex3() {
        assert_eq!(count_deaf_rats("~O~O~O~OP~O~OO~"), 2);
    }
}
