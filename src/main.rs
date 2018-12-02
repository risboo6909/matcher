use matcher;
use csv;

use self::matcher::{Matcher, Order};
use std::io;
use std::error::Error;


fn parse_file(matcher: &mut Matcher) -> Result<(), Box<dyn Error>> {

    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b' ')
        .from_reader(io::stdin());

    for result in rdr.deserialize() {
        let mut order: Order = result?;
        order.start_quantity = order.quantity;
        matcher.new_order_object(order);
    }

    Ok(())

}

fn main() {
    let mut matcher = Matcher::new(true);
    parse_file(&mut matcher).unwrap();
}
