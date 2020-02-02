use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }
    let sum = (1..num).fold(0, |acc, n| {
        if num % n == 0 {
            print!("{}, ", n);
            return acc + n;
        }
        acc
    });
    match sum.cmp(&num) {
        Ordering::Equal => Some(Classification::Perfect),
        Ordering::Less => Some(Classification::Deficient),
        _ => Some(Classification::Abundant),
    }
}
