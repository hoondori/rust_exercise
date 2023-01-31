
/*  Iterator trait를 사용해서 소수 구하기  */

struct PrimeIterator {
    n: u8,
}

impl PrimeIterator {
    fn new() -> Self {
        Self {n: 1}
    }

    fn is_prime(&self) -> bool {
        // 소거법 기반 소수 여부 판단 
        for i in 2..self.n {
            if self.n % i == 0 { return false; }
        }
        return true;
    }
}

// Iterator trait 구현
impl Iterator for PrimeIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // 찾을 때까지 시도
            self.n += 1;
            if std::u8::MAX == self.n { // u8의 최대값을 넘는 구간은 더이상 찾지 않음
                return None;
            }
            if self.is_prime() { return Some(self.n); }
        }
    }
}

fn main() {
    let prime_iter = PrimeIterator::new();

    for p in prime_iter {
        print!("{} ", p);
    }
}

