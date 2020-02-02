use std::collections::HashMap;

pub fn check(candidate: &str) -> bool {
    let mut count = HashMap::new();
    for c in candidate.chars() {
        if c.is_alphabetic() {
            *count.entry(c.to_ascii_lowercase()).or_insert(0) += 1;
        }
    }
    count.values().all(|&n| n == 1)
}
