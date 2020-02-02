struct Primes {
    primes: Vec<u64>,
}

impl Primes {
    pub fn is_prime(&self, n: u64) -> bool {
        let sqrt = (n as f64).sqrt() as u64 + 1;
        for p in &self.primes {
            if p > &sqrt {
                return true;
            }
            if n % p == 0 {
                return false;
            }
        }
        true
    }
}

impl Iterator for Primes {
    type Item = u64;

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

pub fn factors(n: u64) -> Vec<u64> {
    let mut factors = Vec::<u64>::new();
    let mut check = n;
    let mut primes = Primes { primes: vec![] };
    while check > 1 {
        if let Some(p) = primes.next() {
            while check % p == 0 {
                factors.push(p);
                check /= p;
            }
        }
    }
    factors
}
