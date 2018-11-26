extern crate csv;
extern crate skiplist;

use self::csv::StringRecord;
use self::skiplist::OrderedSkipList;
use order::{Order, OrderType, Side};
use std::cmp::min;
use std::cmp::Ordering;

pub struct Matcher {
    verbose: bool,
    // left them public for easier testing
    pub buy_q: OrderedSkipList<Order>,
    pub sell_q: OrderedSkipList<Order>,
}

#[inline]
fn is_compatible(order1: &Order, order2: &Order) -> bool {
    order1.user_id != order2.user_id
}

#[inline]
fn restore_heap(restore_into: &mut OrderedSkipList<Order>, restore_from: Vec<Order>) {
    for order in restore_from {
        restore_into.insert(order);
    }
}

#[inline]
fn process_orders(order1: &mut Order, order2: &mut Order) {
    let min_quantity = min(order1.quantity, order2.quantity);
    order1.quantity -= min_quantity;
    order2.quantity -= min_quantity;
}

impl Matcher {
    pub fn new(verbose: bool) -> Self {
        Matcher {
            verbose: verbose,
            buy_q: unsafe {
                OrderedSkipList::with_comp(|order1: &Order, order2: &Order| {
                    if order1.price_limit > order2.price_limit {
                        Ordering::Less
                    } else if order1.price_limit < order2.price_limit {
                        Ordering::Greater
                    } else {
                        Ordering::Equal
                    }
                })
            },
            sell_q: OrderedSkipList::new(),
        }
    }

    pub fn new_order_deserialize(&mut self, record: StringRecord) -> Order {
        // ugly, but I hadn't enough time to make it look better

        let (side, price_limit, quantity, user_id, order_type);

        if record.get(0).unwrap().to_lowercase() == "buy" {
            side = Side::Buy;
        } else {
            side = Side::Sell;
        }

        price_limit = record.get(1).unwrap().parse::<u64>().unwrap();
        quantity = record.get(2).unwrap().parse::<u64>().unwrap();
        user_id = record.get(3).unwrap().parse::<u64>().unwrap();

        let tmp = record.get(4).unwrap().to_lowercase();

        if tmp == "limit" {
            order_type = OrderType::Limit;
        } else if tmp == "fillorkill" {
            order_type = OrderType::FillOrKill;
        } else {
            order_type = OrderType::ImmediateOrCancel;
        }

        self.new_order(Order {
            side,
            price_limit,
            quantity,
            user_id,
            order_type,
        })
    }

    pub fn new_order(&mut self, order: Order) -> Order {
        match order.order_type {
            OrderType::Limit => self.process_limit(order),
            OrderType::FillOrKill => self.process_fill_or_kill(order),
            OrderType::ImmediateOrCancel => self.process_immediate_or_cancel(order),
        }
    }

    fn process_limit(&mut self, mut order: Order) -> Order {
        let mut restore_from = Vec::new();
        let mut is_order_done = false;

        let order_copy = order.clone();
        let suffix;

        match order.side {
            Side::Buy => {
                suffix = "bought";

                while !self.sell_q.is_empty() {
                    let mut q_order = self.sell_q.pop_front().unwrap();

                    restore_from.push(q_order.clone());

                    // if selling is more equal or less expensive than buy, we accept that order
                    if q_order.price_limit <= order.price_limit {
                        if !is_compatible(&q_order, &order) {
                            continue;
                        }
                        process_orders(&mut q_order, &mut order);
                    } else {
                        break;
                    }

                    if order.order_done() {
                        // if current order is totally satisfied -- end the loop
                        if !q_order.order_done() {
                            // push the last unfinished order
                            self.sell_q.insert(q_order.clone());
                        }
                        is_order_done = true;
                        break;
                    }
                }

                if !is_order_done {
                    restore_heap(&mut self.sell_q, restore_from);
                    // put the rest of user's order into queue
                    self.buy_q.insert(order);
                }
            }

            Side::Sell => {
                suffix = "sold";

                while !self.buy_q.is_empty() {
                    let mut q_order = self.buy_q.pop_front().unwrap();

                    restore_from.push(q_order.clone());

                    // if buying is equal or more expensive than selling, we accept that order
                    if q_order.price_limit >= order.price_limit {
                        if !is_compatible(&q_order, &order) {
                            continue;
                        }
                        process_orders(&mut q_order, &mut order);
                    } else {
                        break;
                    }

                    if order.order_done() {
                        // if current order is totally satisfied -- end the loop
                        if !q_order.order_done() {
                            // push the last unfinished order
                            self.buy_q.insert(q_order.clone());
                        }
                        is_order_done = true;
                        break;
                    }
                }
                if !is_order_done {
                    restore_heap(&mut self.buy_q, restore_from);
                    // put the rest of user's order into queue
                    self.sell_q.insert(order);
                }
            }
        }

        if self.verbose {
            if is_order_done {
                println!("{:?} done", order_copy);
            } else {
                println!(
                    "{:?} put into queue, {} {}",
                    order,
                    order_copy.quantity - order.quantity,
                    suffix
                );
            }
        }

        order
    }

    fn process_fill_or_kill(&mut self, mut order: Order) -> Order {
        let mut restore_from = Vec::new();
        let mut is_order_done = false;

        let order_copy = order.clone();
        let suffix;

        match order.side {
            Side::Buy => {
                suffix = "bought";

                while !self.sell_q.is_empty() {
                    let mut q_order = self.sell_q.pop_front().unwrap();

                    restore_from.push(q_order.clone());

                    // if selling is more equal or less expensive than buy, we accept that order
                    if q_order.price_limit <= order.price_limit {
                        if !is_compatible(&q_order, &order) {
                            continue;
                        }
                        process_orders(&mut q_order, &mut order);
                    } else {
                        break;
                    }

                    if order.order_done() {
                        // if current order is totally satisfied -- end the loop
                        if !q_order.order_done() {
                            // push the last unfinished order
                            self.sell_q.insert(q_order.clone());
                        }
                        is_order_done = true;
                        break;
                    }
                }

                if !is_order_done {
                    restore_heap(&mut self.sell_q, restore_from);
                }
            }

            Side::Sell => {
                suffix = "sold";

                while !self.buy_q.is_empty() {
                    let mut q_order = self.buy_q.pop_front().unwrap();

                    restore_from.push(q_order.clone());

                    // if buying is equal or more expensive than selling, we accept that order
                    if q_order.price_limit >= order.price_limit {
                        if !is_compatible(&q_order, &order) {
                            continue;
                        }
                        process_orders(&mut q_order, &mut order);
                    } else {
                        break;
                    }

                    if order.order_done() {
                        // if current order is totally satisfied -- end the loop
                        if !q_order.order_done() {
                            // push the last unfinished order
                            self.buy_q.insert(q_order.clone());
                        }
                        is_order_done = true;
                        break;
                    }
                }
                if !is_order_done {
                    restore_heap(&mut self.buy_q, restore_from);
                }
            }
        }

        if self.verbose {
            if is_order_done {
                println!("{:?} done, {} {}", order_copy, order_copy.quantity, suffix);
            } else {
                println!("{:?} cancelled", order_copy);
            }
        }

        order
    }

    fn process_immediate_or_cancel(&mut self, mut order: Order) -> Order {
        let mut restore_from = Vec::new();
        let mut is_order_done = false;

        let order_copy = order.clone();
        let suffix;

        match order.side {
            Side::Buy => {
                suffix = "bought";

                while !self.sell_q.is_empty() && !order.order_done() {
                    let mut q_order = self.sell_q.pop_front().unwrap();

                    // if selling is more equal or less expensive than buy, we accept that order
                    if q_order.price_limit <= order.price_limit {
                        if !is_compatible(&q_order, &order) {
                            continue;
                        }
                        process_orders(&mut q_order, &mut order);
                        // any part of order is enough to treat it as done
                        is_order_done = true;
                    } else {
                        if !q_order.order_done() {
                            restore_from.push(q_order.clone());
                        }
                        break;
                    }
                }

                restore_heap(&mut self.sell_q, restore_from);
            }

            Side::Sell => {
                suffix = "sold";

                while !self.buy_q.is_empty() && !order.order_done() {
                    let mut q_order = self.buy_q.pop_front().unwrap();

                    // if buying is equal or more expensive than selling, we accept that order
                    if q_order.price_limit >= order.price_limit {
                        if !is_compatible(&q_order, &order) {
                            continue;
                        }
                        process_orders(&mut q_order, &mut order);
                        // any part of order is enough to treat it as done
                        is_order_done = true;
                    } else {
                        if !q_order.order_done() {
                            restore_from.push(q_order.clone());
                        }
                        break;
                    }
                }

                restore_heap(&mut self.buy_q, restore_from);
            }
        }

        if self.verbose {
            if is_order_done {
                println!(
                    "{:?}, {} {}, {} ignored",
                    order_copy,
                    order_copy.quantity - order.quantity,
                    suffix,
                    order.quantity
                );
            } else {
                println!("{:?} cancelled", order_copy);
            }
        }

        order
    }
}
