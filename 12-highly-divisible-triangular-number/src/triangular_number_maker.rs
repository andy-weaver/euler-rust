#[derive(Debug, PartialEq)]
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

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn can_make_triangular_number_maker() {
        let m1 = TriangularNumberMaker {
            numbers: vec![1],
            current_index: 1,
        };
        let m2 = TriangularNumberMaker::new();

        assert_eq!(m1, m2);
    }

    #[test]
    fn can_make_default() {
        let m1 = TriangularNumberMaker::new();
        let m2 = TriangularNumberMaker::default();

        assert_eq!(m1, m2);
    }

    #[test]
    fn can_get_next_triangular_number() {
        let mut m = TriangularNumberMaker::new();
        m.next();

        assert_eq!(m.numbers, vec![1, 3]);
        assert_eq!(m.current_index, 2);
    }
}
