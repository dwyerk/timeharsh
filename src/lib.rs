pub mod timehash;

#[cfg(test)]
mod tests {
    #[test]
    fn timehash_encode() -> Result<(), String> {
        use timehash;
        assert_eq!(timehash::encode(1487708113.0, 10).unwrap(), "afcccc0e1b");
        Ok(())
    }

    #[test]
    fn timehash_encode_precision_length() -> Result<(), String> {
        use timehash;
        assert_eq!(timehash::encode(1487708113.0, 10)?.len(), 10);
        Ok(())
    }

    #[test]
    fn timehash_decode() -> Result<(), String> {
        use timehash;
        assert_eq!(timehash::decode("afcccc0e1b")?, 1487708111.2923145);
        Ok(())
    }

    #[test]
    fn timehash_before() -> Result<(), String> {
        use timehash;
        assert_eq!(timehash::before("afcccc0e1b")?, "afcccc0e1a");
        assert_eq!(timehash::before("afcccc0e1a")?, "afcccc0e11");
        assert_eq!(timehash::before("afcccc0e10")?, "afcccc0e0f");
        Ok(())
    }

    #[test]
    fn timehash_after() -> Result<(), String> {
        use timehash;
        assert_eq!(timehash::after("afcccc0e0f")?, "afcccc0e10");
        assert_eq!(timehash::after("afcccc0e1a")?, "afcccc0e1b");
        assert_eq!(timehash::after("afcccc0e1b")?, "afcccc0e1c");
        Ok(())
    }

    #[test]
    fn timehash_neighbors() -> Result<(), String> {
        use timehash;
        assert_eq!(timehash::neighbors("afcccc0e0f")?, ("afcccc0e0e".to_string(), "afcccc0e10".to_string()));
        Ok(())
    }

    #[test]
    fn timehash_expand() -> Result<(), String> {
        use timehash;
        assert_eq!(timehash::expand("afcccc0e0f")?, ("afcccc0e0e".to_string(), "afcccc0e0f".to_string(), "afcccc0e10".to_string()));
        Ok(())
    }
}

