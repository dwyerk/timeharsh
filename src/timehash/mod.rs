use std::iter;

/// Look up the ordinal value for a base32 character
fn get_ordinal(&c: &char) -> Result<usize, String> {
    match c {
        '0' => Ok(0),
        '1' => Ok(1),
        'a' => Ok(2),
        'b' => Ok(3),
        'c' => Ok(4),
        'd' => Ok(5),
        'e' => Ok(6),
        'f' => Ok(7),
        _ => Err(format!("Invalid character: {c}"))
    }
}

/// Look up the base32 character corresponding to an ordinal value
fn get_char(i: usize) -> Result<char, String> {
    match i {
        0 => Ok('0'),
        1 => Ok('1'),
        2 => Ok('a'),
        3 => Ok('b'),
        4 => Ok('c'),
        5 => Ok('d'),
        6 => Ok('e'),
        7 => Ok('f'),
        _ => Err(format!("Invalid ordinal value: {i}"))
    }
}

/// Look up the neighboring base32 characters for a given character
fn get_neighbor_chars(&c: &char) -> Result<(char, char), String> {
    match c {
        '0' => Ok(('f', '1')),
        '1' => Ok(('0', 'a')),
        'a' => Ok(('1', 'b')),
        'b' => Ok(('a', 'c')),
        'c' => Ok(('b', 'd')),
        'd' => Ok(('c', 'e')),
        'e' => Ok(('d', 'f')),
        'f' => Ok(('e', '0')),
        _ => Err(format!("Invalid character: {c}"))
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
/// use timeharsh::timehash;
/// assert_eq!(timehash::encode(1236532473.6328125, 6).unwrap(), "abcdef");
/// ```
pub fn encode(timeseconds: f64, precision: usize) -> Result<String, String> {
    let mut timehash: Vec<char> = Vec::new();

    let mut interval: Interval = Interval { begin: 0.0, end: 4039372800.0};
    let bits: [usize; 3] = [4, 2, 1];
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
            timehash.push(get_char(ch)?);
            bit = 0;
            ch = 0;
        }
    }

    return Ok(timehash.into_iter().collect());
}

/// Decode timehash, returning a single floating point value for epoch seconds.
/// 
/// # Examples
/// 
/// ```
/// use timeharsh;
/// assert_eq!(timeharsh::timehash::decode("abcdef").unwrap(), 1236532473.6328125);
/// ```
pub fn decode(timehash: &str) -> Result<f64, String> {
    return Ok(decode_exactly(timehash)?.0);
}

/// Decode timehash, returning a tuple of floating point value for epoch seconds and the plus/minus error for epoch seconds.
/// Returns a tuple of (epoch_seconds, error_margin).
/// 
/// # Examples
/// 
/// ```
/// use timeharsh::timehash;
/// assert_eq!(timehash::decode_exactly("abcdef").unwrap(), (1236532473.6328125, 7704.4921875));
/// ```
pub fn decode_exactly(timehash: &str) -> Result<(f64, f64), String> {
    let mut interval: Interval = Interval { begin: 0.0, end: 4039372800.0};
    let mut time_error: f64 = (interval.begin + interval.end) / 2.0;
    let masks: [usize; 3] = [4, 2, 1];
 
    for c in timehash.chars() {
        let char_idx = get_ordinal(&c)?;
        for mask in masks {
            time_error /= 2.0;

            let mid = (interval.begin + interval.end) / 2.0;
            if (char_idx & mask) != 0 {
                interval.begin = mid;
            } else {
                interval.end = mid;
            }
        }
    }
    return Ok(((interval.begin + interval.end) / 2.0, time_error));
}

/// Return the timehash for the preceding time-window.
/// 
/// # Examples
/// ```
/// use timeharsh::timehash;
/// assert_eq!(timehash::before("abcdef").unwrap(), "abcdee");
/// ```
pub fn before(timehash: &str) -> Result<String, String> {
    let mut i = 1;
    for c in timehash.chars().rev() {
        let padding: String = iter::repeat("f").take(i - 1).collect();
        let pos = timehash.len() - i;
        if c != '0' {
            return Ok(timehash[0..pos].to_string() + &get_neighbor_chars(&c)?.0.to_string() + &padding);
        } else {
            i += 1;
        }
    }
    return Ok("".to_string());
}

/// Return the timehash for the succeeding time-window.
/// 
/// # Examples
/// ```
/// use timeharsh::timehash;
/// assert_eq!(timehash::after("abcdef").unwrap(), "abcdf0");
/// ```
pub fn after(timehash: &str) -> Result<String, String> {
    let mut i = 1;
    for c in timehash.chars().rev() {
        let padding: String = iter::repeat("0").take(i - 1).collect();
        let pos = timehash.len() - i;
        if c != 'f' {
            return Ok(timehash[0..pos].to_string() + &get_neighbor_chars(&c)?.1.to_string() + &padding);
        } else {
            i += 1;
        }
    }
    return Ok("".to_string());
}

/// Return the timehashes for the preceding and succeeding time-windows,
/// excluding the timehash for the current time-window.
/// 
/// # Examples
/// ```
/// use timeharsh::timehash;
/// assert_eq!(timehash::neighbors("abcdef").unwrap(), ("abcdee".to_string(), "abcdf0".to_string()));
/// ```
pub fn neighbors(timehash: &str) -> Result<(String, String), String> {
    return Ok((before(timehash)?, after(timehash)?));
}

/// Return the timehashes for the preceding and succeeding time-windows,
/// including the timehash for the current time-window.
/// 
/// # Examples
/// ```
/// use timeharsh::timehash;
/// assert_eq!(timehash::expand("abcdef").unwrap(), ("abcdee".to_string(), "abcdef".to_string(), "abcdf0".to_string()));
/// ```
pub fn expand(timehash: &str) -> Result<(String, String, String), String> {
    return Ok((before(timehash)?, timehash.to_string(), after(timehash)?));
}
