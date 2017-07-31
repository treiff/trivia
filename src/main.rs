extern crate curl;

use std::io::{stdout, Write};
use curl::easy::Easy;

fn main() {
    let mut handle = Easy::new();

    handle.url("https://opentdb.com/api.php?amount=10").unwrap();

    handle.write_function(|data| {
        Ok(stdout().write(data).unwrap())
    }).unwrap();
    handle.perform().unwrap();
}
