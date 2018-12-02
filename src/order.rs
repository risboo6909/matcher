use std::cmp::Ordering;


#[derive(Debug, Eq, PartialEq, Clone, Copy, Deserialize)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Deserialize)]
pub enum OrderType {
    Limit,
    FillOrKill,
    ImmediateOrCancel,
}

#[derive(Debug, Eq, Clone, Copy, Deserialize)]
pub struct Order {
    #[serde(rename = "Side")]
    pub side: Side,
    #[serde(rename = "PriceLimit")]
    pub price_limit: u64,
    #[serde(rename = "Quantity")]
    pub quantity: u64,
    #[serde(rename = "UserId")]
    pub user_id: u64,
    #[serde(rename = "OrderType")]
    pub order_type: OrderType,
    #[serde(skip_deserializing)]
    pub start_quantity: u64,
}

impl Order {
    pub fn new(
        side: Side,
        price_limit: u64,
        quantity: u64,
        user_id: u64,
        order_type: OrderType,
    ) -> Self {
        Order {
            side,
            price_limit,
            quantity,
            user_id,
            order_type,
            start_quantity: quantity,
        }
    }

    pub fn transaction_done(&self) -> bool {
        match self.order_type {
            OrderType::Limit | OrderType::ImmediateOrCancel => true,
            OrderType::FillOrKill => self.quantity == 0,
        }
    }

    pub fn order_done(&self) -> bool {
        match self.order_type {
            OrderType::Limit | OrderType::FillOrKill => self.quantity == 0,
            OrderType::ImmediateOrCancel => self.quantity < self.start_quantity,
        }
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
