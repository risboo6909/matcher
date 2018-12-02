#[macro_use]
extern crate serde_derive;

mod matcher;
mod order;

pub use self::matcher::Matcher;
pub use self::order::{Order, OrderType, Side};


#[test]
fn test_1() {
    let mut matcher = Matcher::new(false);

    let order = matcher.new_order(Side::Sell, 11, 6, 12, OrderType::Limit);

    assert_eq!(order, Order::new(Side::Sell, 11, 6, 123, OrderType::Limit));

    let order = matcher.new_order(Side::Sell, 12, 4, 123, OrderType::Limit);

    assert_eq!(order, Order::new(Side::Sell, 12, 4, 123, OrderType::Limit));

    let order = matcher.new_order(Side::Buy, 13, 5, 12, OrderType::Limit);

    // partially done, one left and must be added to queue
    assert_eq!(order, Order::new(Side::Buy, 13, 1, 12, OrderType::Limit));

    // check if partial order is in queue
    assert_eq!(
        *matcher.buy_q.borrow_mut().front().unwrap(),
        Order::new(Side::Buy, 13, 1, 12, OrderType::Limit)
    );

    let order = matcher.new_order(Side::Buy, 11, 12, 13, OrderType::ImmediateOrCancel);

    // part of previous order must be done, other part must be disacrded
    assert_eq!(
        order,
        Order::new(Side::Buy, 11, 6, 13, OrderType::ImmediateOrCancel)
    );

    let order = matcher.new_order(Side::Sell, 12, 3, 13, OrderType::FillOrKill);

    // this whole order must be done
    assert_eq!(
        order,
        Order::new(Side::Sell, 12, 3, 13, OrderType::FillOrKill)
    );

    assert_eq!(
        *matcher.buy_q.borrow_mut().front().unwrap(),
        Order::new(Side::Buy, 13, 2, 12, OrderType::Limit)
    );

    let _order = matcher.new_order(Side::Sell, 12, 3, 13, OrderType::FillOrKill);

    // this whole order must be discarded
    assert_eq!(
        *matcher.buy_q.borrow_mut().front().unwrap(),
        Order::new(Side::Buy, 13, 2, 12, OrderType::Limit)
    );
}

#[test]
fn test_2() {
    let mut matcher = Matcher::new(false);

    for _x in 0..6080 {
        matcher.new_order(Side::Sell, 6, 6, 12, OrderType::Limit);
    }

    // add 20 orders which must match with the queried one in benchmark

    matcher.new_order(Side::Sell, 10, 10, 123, OrderType::Limit);
    matcher.new_order(Side::Sell, 10, 10, 123, OrderType::Limit);

    matcher.new_order(Side::Sell, 8, 10, 123, OrderType::Limit);
    matcher.new_order(Side::Sell, 8, 10, 123, OrderType::Limit);
    matcher.new_order(Side::Sell, 8, 10, 123, OrderType::Limit);
    matcher.new_order(Side::Sell, 8, 10, 123, OrderType::Limit);
    matcher.new_order(Side::Sell, 8, 10, 123, OrderType::Limit);
    matcher.new_order(Side::Sell, 8, 10, 123, OrderType::Limit);
    matcher.new_order(Side::Sell, 8, 10, 123, OrderType::Limit);
    matcher.new_order(Side::Sell, 8, 10, 123, OrderType::Limit);
    matcher.new_order(Side::Sell, 5, 10, 123, OrderType::Limit);
    matcher.new_order(Side::Sell, 5, 10, 123, OrderType::Limit);
    matcher.new_order(Side::Sell, 5, 10, 123, OrderType::Limit);
    matcher.new_order(Side::Sell, 5, 10, 123, OrderType::Limit);
    matcher.new_order(Side::Sell, 5, 10, 123, OrderType::Limit);
    matcher.new_order(Side::Sell, 4, 10, 123, OrderType::Limit);
    matcher.new_order(Side::Sell, 4, 10, 123, OrderType::Limit);
    matcher.new_order(Side::Sell, 4, 10, 123, OrderType::Limit);
    matcher.new_order(Side::Sell, 2, 10, 123, OrderType::Limit);
    matcher.new_order(Side::Sell, 1, 10, 123, OrderType::Limit);

    // this order must match with 20 orders above and the remaining 1 must be queued
    let order = matcher.new_order(Side::Buy, 11, 201, 12, OrderType::Limit);

    // one left
    assert_eq!(order, Order::new(Side::Buy, 11, 1, 12, OrderType::Limit));

    // nothing left to satisfy an order it must be queued
    let order = matcher.new_order(Side::Buy, 11, 201, 12, OrderType::Limit);
    assert_eq!(order, Order::new(Side::Buy, 11, 201, 12, OrderType::Limit));

    // ensure two previous orders were queued
    assert_eq!(
        matcher.buy_q.borrow_mut().pop_front().unwrap(),
        Order::new(Side::Buy, 11, 1, 12, OrderType::Limit)
    );
    assert_eq!(
        matcher.buy_q.borrow_mut().pop_front().unwrap(),
        Order::new(Side::Buy, 11, 201, 12, OrderType::Limit)
    );
}

#[test]
fn test_3() {
    let mut matcher = Matcher::new(false);
    matcher.new_order(Side::Sell, 10, 20, 23, OrderType::Limit);
    // FillOrKill must be ignored 30 items requested, only 20 available
    let order = matcher.new_order(Side::Buy, 20, 30, 24, OrderType::FillOrKill);
    assert_eq!(order.quantity, 30);
    // ImmediateOrCancel may be partially done
    let order = matcher.new_order(Side::Buy, 20, 30, 25, OrderType::ImmediateOrCancel);
    assert_eq!(order.quantity, 10);
    // no orders should left in sell queue
    assert_eq!(matcher.sell_q.borrow().len(), 0);
}
