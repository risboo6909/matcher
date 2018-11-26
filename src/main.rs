extern crate csv;
extern crate matcher;
extern crate skiplist;

use matcher::Matcher;
use std::io;

fn main() {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b' ')
        .from_reader(io::stdin());

    let mut matcher = Matcher::new(true);

    for result in rdr.records() {
        let record = result.unwrap();
        matcher.new_order_deserialize(record);
    }
}
