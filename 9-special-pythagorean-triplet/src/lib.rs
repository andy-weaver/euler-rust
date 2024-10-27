fn x2(x: usize) -> usize {
    x * x
}

#[derive(Debug, PartialEq)]
pub struct Triplet {
    pub a: usize,
    pub b: usize,
    pub c: usize,
}

impl Triplet {
    pub fn new(a: usize, b: usize, c: usize) -> Self {
        Triplet { a, b, c }
    }

    pub fn is_ordered_correctly(&self) -> bool {
        (self.a < self.b) && (self.b < self.c)
    }

    pub fn is_pythagorean_triple(&self) -> bool {
        x2(self.a) + x2(self.b) == x2(self.c)
    }

    pub fn is_sum_abc_equal_to_1000(&self) -> bool {
        self.a + self.b + self.c == 1000
    }

    pub fn from_ab(a: usize, b: usize) -> Self {
        let c = 1000 - a - b;
        Triplet { a, b, c }
    }
}
