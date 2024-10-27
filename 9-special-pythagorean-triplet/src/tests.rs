#[cfg(test)]

mod test {

    use special_pythagorean_triplet::Triplet;
    #[test]
    fn can_make_struct() {
        let t1 = Triplet { a: 3, b: 4, c: 5 };
        let t2 = Triplet::new(3, 4, 5);

        assert_eq!(t1, t2);
    }

    #[test]
    fn is_ordered_correctly_true_when_actually_correct() {
        let t1 = Triplet::new(3, 4, 5);
        assert!(t1.is_ordered_correctly());
    }

    #[test]
    fn is_ordered_correctly_false_when_not_correct() {
        let t1 = Triplet::new(4, 3, 5);
        let t2 = Triplet::new(4, 5, 3);
        let t3 = Triplet::new(3, 5, 4);
        let t4 = Triplet::new(5, 4, 3);
        let t5 = Triplet::new(5, 3, 4);

        assert!(!t1.is_ordered_correctly());
        assert!(!t2.is_ordered_correctly());
        assert!(!t3.is_ordered_correctly());
        assert!(!t4.is_ordered_correctly());
        assert!(!t5.is_ordered_correctly());
    }

    #[test]
    fn is_pythagorean_triple_true_when_actually_triple() {
        let t = Triplet::new(3, 4, 5);
        assert!(t.is_pythagorean_triple());
    }

    #[test]
    fn is_pythagorean_triple_false_when_not_triple() {
        let t = Triplet::new(3, 4, 6);
        assert!(!t.is_pythagorean_triple());
    }

    #[test]
    fn test_is_sum_abc_equal_to_1000() {
        let t = Triplet::new(1, 2, 997);
        assert!(t.is_sum_abc_equal_to_1000());

        let bad_t = Triplet::new(3, 4, 5);
        assert!(!bad_t.is_sum_abc_equal_to_1000());
    }

    #[test]
    fn can_make_struct_from_just_ab() {
        let t1 = Triplet {
            a: 100,
            b: 200,
            c: 700,
        };
        let t2 = Triplet::from_ab(100, 200);

        assert_eq!(t1, t2);
    }
}
