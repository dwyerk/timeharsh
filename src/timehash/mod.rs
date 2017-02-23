use std::collections::HashMap;

static base32: &'static str = "01abcdef";
static before: &'static str = "f01abcde";
static after: &'static str = "1abcdef0";

// I think maybe this should be called once and then we just hard-code
// the resulting maps since they don't need to be computed on the fly every time
fn create_decode_map() -> HashMap<char, usize> {
    let mut _decode_map: HashMap<char, usize> = HashMap::new();
    let mut neighbor_map = HashMap::new();

    for i in 0..base32.len() {
        let b32_char = base32.chars().nth(i).unwrap();
        println!("{} is {:?}", i, b32_char);
        _decode_map.insert(b32_char, i);
        neighbor_map.insert(b32_char, (before.chars().nth(i).unwrap(), after.chars().nth(i).unwrap()));
    }
    println!("map1: {:?}", _decode_map);
    println!("map2: {:?}", neighbor_map);
    return _decode_map;
}

#[cfg(test)]
mod tests {
    #[test]
    fn timehash_decode_map() {
        use super::*;
        create_decode_map();
    }
}

pub fn encode(timeseconds: i32, precision: i32) -> String {
    // Fill me in!
    "abcdef1234".to_string()
}

pub fn decode(timehash: &str) -> i32 {
    let epoch_begin: f64 = 0.0;
    let epoch_end: f64 = 4039372800.0;
    let time_error: f64 = (epoch_begin + epoch_end) / 2.0;

    for c in timehash.chars() {
        
    }
    1
}
