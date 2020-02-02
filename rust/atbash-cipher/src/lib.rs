/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let mut acc = String::new();
    plain.chars().fold(0, |count, c| {
        if count > 0 && count % 5 == 0 && c.is_ascii_alphanumeric() {
            acc.push(' ');
        }
        add_inverse(&mut acc, c) + count
    });
    acc
}

/// adds the corresponding inverse of c to the accumulator acc and returns the
/// number of characters that were added
fn add_inverse(acc: &mut String, c: char) -> usize {
    match c {
        'A'..='Z' | 'a'..='z' => {
            acc.push((122 - c.to_ascii_lowercase() as u8 + 97) as char);
            1
        }
        '0'..='9' => {
            acc.push(c);
            1
        }
        _ => 0,
    }
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    let mut acc = String::new();
    for c in cipher.chars() {
        add_inverse(&mut acc, c);
    }
    acc
}
