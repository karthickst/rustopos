use std::time::{Duration, Instant};

fn main() {
    let mut repo = TradeRepository::new();
    let start = Instant::now();

    // Add 1 million trades
    for i in 0..1000000 {
        repo.add_trade(Trade::new(i as i32, chrono::Local::today().naive_local(), "AAPL".to_string(), 100, 100.0, Side::Buy));
    }

    let duration = start.elapsed();
    println!("Rust - Add trades: {} ms", duration.as_millis());

    let start = Instant::now();
    // Amend trades
    for i in 0..1000000 {
        repo.amend_trade(i as i32, 150, 120.0);
    }
    let duration = start.elapsed();
    println!("Rust - Amend trades: {} ms", duration.as_millis());

    let start = Instant::now();
    // Cancel trades
    for i in 0..1000000 {
        repo.cancel_trade(i as i32);
    }
    let duration = start.elapsed();
    println!("Rust - Cancel trades: {} ms", duration.as_millis());
}
