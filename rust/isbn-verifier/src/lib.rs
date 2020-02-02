/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    if isbn.len() > 13 {
        return false;
    }
    let (coef, sum) = isbn.chars().fold((10, 0), |(coef, acc), c| {
        if coef == 0 {
            return (0, 1);
        }
        if let Some(n) = c.to_digit(10) {
            return (coef - 1, acc + coef * n);
        }
        if c == 'X' && coef == 1 {
            return (coef - 1, acc + coef * 10);
        }
        (coef, acc)
    });
    coef == 0 && sum % 11 == 0
}
