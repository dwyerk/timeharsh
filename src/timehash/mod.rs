use std::iter;

/// Look up the ordinal value for a base32 character
fn get_ordinal(&c: &char) -> usize {
    match c {
        '0' => 0,
        '1' => 1,
        'a' => 2,
        'b' => 3,
        'c' => 4,
        'd' => 5,
        'e' => 6,
        'f' => 7,
        _ => panic!("Invalid character")
    }
}

/// Look up the base32 character corresponding to an ordinal value
fn get_char(i: usize) -> char {
    match i {
        0 => '0',
        1 => '1',
        2 => 'a',
        3 => 'b',
        4 => 'c',
        5 => 'd',
        6 => 'e',
        7 => 'f',
        _ => panic!("Invalid index")
    }
}

/// Look up the neighboring base32 characters for a given character
fn get_neighbor_chars(&c: &char) -> (char, char) {
    match c {
        '0' => ('f', '1'),
        '1' => ('0', 'a'),
        'a' => ('1', 'b'),
        'b' => ('a', 'c'),
        'c' => ('b', 'd'),
        'd' => ('c', 'e'),
        'e' => ('d', 'f'),
        'f' => ('e', '0'),
        _ => panic!("Invalid character")
    }
}

struct Interval {
    begin: f64,
    end: f64
}

/// Encode time in seconds to a timehash string with a given precision.
/// 
/// # Examples
/// 
/// ```
/// use timeharsh;
/// assert_eq!(timeharsh::timehash::encode(1236532473.6328125, 6), "abcdef");
/// ```
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
            timehash.push(get_char(ch));
            bit = 0;
            ch = 0;
        }
    }

    return timehash.into_iter().collect();
}

/// Decode timehash, returning a single floating point value for epoch seconds.
/// 
/// # Examples
/// 
/// ```
/// use timeharsh;
/// assert_eq!(timeharsh::timehash::decode("abcdef"), 1236532473.6328125);
/// ```
pub fn decode(timehash: &str) -> f64 {
    return decode_exactly(timehash).0;
}

/// Decode timehash, returning a tuple of floating point value for epoch seconds and the plus/minus error for epoch seconds.
/// Returns a tuple of (epoch_seconds, error_margin).
/// 
/// # Examples
/// 
/// ```
/// use timeharsh;
/// assert_eq!(timeharsh::timehash::decode_exactly("abcdef"), (1236532473.6328125, 7704.4921875));
/// ```
pub fn decode_exactly(timehash: &str) -> (f64, f64) {
    let mut interval: Interval = Interval { begin: 0.0, end: 4039372800.0};
    let mut time_error: f64 = (interval.begin + interval.end) / 2.0;

    for c in timehash.chars() {
        let char_idx = get_ordinal(&c);
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

/// Return the timehash for the preceding time-window.
/// 
/// # Examples
/// ```
/// use timeharsh;
/// assert_eq!(timeharsh::timehash::before("abcdef"), "abcdee");
/// ```
pub fn before(timehash: &str) -> String {
    let mut i = 1;
    for c in timehash.chars().rev() {
        let padding: String = iter::repeat("f").take(i - 1).collect();
        let pos = timehash.len() - i;
        if c != '0' {
            return timehash[0..pos].to_string() + &get_neighbor_chars(&c).0.to_string() + &padding;
        } else {
            i += 1;
        }
    }
    return "".to_string();
}

/// Return the timehash for the succeeding time-window.
/// 
/// # Examples
/// ```
/// use timeharsh;
/// assert_eq!(timeharsh::timehash::after("abcdef"), "abcdf0");
/// ```
pub fn after(timehash: &str) -> String {
    let mut i = 1;
    for c in timehash.chars().rev() {
        let padding: String = iter::repeat("0").take(i - 1).collect();
        let pos = timehash.len() - i;
        if c != 'f' {
            return timehash[0..pos].to_string() + &get_neighbor_chars(&c).1.to_string() + &padding;
        } else {
            i += 1;
        }
    }
    return "".to_string();
}

/// Return the timehashes for the preceding and succeeding time-windows,
/// excluding the timehash for the current time-window.
/// 
/// # Examples
/// ```
/// use timeharsh;
/// assert_eq!(timeharsh::timehash::neighbors("abcdef"), ("abcdee".to_string(), "abcdf0".to_string()));
/// ```
pub fn neighbors(timehash: &str) -> (String, String) {
    return (before(timehash), after(timehash));
}

/// Return the timehashes for the preceding and succeeding time-windows,
/// including the timehash for the current time-window.
/// 
/// # Examples
/// ```
/// use timeharsh;
/// assert_eq!(timeharsh::timehash::expand("abcdef"), ("abcdee".to_string(), "abcdef".to_string(), "abcdf0".to_string()));
/// ```
pub fn expand(timehash: &str) -> (String, String, String) {
    return (before(timehash), timehash.to_string(), after(timehash));
}
