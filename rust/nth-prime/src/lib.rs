struct Primes {
    primes: Vec<u32>,
}

impl Primes {
    pub fn new(n: usize) -> Self {
        Primes {
            primes: Vec::with_capacity(n + 1),
        }
    }

    pub fn is_prime(&self, n: u32) -> bool {
        let sqr = (n as f32).sqrt() as u32 + 1;
        for i in &self.primes {
            if i > &sqr {
                return true;
            }
            if n % i == 0 {
                return false;
            }
        }
        true
    }
}

impl Iterator for Primes {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(l) = self.primes.last() {
            for n in (l + 1).. {
                if self.is_prime(n) {
                    self.primes.push(n);
                    return Some(n);
                }
            }
            None
        } else {
            self.primes.push(2);
            Some(2)
        }
    }
}

pub fn nth(n: u32) -> u32 {
    Primes::new(n as usize).nth(n as usize).unwrap()
}
