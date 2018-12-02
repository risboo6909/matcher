use skiplist;

use self::skiplist::OrderedSkipList;
use crate::order::{Order, OrderType, Side};
use std::cell::RefCell;
use std::cmp::{min, Ordering};
use std::rc::Rc;

pub struct Matcher {
    verbose: bool,
    // left them public for easier testing
    pub buy_q: Rc<RefCell<OrderedSkipList<Order>>>,
    pub sell_q: Rc<RefCell<OrderedSkipList<Order>>>,
}

#[inline]
fn is_compatible(order1: &Order, order2: &Order) -> bool {
    order1.user_id != order2.user_id
}

#[inline]
fn process_orders(mut cur_order: Order, mut q_order: Order) -> (Order, Order) {
    let min_quantity = min(cur_order.quantity, q_order.quantity);
    cur_order.quantity -= min_quantity;
    q_order.quantity -= min_quantity;
    (cur_order, q_order)
}

#[inline]
fn is_price_satisfied(q_order: &Order, order: &Order) -> bool {
    match order.side {
        Side::Sell => q_order.price_limit >= order.price_limit,
        Side::Buy => q_order.price_limit <= order.price_limit,
    }
}

#[inline]
fn report(order: &Order) {
    match order.order_type {
        OrderType::Limit => {
            if !order.order_done() {
                println!(", {} enqueued", order.quantity);
            } else {
                println!();
            }      
        },
        OrderType::FillOrKill => {
            if !order.order_done() {
                println!(", order cancelled");
            } else {
                println!();
            }      
        },
        OrderType::ImmediateOrCancel => {
            if !order.order_done() {
                println!(", {} ignored", order.quantity);
            } else {
                println!();
            }
        }
    }
}


impl Matcher {

    pub fn new(verbose: bool) -> Self {
        Matcher {
            verbose,
            // save from most expensive to least expensive
            buy_q: Rc::new(RefCell::new(unsafe {
                OrderedSkipList::with_comp(|order1: &Order, order2: &Order| {
                    if order1.price_limit > order2.price_limit {
                        Ordering::Less
                    } else if order1.price_limit < order2.price_limit {
                        Ordering::Greater
                    } else {
                        Ordering::Equal
                    }
                })
            })),
            // save from least expensive to most expensive
            sell_q: Rc::new(RefCell::new(OrderedSkipList::new())),
        }
    }

    pub fn new_order(
        &mut self,
        side: Side,
        price_limit: u64,
        quantity: u64,
        user_id: u64,
        order_type: OrderType,
    ) -> Order {
        self.new_order_object(Order::new(side, price_limit, quantity, user_id, order_type))
    }

    pub fn new_order_object(
        &mut self,
        mut order: Order,
    ) -> Order {

        let (order_q, opposite_q) = match order.side {
            Side::Buy => (Rc::clone(&self.sell_q), Rc::clone(&self.buy_q)),
            Side::Sell => (Rc::clone(&self.buy_q), Rc::clone(&self.sell_q)),
        };

        let order = match order.order_type {
            OrderType::Limit => {
                order = self.process(&order_q, order);
                if !order.order_done() {
                    opposite_q.borrow_mut().insert(order);
                };
                order
            }
            OrderType::FillOrKill => self.process(&order_q, order),
            OrderType::ImmediateOrCancel => self.process(&order_q, order),
        };

        if self.verbose {
            match order.side {
                Side::Buy => {
                    print!("Order of type {:?}, width id {}, has been processed, {} of {} items have been bought", order.order_type, order.user_id,
                    order.start_quantity - order.quantity, order.start_quantity);
                    report(&order);
                },

                Side::Sell => {
                    print!("Order of type {:?}, with id {}, has been processed, {} of {} items have been sold", order.order_type, order.user_id,
                    order.start_quantity - order.quantity, order.start_quantity);
                    report(&order);
                },
            }
        }

        order
    }

    fn process(&mut self, q: &Rc<RefCell<OrderedSkipList<Order>>>, mut order: Order) -> Order {
   
        let mut idx = 0;

        while !q.borrow().is_empty() && idx < q.borrow().len() {
            
            let q_order = *q.borrow().get(&idx).unwrap();

            if !is_price_satisfied(&q_order, &order) {
                // don't need to scan further because all orders are sorted
                break;
            }

            if is_compatible(&q_order, &order) {
             
                let (tmp_cur_order, tmp_q_order) = process_orders(order, q_order);

                if tmp_cur_order.transaction_done() {
                    q.borrow_mut().remove_index(&idx);
                    if tmp_q_order.quantity != 0 {
                        q.borrow_mut().insert(tmp_q_order);
                    }
                    order = tmp_cur_order;

                }

                if order.quantity == 0 {
                    break;
                }
            }

            idx += 1;
        }

        order
    }
}
