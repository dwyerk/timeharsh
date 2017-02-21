pub mod timehash;

#[cfg(test)]
mod tests {
    #[test]
    fn timehash_encode() {
        use timehash;
        assert_eq!(timehash::encode(1487708113, 10), "afcccbaefd");
    }

    #[test]
    fn timehash_decode() {
        use timehash;
        assert_eq!(timehash::decode("afcccbaefd"), 1487708113);
    }
}

