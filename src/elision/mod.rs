pub fn return_str(s1: &str) -> &str {
    s1
}

// returnwith multi
pub fn return_str_with_multi<'a, 'b>(s1: &'a str, s2: &'b str) -> &'a str {
    s1
}
