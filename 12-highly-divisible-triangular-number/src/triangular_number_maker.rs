pub struct TriangularNumberMaker {
    pub numbers: Vec<u32>,
    pub current_index: u32,
}

impl TriangularNumberMaker {
    pub fn new() -> Self {
        TriangularNumberMaker {
            numbers: vec![1],
            current_index: 1,
        }
    }
}

impl Default for TriangularNumberMaker {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for TriangularNumberMaker {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.current_index += 1;

        let res = self.numbers.last().unwrap().to_owned() + self.current_index;
        self.numbers.push(res);
        Some(res)
    }
}
