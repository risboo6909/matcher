#[macro_use]
extern crate criterion;

extern crate matcher;

use criterion::Criterion;
use matcher::{Matcher, Order, OrderType, Side};


fn criterion_benchmark(c: &mut Criterion) {

    c.bench_function("matcher", move |b| b.iter_with_setup(
        || {
            let mut matcher = Matcher::new(false);

            for _x in 0..6080 {
                matcher.new_order(Order{side: Side::Sell, price_limit: 6, quantity: 6, user_id: 12, order_type: OrderType::Limit});
            }

            // add 20 orders which must match with the queried one in benchmark

            matcher.new_order(Order{side: Side::Sell, price_limit: 10, quantity: 10, user_id: 123, order_type: OrderType::Limit});
            matcher.new_order(Order{side: Side::Sell, price_limit: 10, quantity: 10, user_id: 123, order_type: OrderType::Limit});

            matcher.new_order(Order{side: Side::Sell, price_limit: 8, quantity: 10, user_id: 123, order_type: OrderType::Limit});
            matcher.new_order(Order{side: Side::Sell, price_limit: 8, quantity: 10, user_id: 123, order_type: OrderType::Limit});
            matcher.new_order(Order{side: Side::Sell, price_limit: 8, quantity: 10, user_id: 123, order_type: OrderType::Limit});
            matcher.new_order(Order{side: Side::Sell, price_limit: 8, quantity: 10, user_id: 123, order_type: OrderType::Limit});
            matcher.new_order(Order{side: Side::Sell, price_limit: 8, quantity: 10, user_id: 123, order_type: OrderType::Limit});
            matcher.new_order(Order{side: Side::Sell, price_limit: 8, quantity: 10, user_id: 123, order_type: OrderType::Limit});
            matcher.new_order(Order{side: Side::Sell, price_limit: 8, quantity: 10, user_id: 123, order_type: OrderType::Limit});
            matcher.new_order(Order{side: Side::Sell, price_limit: 8, quantity: 10, user_id: 123, order_type: OrderType::Limit});

            matcher.new_order(Order{side: Side::Sell, price_limit: 5, quantity: 10, user_id: 123, order_type: OrderType::Limit});
            matcher.new_order(Order{side: Side::Sell, price_limit: 5, quantity: 10, user_id: 123, order_type: OrderType::Limit});
            matcher.new_order(Order{side: Side::Sell, price_limit: 5, quantity: 10, user_id: 123, order_type: OrderType::Limit});
            matcher.new_order(Order{side: Side::Sell, price_limit: 5, quantity: 10, user_id: 123, order_type: OrderType::Limit});
            matcher.new_order(Order{side: Side::Sell, price_limit: 5, quantity: 10, user_id: 123, order_type: OrderType::Limit});
            matcher.new_order(Order{side: Side::Sell, price_limit: 4, quantity: 10, user_id: 123, order_type: OrderType::Limit});
            matcher.new_order(Order{side: Side::Sell, price_limit: 4, quantity: 10, user_id: 123, order_type: OrderType::Limit});
            matcher.new_order(Order{side: Side::Sell, price_limit: 4, quantity: 10, user_id: 123, order_type: OrderType::Limit});

            matcher.new_order(Order{side: Side::Sell, price_limit: 2, quantity: 10, user_id: 123, order_type: OrderType::Limit});
            matcher.new_order(Order{side: Side::Sell, price_limit: 1, quantity: 10, user_id: 123, order_type: OrderType::Limit});
            
            matcher
        }, 
        |mut matcher| {
            matcher.new_order(
                Order{side: Side::Buy, price_limit: 11, quantity: 201, user_id: 12, order_type: OrderType::Limit}
            );
        })
    );

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
