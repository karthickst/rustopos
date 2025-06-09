use std::collections::HashMap;
use chrono::NaiveDate;

#[derive(Debug)]
enum Side {
    Buy,
    Sell,
}

#[derive(Debug)]
struct Trade {
    trade_id: i32,
    trade_date: NaiveDate,
    instrument: String,
    quantity: i32,
    price: f64,
    side: Side,
}

impl Trade {
    fn new(trade_id: i32, trade_date: NaiveDate, instrument: String, quantity: i32, price: f64, side: Side) -> Trade {
        Trade {
            trade_id,
            trade_date,
            instrument,
            quantity,
            price,
            side,
        }
    }
}

#[derive(Debug)]
struct TradePosition {
    instrument: String,
    quantity: i32,
    average_price: f64,
}

impl TradePosition {
    fn new(instrument: String) -> TradePosition {
        TradePosition {
            instrument,
            quantity: 0,
            average_price: 0.0,
        }
    }

    fn update_position(&mut self, trade: &Trade) {
        match trade.side {
            Side::Buy => {
                self.quantity += trade.quantity;
                self.average_price = (self.average_price * (self.quantity - trade.quantity) as f64 + trade.price * trade.quantity as f64) / self.quantity as f64;
            },
            Side::Sell => {
                self.quantity -= trade.quantity;
                if self.quantity == 0 {
                    self.average_price = 0.0;
                }
            }
        }
    }

    fn cancel_trade(&mut self, trade: &Trade) {
        match trade.side {
            Side::Buy => {
                self.quantity -= trade.quantity;
            },
            Side::Sell => {
                self.quantity += trade.quantity;
            }
        }
        if self.quantity == 0 {
            self.average_price = 0.0;
        }
    }
}

#[derive(Debug)]
struct TradeRepository {
    trades: HashMap<i32, Trade>,
    positions: HashMap<String, TradePosition>,
}

impl TradeRepository {
    fn new() -> TradeRepository {
        TradeRepository {
            trades: HashMap::new(),
            positions: HashMap::new(),
        }
    }

    fn add_trade(&mut self, trade: Trade) {
        self.trades.insert(trade.trade_id, trade.clone());
        if !self.positions.contains_key(&trade.instrument) {
            self.positions.insert(trade.instrument.clone(), TradePosition::new(trade.instrument.clone()));
        }
        self.positions.get_mut(&trade.instrument).unwrap().update_position(&trade);
    }

    fn amend_trade(&mut self, trade_id: i32, new_quantity: i32, new_price: f64) {
        if let Some(trade) = self.trades.get_mut(&trade_id) {
            let instrument = trade.instrument.clone();
            self.positions.get_mut(&instrument).unwrap().cancel_trade(trade);
            trade.quantity = new_quantity;
            trade.price = new_price;
            self.positions.get_mut(&instrument).unwrap().update_position(trade);
        }
    }

    fn cancel_trade(&mut self, trade_id: i32) {
        if let Some(trade) = self.trades.remove(&trade_id) {
            self.positions.get_mut(&trade.instrument).unwrap().cancel_trade(&trade);
        }
    }

    fn get_position(&self, instrument: &str) -> Option<&TradePosition> {
        self.positions.get(instrument)
    }
}

fn main() {
    let mut repo = TradeRepository::new();

    // Add trades
    repo.add_trade(Trade::new(1, NaiveDate::from_ymd(2022, 1, 1), "AAPL".to_string(), 100, 100.0, Side::Buy));
    repo.add_trade(Trade::new(2, NaiveDate::from_ymd(2022, 1, 2), "AAPL".to_string(), 50, 110.0, Side::Buy));
    repo.add_trade(Trade::new(3, NaiveDate::from_ymd(2022, 1, 3), "AAPL".to_string(), 20, 120.0, Side::Sell));

    // Get position
    let position = repo.get_position("AAPL").unwrap();
    println!("Quantity: {}, Average Price: {}", position.quantity, position.average_price);

    // Amend trade
    repo.amend_trade(2, 70, 115.0);

    // Get updated position
    let position = repo.get_position("AAPL").unwrap();
    println!("Quantity: {}, Average Price: {}", position.quantity, position.average_price);

    // Cancel trade
    repo.cancel_trade(1);

    // Get updated position
    let position = repo.get_position("AAPL").unwrap();
    println!("Quantity: {}, Average Price: {}", position.quantity, position.average_price);
}
