pub struct OrderBook {
    pub(super) symbol: String,
    pub(super) bids: DashMap<u64, Arc<PriceLevel>>,
    pub(super) asks: DashMap<u64, Arc<PriceLevel>>,
    pub(super) order_locations: DashMap<OrderId, (u64, Side)>,
    pub(super) transaction_id_generator: AtomicU64,
    pub(super) last_trade: DashMap<String, LastTrade>,
    pub(super) has_market_close: AtomicBool,
    pub(super) market_close_timestamp: AtomicU64,
    pub(super) cache: PriceLevelCache,
    pub(super) is_active: AtomicBool,
    pub(super) listener: Arc<Mutex<Listener>>,
}