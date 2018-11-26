use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum OrderType {
    Limit,
    FillOrKill,
    ImmediateOrCancel,
}

#[derive(Debug, Eq, Clone, Copy)]
pub struct Order {
    pub side: Side,
    pub price_limit: u64,
    pub quantity: u64,
    pub user_id: u64,
    pub order_type: OrderType,
}

impl Order {
    pub fn order_done(&self) -> bool {
        self.quantity == 0
    }
}

impl Ord for Order {
    fn cmp(&self, other: &Order) -> Ordering {
        self.price_limit.cmp(&other.price_limit)
    }
}

impl PartialOrd for Order {
    fn partial_cmp(&self, other: &Order) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl PartialEq for Order {
    fn eq(&self, other: &Order) -> bool {
        self.price_limit == other.price_limit
    }
}
