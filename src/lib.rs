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
}

