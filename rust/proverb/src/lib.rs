pub fn build_proverb(list: &[&str]) -> String {
    if let Some(first) = list.first() {
        return list.windows(2).fold(String::new(), |acc, s| {
            acc + &format!("For want of a {} the {} was lost.\n", s[0], s[1])
        }) + &format!("And all for the want of a {}.", first);
    }
    String::new()
}
