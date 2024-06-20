#[derive(Copy, Clone,PartialEq,Eq)]
pub struct Collatz {
    pub v: u64,
}

impl Iterator for Collatz {
    type Item = Collatz;

    fn next(&mut self) -> Option<Self::Item> {
        if self.v == 1 {
            return None; // Stop the iteration when we reach 1
        }
        let current_value = self.clone();
        let next_value = if self.v % 2 == 0 {
            self.v / 2
        } else {
            self.v * 3 + 1
        };

        self.v = next_value;
        Some(current_value)
    }
}

impl Collatz {
    pub fn new(n: u64) -> Self {
        Collatz { v: n }
    }
}

pub fn collatz(n: u64) -> usize {
    if n==0{
        return 0
    }
    Collatz::new(n).count()
}


