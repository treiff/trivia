extern crate curl;
extern crate serde_json;

use std::str;
use curl::easy::Easy;

fn main() {
    let mut handle = Easy::new();
    let mut json_string = String::new();

    handle.url("https://opentdb.com/api.php?amount=10").unwrap();

    {
        let mut transfer = handle.transfer();
        transfer.write_function(|data| {
            json_string.push_str(str::from_utf8(data).unwrap());
            Ok(data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }

    println!("string is: {0}", json_string);
}
