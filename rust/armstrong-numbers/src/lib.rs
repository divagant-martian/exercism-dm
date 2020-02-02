pub fn is_armstrong_number(num: u32) -> bool {
    let mut n = num;
    let mut digits = Vec::<u32>::new();
    while n > 0 {
        let d = n % 10;
        digits.push(d);
        n = (n - d) / 10;
    }
    digits
        .iter()
        .map(|d| d.pow(digits.len() as u32))
        .sum::<u32>()
        == num
}
