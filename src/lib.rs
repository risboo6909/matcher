mod matchers;
mod order;

pub use matchers::Matcher;
pub use order::{Order, OrderType, Side};

#[test]
fn test_1() {
    let mut matcher = Matcher::new(false);

    let order = matcher.new_order(Order {
        side: Side::Sell,
        price_limit: 11,
        quantity: 6,
        user_id: 12,
        order_type: OrderType::Limit,
    });

    assert_eq!(
        order,
        Order {
            side: Side::Sell,
            price_limit: 11,
            quantity: 6,
            user_id: 123,
            order_type: OrderType::Limit
        }
    );

    let order = matcher.new_order(Order {
        side: Side::Sell,
        price_limit: 12,
        quantity: 4,
        user_id: 123,
        order_type: OrderType::Limit,
    });

    assert_eq!(
        order,
        Order {
            side: Side::Sell,
            price_limit: 12,
            quantity: 4,
            user_id: 123,
            order_type: OrderType::Limit
        }
    );

    let order = matcher.new_order(Order {
        side: Side::Buy,
        price_limit: 13,
        quantity: 5,
        user_id: 12,
        order_type: OrderType::Limit,
    });

    // partially done, one left and must be added to queue
    assert_eq!(
        order,
        Order {
            side: Side::Buy,
            price_limit: 13,
            quantity: 1,
            user_id: 12,
            order_type: OrderType::Limit
        }
    );

    // check if partial order is in queue
    assert_eq!(
        *matcher.buy_q.peek_min().unwrap(),
        Order {
            side: Side::Buy,
            price_limit: 13,
            quantity: 1,
            user_id: 12,
            order_type: OrderType::Limit
        }
    );

    let order = matcher.new_order(Order {
        side: Side::Buy,
        price_limit: 11,
        quantity: 12,
        user_id: 13,
        order_type: OrderType::ImmediateOrCancel,
    });

    // part of previous order must be done, other part must be disacrded
    assert_eq!(
        order,
        Order {
            side: Side::Buy,
            price_limit: 11,
            quantity: 6,
            user_id: 13,
            order_type: OrderType::ImmediateOrCancel
        }
    );

    let order = matcher.new_order(Order {
        side: Side::Sell,
        price_limit: 12,
        quantity: 3,
        user_id: 13,
        order_type: OrderType::FillOrKill,
    });

    // this whole order must be done
    assert_eq!(
        order,
        Order {
            side: Side::Sell,
            price_limit: 12,
            quantity: 3,
            user_id: 13,
            order_type: OrderType::FillOrKill
        }
    );
    assert_eq!(
        *matcher.buy_q.peek_min().unwrap(),
        Order {
            side: Side::Buy,
            price_limit: 13,
            quantity: 2,
            user_id: 12,
            order_type: OrderType::Limit
        }
    );

    let order = matcher.new_order(Order {
        side: Side::Sell,
        price_limit: 12,
        quantity: 3,
        user_id: 13,
        order_type: OrderType::FillOrKill,
    });

    // this whole order must be discarded
    assert_eq!(
        *matcher.buy_q.peek_min().unwrap(),
        Order {
            side: Side::Buy,
            price_limit: 13,
            quantity: 2,
            user_id: 12,
            order_type: OrderType::Limit
        }
    );
}

#[test]
fn test_2() {
    let mut matcher = Matcher::new(false);

    for _x in 0..6080 {
        matcher.new_order(Order {
            side: Side::Sell,
            price_limit: 6,
            quantity: 6,
            user_id: 12,
            order_type: OrderType::Limit,
        });
    }

    // add 20 orders which must match with the queried one in benchmark

    matcher.new_order(Order {
        side: Side::Sell,
        price_limit: 10,
        quantity: 10,
        user_id: 123,
        order_type: OrderType::Limit,
    });
    matcher.new_order(Order {
        side: Side::Sell,
        price_limit: 10,
        quantity: 10,
        user_id: 123,
        order_type: OrderType::Limit,
    });

    matcher.new_order(Order {
        side: Side::Sell,
        price_limit: 8,
        quantity: 10,
        user_id: 123,
        order_type: OrderType::Limit,
    });
    matcher.new_order(Order {
        side: Side::Sell,
        price_limit: 8,
        quantity: 10,
        user_id: 123,
        order_type: OrderType::Limit,
    });
    matcher.new_order(Order {
        side: Side::Sell,
        price_limit: 8,
        quantity: 10,
        user_id: 123,
        order_type: OrderType::Limit,
    });
    matcher.new_order(Order {
        side: Side::Sell,
        price_limit: 8,
        quantity: 10,
        user_id: 123,
        order_type: OrderType::Limit,
    });
    matcher.new_order(Order {
        side: Side::Sell,
        price_limit: 8,
        quantity: 10,
        user_id: 123,
        order_type: OrderType::Limit,
    });
    matcher.new_order(Order {
        side: Side::Sell,
        price_limit: 8,
        quantity: 10,
        user_id: 123,
        order_type: OrderType::Limit,
    });
    matcher.new_order(Order {
        side: Side::Sell,
        price_limit: 8,
        quantity: 10,
        user_id: 123,
        order_type: OrderType::Limit,
    });
    matcher.new_order(Order {
        side: Side::Sell,
        price_limit: 8,
        quantity: 10,
        user_id: 123,
        order_type: OrderType::Limit,
    });

    matcher.new_order(Order {
        side: Side::Sell,
        price_limit: 5,
        quantity: 10,
        user_id: 123,
        order_type: OrderType::Limit,
    });
    matcher.new_order(Order {
        side: Side::Sell,
        price_limit: 5,
        quantity: 10,
        user_id: 123,
        order_type: OrderType::Limit,
    });
    matcher.new_order(Order {
        side: Side::Sell,
        price_limit: 5,
        quantity: 10,
        user_id: 123,
        order_type: OrderType::Limit,
    });
    matcher.new_order(Order {
        side: Side::Sell,
        price_limit: 5,
        quantity: 10,
        user_id: 123,
        order_type: OrderType::Limit,
    });
    matcher.new_order(Order {
        side: Side::Sell,
        price_limit: 5,
        quantity: 10,
        user_id: 123,
        order_type: OrderType::Limit,
    });
    matcher.new_order(Order {
        side: Side::Sell,
        price_limit: 4,
        quantity: 10,
        user_id: 123,
        order_type: OrderType::Limit,
    });
    matcher.new_order(Order {
        side: Side::Sell,
        price_limit: 4,
        quantity: 10,
        user_id: 123,
        order_type: OrderType::Limit,
    });
    matcher.new_order(Order {
        side: Side::Sell,
        price_limit: 4,
        quantity: 10,
        user_id: 123,
        order_type: OrderType::Limit,
    });

    matcher.new_order(Order {
        side: Side::Sell,
        price_limit: 2,
        quantity: 10,
        user_id: 123,
        order_type: OrderType::Limit,
    });
    matcher.new_order(Order {
        side: Side::Sell,
        price_limit: 1,
        quantity: 10,
        user_id: 123,
        order_type: OrderType::Limit,
    });

    // this order must match with 20 orders above and the remaining 1 must be queued
    let order = matcher.new_order(Order {
        side: Side::Buy,
        price_limit: 11,
        quantity: 201,
        user_id: 12,
        order_type: OrderType::Limit,
    });

    // one left
    assert_eq!(
        order,
        Order {
            side: Side::Buy,
            price_limit: 11,
            quantity: 1,
            user_id: 12,
            order_type: OrderType::Limit
        }
    );

    // nothing left to satisfy an order it must be queued
    let order = matcher.new_order(Order {
        side: Side::Buy,
        price_limit: 11,
        quantity: 201,
        user_id: 12,
        order_type: OrderType::Limit,
    });
    assert_eq!(
        order,
        Order {
            side: Side::Buy,
            price_limit: 11,
            quantity: 201,
            user_id: 12,
            order_type: OrderType::Limit
        }
    );

    // ensure two previous orders were queued
    assert_eq!(
        matcher.buy_q.pop_min().unwrap(),
        Order {
            side: Side::Buy,
            price_limit: 11,
            quantity: 1,
            user_id: 12,
            order_type: OrderType::Limit
        }
    );
    assert_eq!(
        matcher.buy_q.pop_min().unwrap(),
        Order {
            side: Side::Buy,
            price_limit: 11,
            quantity: 201,
            user_id: 12,
            order_type: OrderType::Limit
        }
    );
}
