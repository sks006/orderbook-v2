## AtomicU64
AtomicU64 solves this problem. It's a special type that guarantees that any operation on it is atomic, or "uninterruptible." It means that when one thread is performing an operation on an AtomicU64, no other thread can read or write to it until that operation is completely finished.

## Arc
arc job is to manage the memory of the data it points to. It makes sure the data isn't deleted while any thread is still using it. It's a safety feature for memory, not for access.

## PriceLevel
enterprise-level matching engine is price-time priority. The DashMap gives us the price priority, but the PriceLevel needs to handle the time priority. This means it needs to keep orders that arrive at the same price in the exact order they were submitted.