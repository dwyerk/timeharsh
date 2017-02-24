use std::collections::HashMap;

static BASE32: &'static str = "01abcdef";

//{'c': 4, 'a': 2, 'e': 6, '0': 0, '1': 1, 'b': 3, 'd': 5, 'f': 7}
fn get_decode_map() -> HashMap<char, usize> {
    let mut decode_map: HashMap<char, usize> = HashMap::new();
    decode_map.insert('0', 0);
    decode_map.insert('1', 1);
    decode_map.insert('a', 2);
    decode_map.insert('b', 3);
    decode_map.insert('c', 4);
    decode_map.insert('d', 5);
    decode_map.insert('e', 6);
    decode_map.insert('f', 7);
    return decode_map;
}

//{'a': ('1', 'b'), 'f': ('e', '0'), 'd': ('c', 'e'), '1': ('0', 'a'), '0': ('f', '1'), 'e': ('d', 'f'), 'c': ('b', 'd'), 'b': ('a', 'c')}
fn get_neighbor_map() -> HashMap<char, (char, char)> {
    let mut neighbor_map: HashMap<char, (char, char)> = HashMap::new();
    neighbor_map.insert('0', ('f', '1'));
    neighbor_map.insert('1', ('0', 'a'));
    neighbor_map.insert('a', ('1', 'b'));
    neighbor_map.insert('b', ('a', 'c'));
    neighbor_map.insert('c', ('b', 'd'));
    neighbor_map.insert('d', ('c', 'e'));
    neighbor_map.insert('e', ('d', 'f'));
    neighbor_map.insert('f', ('e', '0'));
    return neighbor_map;
}

fn create_decode_map() -> HashMap<char, usize> {
    let before = "f01abcde";
    let after = "1abcdef0";
    let mut _decode_map: HashMap<char, usize> = HashMap::new();
    let mut _neighbor_map = HashMap::new();

    for i in 0..BASE32.len() {
        let b32_char = BASE32.chars().nth(i).unwrap();
        println!("{} is {:?}", i, b32_char);
        _decode_map.insert(b32_char, i);
        _neighbor_map.insert(b32_char, (before.chars().nth(i).unwrap(), after.chars().nth(i).unwrap()));
    }
    println!("map1: {:?}", _decode_map);
    println!("map2: {:?}", _neighbor_map);
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

struct Interval {
    begin: f64,
    end: f64
}

pub fn encode(timeseconds: f64, precision: usize) -> String {
    let mut timehash: Vec<char> = Vec::new();

    let mut interval: Interval = Interval { begin: 0.0, end: 4039372800.0};
    let bits = vec![4, 2, 1];
    let mut bit = 0;
    let mut ch = 0;

    while timehash.len() < precision {
        let mid = (interval.begin + interval.end) / 2.0;

        if timeseconds > mid {
            ch |= bits[bit];
            interval.begin = mid;
        } else {
            interval.end = mid;
        }

        if bit < 2 {
            bit += 1;
        } else {
            timehash.push(BASE32.chars().nth(ch).unwrap());
            bit = 0;
            ch = 0;
        }
    }

    return timehash.into_iter().collect();
}

pub fn decode(timehash: &str) -> f64 {
    return decode_exactly(timehash).0;
}

pub fn decode_exactly(timehash: &str) -> (f64, f64) {
    let mut interval: Interval = Interval { begin: 0.0, end: 4039372800.0};
    let mut time_error: f64 = (interval.begin + interval.end) / 2.0;
    let decode_map = get_decode_map();

    for c in timehash.chars() {
        let char_idx = decode_map.get(&c).unwrap();
        for mask in vec![4, 2, 1] {
            time_error /= 2.0;

            let mid = (interval.begin + interval.end) / 2.0;
            if (char_idx & mask) != 0 {
                interval.begin = mid;
            } else {
                interval.end = mid;
            }
        }
    }
    return ((interval.begin + interval.end) / 2.0, time_error);
}
