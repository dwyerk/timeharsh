pub mod timehash;

#[cfg(test)]
mod tests {
    #[test]
    fn timehash_encode() {
        use timehash;
        assert_eq!(timehash::encode(1487708113.0, 10), "afcccc0e1b");
    }

    #[test]
    fn timehash_encode_precision_length() {
        use timehash;
        assert_eq!(timehash::encode(1487708113.0, 10).len(), 10);
    }

    #[test]
    fn timehash_decode() {
        use timehash;
        assert_eq!(timehash::decode("afcccc0e1b"), 1487708111.2923145);
    }

    #[test]
    fn timehash_before() {
        use timehash;
        assert_eq!(timehash::before("afcccc0e1b"), "afcccc0e1a");
        assert_eq!(timehash::before("afcccc0e1a"), "afcccc0e11");
        assert_eq!(timehash::before("afcccc0e10"), "afcccc0e0f");
    }

    #[test]
    fn timehash_after() {
        use timehash;
        assert_eq!(timehash::after("afcccc0e0f"), "afcccc0e10");
        assert_eq!(timehash::after("afcccc0e1a"), "afcccc0e1b");
        assert_eq!(timehash::after("afcccc0e1b"), "afcccc0e1c");
    }

    #[test]
    fn timehash_neighbors() {
        use timehash;
        assert_eq!(timehash::neighbors("afcccc0e0f"), ("afcccc0e0e".to_string(), "afcccc0e10".to_string()));
    }

    #[test]
    fn timehash_expand() {
        use timehash;
        assert_eq!(timehash::expand("afcccc0e0f"), ("afcccc0e0e".to_string(), "afcccc0e0f".to_string(), "afcccc0e10".to_string()));
    }
}

