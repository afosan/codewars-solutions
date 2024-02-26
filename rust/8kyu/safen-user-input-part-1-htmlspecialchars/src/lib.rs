//! https://www.codewars.com/kata/56bcaedfcf6b7f2125001118/train/rust

pub fn html_special_chars(html: &str) -> String {
    html
        .chars()
        .map(|c|
            match c {
                '<' => "&lt;".to_string(),
                '>' => "&gt;".to_string(),
                '"' => "&quot;".to_string(),
                '&' => "&amp;".to_string(),
                _ => c.to_string(),
            }
        )
        .collect()
}

#[cfg(test)]
mod tests {
    use super::html_special_chars;
    
    #[test]
    fn sample_tests() {
        assert_eq!(html_special_chars("<h2>Hello World</h2>"), 
            "&lt;h2&gt;Hello World&lt;/h2&gt;");
        assert_eq!(html_special_chars("Hello, how would you & I fare?"), 
            "Hello, how would you &amp; I fare?");
        assert_eq!(html_special_chars("How was \"The Matrix\"?  Did you like it?"), 
            "How was &quot;The Matrix&quot;?  Did you like it?");
        assert_eq!(html_special_chars("<script>alert('Website Hacked!');</script>"), 
            "&lt;script&gt;alert('Website Hacked!');&lt;/script&gt;");
    }
}
