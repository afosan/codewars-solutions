//! https://www.codewars.com/kata/56dbe7f113c2f63570000b86/train/rust

pub fn rot(s: &str) -> String {
    s.chars().rev().collect()
}

pub fn selfie_and_rot(s: &str) -> String {
    let sub_length = s.split("\n").next().unwrap().len();
    let mut first = s.split("\n").map(|line| format!("{line}{}", ".".repeat(sub_length))).collect::<Vec<String>>();
    let mut second = rot(s).split("\n").map(|line| format!("{}{line}", ".".repeat(sub_length))).collect::<Vec<String>>();
    first.append(&mut second);

    first.join("\n")
}

pub fn oper(f: fn(&str) -> String, s: &str) -> String {
    f(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing1(s: &str, exp: &str) -> () {
        assert_eq!(oper(rot, s), exp.to_string())
    }
    fn testing2(s: &str, exp: &str) -> () {
        assert_eq!(oper(selfie_and_rot, s), exp.to_string())
    }
    
    #[test]
    fn basics_oper() {
        testing1("fijuoo\nCqYVct\nDrPmMJ\nerfpBA\nkWjFUG\nCVUfyL", 
            "LyfUVC\nGUFjWk\nABpfre\nJMmPrD\ntcVYqC\nooujif");
        testing1("rkKv\ncofM\nzXkh\nflCB", "BClf\nhkXz\nMfoc\nvKkr"); 
        
        testing2("xZBV\njsbS\nJcpN\nfVnP", 
            "xZBV....\njsbS....\nJcpN....\nfVnP....\n....PnVf\n....NpcJ\n....Sbsj\n....VBZx");
        testing2("uLcq\nJkuL\nYirX\nnwMB",
            "uLcq....\nJkuL....\nYirX....\nnwMB....\n....BMwn\n....XriY\n....LukJ\n....qcLu");
    }
}
