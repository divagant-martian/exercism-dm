pub fn encode(source: &str) -> String {
    if source.is_empty() {
        return String::new();
    }
    let (last_c, count, answer) =
        source
            .chars()
            .fold(('.', 0, String::new()), |(last_c, count, acc), c| {
                if last_c == c {
                    return (last_c, count + 1, acc);
                }
                // Write the encoding when there are no more repeated characters
                match count {
                    0 => (c, 1, acc),
                    1 => (c, 1, acc + &last_c.to_string()),
                    _ => (c, 1, acc + &count.to_string() + &last_c.to_string()),
                }
            });
    // Handle the last char case
    if count == 1 {
        return answer + &last_c.to_string();
    }
    answer + &count.to_string() + &last_c.to_string()
}

pub fn decode(source: &str) -> String {
    source
        .chars()
        .fold(('.', 0, String::new()), |(last_c, count, acc), c| {
            if let Some(n) = c.to_digit(10) {
                return (last_c, 10 * count + n, acc);
            }
            (c, 0, acc + &c.to_string().repeat(count.max(1) as usize))
        })
        .2
}
