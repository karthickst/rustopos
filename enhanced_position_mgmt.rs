use std::collections::HashMap;
use chrono::NaiveDate;

#[derive(Debug, Clone)]
enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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
                // Fixed average price calculation
                if self.quantity > 0 {
                    let old_value = self.average_price * self.quantity as f64;
                    let new_value = trade.price * trade.quantity as f64;
                    self.quantity += trade.quantity;
                    self.average_price = (old_value + new_value) / self.quantity as f64;
                } else {
                    // First buy or switching from short to long
                    self.quantity += trade.quantity;
                    self.average_price = trade.price;
                }
            },
            Side::Sell => {
                self.quantity -= trade.quantity;
                if self.quantity == 0 {
                    self.average_price = 0.0;
                }
                // Note: For sells, we don't change average price of remaining position
            }
        }
    }

    fn cancel_trade(&mut self, trade: &Trade) {
        match trade.side {
            Side::Buy => {
                if self.quantity > trade.quantity {
                    // Recalculate average price after removing this buy
                    let total_value = self.average_price * self.quantity as f64;
                    let trade_value = trade.price * trade.quantity as f64;
                    self.quantity -= trade.quantity;
                    if self.quantity > 0 {
                        self.average_price = (total_value - trade_value) / self.quantity as f64;
                    } else {
                        self.average_price = 0.0;
                    }
                } else {
                    self.quantity -= trade.quantity;
                    if self.quantity == 0 {
                        self.average_price = 0.0;
                    }
                }
            },
            Side::Sell => {
                self.quantity += trade.quantity;
            }
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
        let instrument = trade.instrument.clone();
        self.trades.insert(trade.trade_id, trade.clone());
        
        if !self.positions.contains_key(&instrument) {
            self.positions.insert(instrument.clone(), TradePosition::new(instrument.clone()));
        }
        self.positions.get_mut(&instrument).unwrap().update_position(&trade);
    }

    fn amend_trade(&mut self, trade_id: i32, new_quantity: i32, new_price: f64) {
        if let Some(trade) = self.trades.get_mut(&trade_id) {
            let instrument = trade.instrument.clone();
            // Cancel old trade effect
            self.positions.get_mut(&instrument).unwrap().cancel_trade(trade);
            // Update trade
            trade.quantity = new_quantity;
            trade.price = new_price;
            // Apply new trade effect
            self.positions.get_mut(&instrument).unwrap().update_position(trade);
        }
    }

    // NEW: Amend trade based on date
    fn amend_trade_by_date(&mut self, instrument: &str, trade_date: NaiveDate, new_quantity: i32, new_price: f64) -> Result<(), String> {
        // Find trade by instrument and date
        let trade_id = self.trades
            .iter()
            .find(|(_, trade)| trade.instrument == instrument && trade.trade_date == trade_date)
            .map(|(id, _)| *id);

        match trade_id {
            Some(id) => {
                self.amend_trade(id, new_quantity, new_price);
                Ok(())
            },
            None => Err(format!("No trade found for {} on {}", instrument, trade_date))
        }
    }

    // NEW: Find trades by date range
    fn find_trades_by_date(&self, start_date: NaiveDate, end_date: NaiveDate) -> Vec<&Trade> {
        self.trades
            .values()
            .filter(|trade| trade.trade_date >= start_date && trade.trade_date <= end_date)
            .collect()
    }

    fn cancel_trade(&mut self, trade_id: i32) {
        if let Some(trade) = self.trades.remove(&trade_id) {
            self.positions.get_mut(&trade.instrument).unwrap().cancel_trade(&trade);
        }
    }

    fn get_position(&self, instrument: &str) -> Option<&TradePosition> {
        self.positions.get(instrument)
    }

    fn get_all_positions(&self) -> &HashMap<String, TradePosition> {
        &self.positions
    }

    // NEW: Build position map as of a specific date
    fn build_position_map_as_of_date(&self, as_of_date: NaiveDate) -> HashMap<String, TradePosition> {
        let mut positions_map: HashMap<String, TradePosition> = HashMap::new();
        
        // Get all trades up to and including the specified date
        let mut relevant_trades: Vec<&Trade> = self.trades
            .values()
            .filter(|trade| trade.trade_date <= as_of_date)
            .collect();
        
        // Sort trades by date to ensure proper chronological processing
        relevant_trades.sort_by(|a, b| a.trade_date.cmp(&b.trade_date));
        
        // Process trades chronologically to build positions
        for trade in relevant_trades {
            if !positions_map.contains_key(&trade.instrument) {
                positions_map.insert(trade.instrument.clone(), TradePosition::new(trade.instrument.clone()));
            }
            positions_map.get_mut(&trade.instrument).unwrap().update_position(trade);
        }
        
        positions_map
    }

    // NEW: Get position history for an instrument over date range
    fn get_position_history(&self, instrument: &str, start_date: NaiveDate, end_date: NaiveDate) -> Vec<(NaiveDate, TradePosition)> {
        let mut history = Vec::new();
        let mut current_date = start_date;
        
        while current_date <= end_date {
            let position_map = self.build_position_map_as_of_date(current_date);
            if let Some(position) = position_map.get(instrument) {
                history.push((current_date, position.clone()));
            } else {
                history.push((current_date, TradePosition::new(instrument.to_string())));
            }
            current_date = current_date.succ_opt().unwrap_or(current_date);
        }
        
        history
    }

    // NEW: Print position summary as of date
    fn print_position_summary_as_of(&self, as_of_date: NaiveDate) {
        println!("\n=== Position Summary as of {} ===", as_of_date);
        let positions = self.build_position_map_as_of_date(as_of_date);
        
        for (instrument, position) in positions {
            if position.quantity != 0 {
                println!("{}: {} shares @ ${:.2} avg price (Total Value: ${:.2})", 
                    instrument, 
                    position.quantity, 
                    position.average_price,
                    position.quantity as f64 * position.average_price
                );
            }
        }
    }
}

fn main() {
    let mut repo = TradeRepository::new();

    // Add trades with different dates
    repo.add_trade(Trade::new(1, NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(), "AAPL".to_string(), 100, 100.0, Side::Buy));
    repo.add_trade(Trade::new(2, NaiveDate::from_ymd_opt(2022, 1, 2).unwrap(), "AAPL".to_string(), 50, 110.0, Side::Buy));
    repo.add_trade(Trade::new(3, NaiveDate::from_ymd_opt(2022, 1, 3).unwrap(), "MSFT".to_string(), 200, 150.0, Side::Buy));
    repo.add_trade(Trade::new(4, NaiveDate::from_ymd_opt(2022, 1, 4).unwrap(), "AAPL".to_string(), 20, 120.0, Side::Sell));
    repo.add_trade(Trade::new(5, NaiveDate::from_ymd_opt(2022, 1, 5).unwrap(), "MSFT".to_string(), 50, 160.0, Side::Buy));

    // Print positions as of different dates
    repo.print_position_summary_as_of(NaiveDate::from_ymd_opt(2022, 1, 1).unwrap());
    repo.print_position_summary_as_of(NaiveDate::from_ymd_opt(2022, 1, 3).unwrap());
    repo.print_position_summary_as_of(NaiveDate::from_ymd_opt(2022, 1, 5).unwrap());

    // Test amend by date - modify the AAPL trade from Jan 2nd
    println!("\n=== Testing Amend by Date ===");
    println!("Amending AAPL trade from 2022-01-02...");
    
    match repo.amend_trade_by_date("AAPL", NaiveDate::from_ymd_opt(2022, 1, 2).unwrap(), 75, 115.0) {
        Ok(()) => println!("Successfully amended trade"),
        Err(e) => println!("Error: {}", e),
    }

    // Show position after amendment
    repo.print_position_summary_as_of(NaiveDate::from_ymd_opt(2022, 1, 5).unwrap());

    // Show trades in date range
    println!("\n=== Trades from 2022-01-01 to 2022-01-03 ===");
    let trades_in_range = repo.find_trades_by_date(
        NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(),
        NaiveDate::from_ymd_opt(2022, 1, 3).unwrap()
    );
    
    for trade in trades_in_range {
        println!("{:?}", trade);
    }

    // Show position history for AAPL
    println!("\n=== AAPL Position History ===");
    let history = repo.get_position_history(
        "AAPL",
        NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(),
        NaiveDate::from_ymd_opt(2022, 1, 5).unwrap()
    );
    
    for (date, position) in history {
        println!("{}: {} shares @ ${:.2}", date, position.quantity, position.average_price);
    }
}