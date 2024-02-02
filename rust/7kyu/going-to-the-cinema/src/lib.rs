//! https://www.codewars.com/kata/562f91ff6a8b77dfe900006e/train/rust

pub fn movie(card: i32, ticket: i32, perc: f64) -> i32 {
    let card = card as f64;
    let ticket = ticket as f64;
    let mut cost1 = 0_f64;
    let mut cost2 = card;
    let mut prev = ticket;
    let mut i = 0_i32;

    while cost2.ceil() >= cost1 {
        i += 1;
        prev *= perc;
        cost1 += ticket;
        cost2 += prev;
    }

    i
}

#[cfg(test)]
    mod tests {
    use super::*;

    fn dotest(card: i32, ticket: i32, perc: f64, exp: i32) -> () {
        println!(" card: {:?};", card);
        println!("ticket: {:?};", ticket);
        println!("perc: {:?};", perc);
        let ans = movie(card, ticket, perc);
        println!("actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!(" {};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic_tests() {
        dotest(500, 15, 0.9, 43);
        dotest(100, 10, 0.95, 24);
        dotest(0, 10, 0.95, 2);
        
    }
}
