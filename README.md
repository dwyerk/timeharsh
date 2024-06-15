# timeharsh [![Build Status](https://travis-ci.org/dwyerk/timeharsh.svg?branch=master)](https://travis-ci.org/dwyerk/timeharsh)
Timehash in Rust

This project is a Rust implementation of the timehash algorithm. The original
algorithm was written in Python by Abe Usher and is available at https://github.com/abeusher/timehash

> timehash is an algorithm (with multiple reference implementations) for
> calculating variable precision sliding windows of time. When performing
> aggregations and correlations on large-scale data sets, the ability to
> convert precise time values into 'malleable intervals' allows for many
> novel analytics.

An example use case of timehash is described in the following paper: https://isprs-annals.copernicus.org/articles/IV-4-W2/31/2017/isprs-annals-IV-4-W2-31-2017.pdf

## Usage
```rust
extern crate timeharsh;

let hash1 = timehash::encode(1236532473.6328125, 6).unwrap();
// hash1 == "abcdef"

let t1 = timeharsh::timehash::decode("abcdef").unwrap();
// t1 == 1236532473.6328125
```

## Building the project
`cargo build`

## Running the tests
`cargo test`
