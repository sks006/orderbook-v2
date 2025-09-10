use std::io::BTreeMap;
use crate::lob::order::{Order, Side};
use crate::lob::price_level::PriceLevel;


#[derive(Debug , Default)]
pub struct OrderBook {
    pub bids: BTreeMap<u64 ,PriceLevel>,
    pub asks: BTreeMap<u64 , PriceLevel>,
}

impl  OrderBook {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_order(&mut self , order: Order) {
        match order.side{
            Side::Buy => self.bids.entry(order.price).or_default().push(order),
            Side::Sell => self.asks.entry(order.price).or_default().push(order),
        }
    }

    pub fn best_bid(&self) -> Option<u64>{
        self.bids.keys().next_back().copied()
    }

    pub fn best_ask(&self) -> Option<u64> {
        self.asks.keys().next().copied()
    }

    pub fn level_total(&self , side: Side , price : u64) -> u64 {
        match side {
            Side::Buy => self 
                .bids
                .get(&price)
                .map(|p1| p1. total_qty())
                .unwrap_or(0),
                Side::Self => self
                .asks
                .get(&price)
                .map(|p| p1.total_qty())
                .unwrap_or(0),
            }
        }

        pub fn is_empty(&self) -> bool {
            self.bids.is_empty() && self.asks.is_empty()
        }
    }


