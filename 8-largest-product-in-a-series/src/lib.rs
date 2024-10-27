pub const NUMBER: &str = "7316717653133062491922511967442657474235534919493496983520312774506326239578318016984801869478851843858615607891129494954595017379583319528532088055111254069874715852386305071569329096329522744304355766896648950445244523161731856403098711121722383113622298934233803081353362766142828064444866452387493035890729629049156044077239071381051585930796086670172427121883998797908792274921901699720888093776657273330010533678812202354218097512545405947522435258490771167055601360483958644670632441572215539753697817977846174064955149290862569321978468622482839722413756570560574902614079729686524145351004748216637048440319989000889524345065854122758866688116427171479924442928230863465674813919123162824586178664583591245665294765456828489128831426076900422421902267105562632111110937054421750694165896040807198403850962455444362981230987879927244284909188845801561660979191338754992005240636899125607176060588611646710940507754100225698315520005593572972571636269561882670428252483600823257530420752963450";

pub fn find_max_product(windower: &mut Windower) -> usize {
    let n_iter = windower.get_number().len() - windower.get_window_size();

    (0..n_iter).for_each(|_| {
        windower.next();
    });

    windower.get_current_max_product()
}

#[derive(Debug, PartialEq)]
pub struct Windower {
    number: String,
    window_size: usize,
    current_window: Vec<char>,
    current_max_product: usize,
    index: usize,
}

impl Windower {
    pub fn new(window_size: usize) -> Self {
        let current_window = NUMBER.chars().take(window_size).collect::<Vec<char>>();
        let current_max_product = current_window
            .clone()
            .iter()
            .map(|x| *x as usize - '0' as usize)
            .product();

        Windower {
            number: NUMBER.to_string(),
            window_size,
            current_window,
            current_max_product,
            index: 0,
        }
    }

    pub fn prod(&self) -> usize {
        self.current_window
            .iter()
            .map(|x| *x as usize - '0' as usize)
            .product()
    }

    pub fn increment_window(&mut self) {
        self.index += 1;
        self.current_window = self
            .number
            .chars()
            .skip(self.index)
            .take(self.window_size)
            .collect::<Vec<char>>();
    }

    pub fn get_current_max_product(&self) -> usize {
        self.current_max_product
    }

    pub fn get_number(&self) -> String {
        self.number.clone()
    }

    pub fn get_window_size(&self) -> usize {
        self.window_size
    }

    pub fn len(&self) -> usize {
        self.get_number().len()
    }
}

impl Iterator for Windower {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.increment_window();

        let product = self.prod();

        if product > self.current_max_product {
            self.current_max_product = product;
        }

        Some(product)
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn can_build_struct() {
        let w1 = Windower {
            number: NUMBER.to_string(),
            window_size: 4,
            current_window: vec!['7', '3', '1', '6'],
            current_max_product: 7 * 3 * 6,
            index: 0,
        };

        let w2 = Windower::new(4);

        assert_eq!(w1, w2);

        let w1 = Windower {
            number: NUMBER.to_string(),
            window_size: 2,
            current_window: vec!['7', '3'],
            current_max_product: 7 * 3,
            index: 0,
        };

        let w2 = Windower::new(2);

        assert_eq!(w1, w2);
    }

    #[test]
    fn can_get_window_product() {
        let w = Windower::new(2);
        let expected = 7 * 3;
        let actual = w.prod();

        println!("Actual: {}", actual);
        assert_eq!(actual, expected);
    }

    #[test]
    fn can_slide_the_window() {
        let mut w = Windower::new(2);
        assert_eq!(w.current_window, vec!['7', '3']);

        w.next();
        assert_eq!(w.current_window, vec!['3', '1']);

        w.next();
        assert_eq!(w.current_window, vec!['1', '6']);
    }

    #[test]
    fn iterator_correctly_updates_max_product_at_some_point() {
        let mut w = Windower::new(2);
        (0..3).for_each(|_| {
            w.next();
        });

        assert_eq!(w.get_current_max_product(), 7 * 6);
    }

    #[test]
    fn can_find_max_product_over_whole_number() {
        let mut w = Windower::new(4);
        let max_product = find_max_product(&mut w);

        assert_eq!(max_product, 5832); // this comes from the problem statement
    }

    #[test]
    fn can_get_length() {
        let w = Windower::new(4);

        let expected = NUMBER.len();
        let actual = w.len();

        assert_eq!(actual, expected);
    }
}
