#[derive(Debug)]
pub struct Numbers<'a> {
    numbers: &'a [u32],
}

impl<'a> Numbers<'a> {
    pub fn new(numbers: &'a [u32]) -> Self {
        Numbers { numbers }
    }
    pub fn list(&self) -> &[u32] {
        self.numbers
    }
    pub fn latest(&self) -> Option<u32> {
        self.numbers.last().copied()
    }
    pub fn highest(&self) -> Option<u32> {
        self.numbers.iter().max().copied()
    }
    pub fn highest_three(&self) -> Vec<u32> {
        let mut top_three = self.numbers.to_vec();
        top_three.sort_unstable_by(|a, b| b.cmp(a)); 
        top_three.truncate(3);
        top_three
    }
}


