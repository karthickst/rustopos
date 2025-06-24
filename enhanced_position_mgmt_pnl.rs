use std::collections::HashMap;
use chrono::NaiveDate;

#[derive(Debug, Clone)]
enum TradeStatus {
    Active,
    Cancelled,
    Amended,
}

#[derive(Debug, Clone)]
enum TradeType {
    Market,
    Limit,
    Stop,
}

#[derive(Debug, Clone)]
struct TradeFilter {
    instrument: Option<String>,
    side: Option<Side>,
    trade_type: Option<TradeType>,
    status: Option<TradeStatus>,
    date_from: Option<NaiveDate>,
    date_to: Option<NaiveDate>,
    min_quantity: Option<i32>,
    max_quantity: Option<i32>,
    min_price: Option<f64>,
    max_price: Option<f64>,
}

impl TradeFilter {
    fn new() -> Self {
        TradeFilter {
            instrument: None,
            side: None,
            trade_type: None,
            status: None,
            date_from: None,
            date_to: None,
            min_quantity: None,
            max_quantity: None,
            min_price: None,
            max_price: None,
        }
    }

    fn instrument(mut self, instrument: String) -> Self {
        self.instrument = Some(instrument);
        self
    }

    fn side(mut self, side: Side) -> Self {
        self.side = Some(side);
        self
    }

    fn date_range(mut self, from: NaiveDate, to: NaiveDate) -> Self {
        self.date_from = Some(from);
        self.date_to = Some(to);
        self
    }

    fn quantity_range(mut self, min: i32, max: i32) -> Self {
        self.min_quantity = Some(min);
        self.max_quantity = Some(max);
        self
    }

    fn price_range(mut self, min: f64, max: f64) -> Self {
        self.min_price = Some(min);
        self.max_price = Some(max);
        self
    }
}

#[derive(Debug, Clone)]
struct Trade {
    trade_id: i32,
    trade_date: NaiveDate,
    instrument: String,
    quantity: i32,
    price: f64,
    side: Side,
    trade_type: TradeType,
    status: TradeStatus,
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
            trade_type: TradeType::Market,
            status: TradeStatus::Active,
        }
    }

    fn new_with_type(trade_id: i32, trade_date: NaiveDate, instrument: String, quantity: i32, price: f64, side: Side, trade_type: TradeType) -> Trade {
        Trade {
            trade_id,
            trade_date,
            instrument,
            quantity,
            price,
            side,
            trade_type,
            status: TradeStatus::Active,
        }
    }

    fn matches_filter(&self, filter: &TradeFilter) -> bool {
        if let Some(ref instr) = filter.instrument {
            if &self.instrument != instr { return false; }
        }
        if let Some(ref side) = filter.side {
            if !matches!((&self.side, side), (Side::Buy, Side::Buy) | (Side::Sell, Side::Sell)) { return false; }
        }
        if let Some(ref status) = filter.status {
            if !matches!((&self.status, status), (TradeStatus::Active, TradeStatus::Active) | (TradeStatus::Cancelled, TradeStatus::Cancelled) | (TradeStatus::Amended, TradeStatus::Amended)) { return false; }
        }
        if let Some(date_from) = filter.date_from {
            if self.trade_date < date_from { return false; }
        }
        if let Some(date_to) = filter.date_to {
            if self.trade_date > date_to { return false; }
        }
        if let Some(min_qty) = filter.min_quantity {
            if self.quantity < min_qty { return false; }
        }
        if let Some(max_qty) = filter.max_quantity {
            if self.quantity > max_qty { return false; }
        }
        if let Some(min_price) = filter.min_price {
            if self.price < min_price { return false; }
        }
        if let Some(max_price) = filter.max_price {
            if self.price > max_price { return false; }
        }
        true
    }
}

#[derive(Debug, Clone)]
struct TradePosition {
    instrument: String,
    quantity: i32,
    average_price: f64,
    realized_pnl: f64,  // P&L from closed positions
    total_cost: f64,    // Total amount invested
}

impl TradePosition {
    fn new(instrument: String) -> TradePosition {
        TradePosition {
            instrument,
            quantity: 0,
            average_price: 0.0,
            realized_pnl: 0.0,
            total_cost: 0.0,
        }
    }

    // Calculate unrealized P&L based on current market price
    fn unrealized_pnl(&self, current_price: f64) -> f64 {
        if self.quantity == 0 {
            0.0
        } else {
            (current_price - self.average_price) * self.quantity as f64
        }
    }

    // Calculate total P&L (realized + unrealized)
    fn total_pnl(&self, current_price: f64) -> f64 {
        self.realized_pnl + self.unrealized_pnl(current_price)
    }

    // Get current market value of position
    fn market_value(&self, current_price: f64) -> f64 {
        self.quantity as f64 * current_price
    }

    fn update_position(&mut self, trade: &Trade) {
        match trade.side {
            Side::Buy => {
                if self.quantity >= 0 {
                    // Adding to long position or starting new long position
                    if self.quantity > 0 {
                        let old_value = self.average_price * self.quantity as f64;
                        let new_value = trade.price * trade.quantity as f64;
                        self.total_cost += new_value;
                        self.quantity += trade.quantity;
                        self.average_price = (old_value + new_value) / self.quantity as f64;
                    } else {
                        // First buy or switching from flat to long
                        self.quantity += trade.quantity;
                        self.average_price = trade.price;
                        self.total_cost += trade.price * trade.quantity as f64;
                    }
                } else {
                    // Covering short position
                    let covered_quantity = std::cmp::min(trade.quantity, -self.quantity);
                    let remaining_buy = trade.quantity - covered_quantity;
                    
                    // Realize P&L on covered portion
                    self.realized_pnl += (self.average_price - trade.price) * covered_quantity as f64;
                    self.quantity += covered_quantity;
                    
                    if remaining_buy > 0 && self.quantity >= 0 {
                        // After covering short, now going long
                        self.quantity += remaining_buy;
                        self.average_price = trade.price;
                        self.total_cost = trade.price * remaining_buy as f64;
                    }
                }
            },
            Side::Sell => {
                if self.quantity > 0 {
                    // Selling from long position
                    let sold_quantity = std::cmp::min(trade.quantity, self.quantity);
                    let remaining_sell = trade.quantity - sold_quantity;
                    
                    // Realize P&L on sold portion
                    self.realized_pnl += (trade.price - self.average_price) * sold_quantity as f64;
                    self.quantity -= sold_quantity;
                    
                    if self.quantity == 0 {
                        self.average_price = 0.0;
                        self.total_cost = 0.0;
                    }
                    
                    if remaining_sell > 0 {
                        // After selling all long, now going short
                        self.quantity -= remaining_sell;
                        self.average_price = trade.price;
                    }
                } else {
                    // Adding to short position or starting new short position
                    if self.quantity < 0 {
                        let old_value = self.average_price * (-self.quantity) as f64;
                        let new_value = trade.price * trade.quantity as f64;
                        self.quantity -= trade.quantity;
                        self.average_price = (old_value + new_value) / (-self.quantity) as f64;
                    } else {
                        // Starting new short position
                        self.quantity -= trade.quantity;
                        self.average_price = trade.price;
                    }
                }
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
    // Market data for P&L calculations
    positions: HashMap<String, TradePosition>,
    market_prices: HashMap<String, f64>,
}

impl TradeRepository {
    fn new() -> TradeRepository {
        TradeRepository {
            trades: HashMap::new(),
            positions: HashMap::new(),
            market_prices: HashMap::new(),
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
        if let Some(mut trade) = self.trades.get_mut(&trade_id) {
            trade.status = TradeStatus::Cancelled;
            self.positions.get_mut(&trade.instrument).unwrap().cancel_trade(&trade);
        }
    }

    // Update market price for P&L calculations
    fn update_market_price(&mut self, instrument: &str, price: f64) {
        self.market_prices.insert(instrument.to_string(), price);
    }

    // Get current market price
    fn get_market_price(&self, instrument: &str) -> Option<f64> {
        self.market_prices.get(instrument).copied()
    }

    // Advanced trade filtering
    fn filter_trades(&self, filter: &TradeFilter) -> Vec<&Trade> {
        self.trades
            .values()
            .filter(|trade| trade.matches_filter(filter))
            .collect()
    }

    // Get trades by multiple criteria
    fn get_trades_by_criteria(&self, instruments: Vec<String>, side: Option<Side>, date_range: Option<(NaiveDate, NaiveDate)>) -> Vec<&Trade> {
        self.trades
            .values()
            .filter(|trade| {
                // Check instrument
                if !instruments.is_empty() && !instruments.contains(&trade.instrument) {
                    return false;
                }
                // Check side
                if let Some(ref filter_side) = side {
                    if !matches!((&trade.side, filter_side), (Side::Buy, Side::Buy) | (Side::Sell, Side::Sell)) {
                        return false;
                    }
                }
                // Check date range
                if let Some((start, end)) = date_range {
                    if trade.trade_date < start || trade.trade_date > end {
                        return false;
                    }
                }
                true
            })
            .collect()
    }

    // Calculate portfolio P&L
    fn calculate_portfolio_pnl(&self) -> (f64, f64, f64) {
        let mut total_realized = 0.0;
        let mut total_unrealized = 0.0;
        let mut total_market_value = 0.0;

        for (instrument, position) in &self.positions {
            total_realized += position.realized_pnl;
            
            if let Some(market_price) = self.get_market_price(instrument) {
                total_unrealized += position.unrealized_pnl(market_price);
                total_market_value += position.market_value(market_price);
            }
        }

        (total_realized, total_unrealized, total_market_value)
    }

    // Get top gainers/losers
    fn get_top_performers(&self, limit: usize, sort_by_unrealized: bool) -> Vec<(String, f64, f64)> {
        let mut performers: Vec<(String, f64, f64)> = self.positions
            .iter()
            .filter_map(|(instrument, position)| {
                if let Some(market_price) = self.get_market_price(instrument) {
                    let unrealized = position.unrealized_pnl(market_price);
                    let realized = position.realized_pnl;
                    Some((instrument.clone(), realized, unrealized))
                } else {
                    None
                }
            })
            .collect();

        if sort_by_unrealized {
            performers.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
        } else {
            performers.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        }

        performers.into_iter().take(limit).collect()
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

    // NEW: Print position summary with P&L as of date
    fn print_position_summary_as_of(&self, as_of_date: NaiveDate) {
        println!("\n=== Position Summary as of {} ===", as_of_date);
        let positions = self.build_position_map_as_of_date(as_of_date);
        let mut total_market_value = 0.0;
        let mut total_realized_pnl = 0.0;
        let mut total_unrealized_pnl = 0.0;
        
        for (instrument, position) in positions {
            if position.quantity != 0 {
                let market_price = self.get_market_price(&instrument).unwrap_or(position.average_price);
                let market_value = position.market_value(market_price);
                let unrealized_pnl = position.unrealized_pnl(market_price);
                
                total_market_value += market_value;
                total_realized_pnl += position.realized_pnl;
                total_unrealized_pnl += unrealized_pnl;
                
                println!("{}: {} shares @ ${:.2} avg | Market: ${:.2} | Value: ${:.2} | Realized P&L: ${:.2} | Unrealized P&L: ${:.2}", 
                    instrument, 
                    position.quantity, 
                    position.average_price,
                    market_price,
                    market_value,
                    position.realized_pnl,
                    unrealized_pnl
                );
            }
        }
        
        println!("\n--- Portfolio Summary ---");
        println!("Total Market Value: ${:.2}", total_market_value);
        println!("Total Realized P&L: ${:.2}", total_realized_pnl);
        println!("Total Unrealized P&L: ${:.2}", total_unrealized_pnl);
        println!("Total P&L: ${:.2}", total_realized_pnl + total_unrealized_pnl);
    }

    // Print trade analysis
    fn print_trade_analysis(&self, filter: &TradeFilter) {
        let filtered_trades = self.filter_trades(filter);
        
        println!("\n=== Trade Analysis ===");
        println!("Total trades matching filter: {}", filtered_trades.len());
        
        let mut total_volume = 0.0;
        let mut buy_volume = 0.0;
        let mut sell_volume = 0.0;
        let mut buy_count = 0;
        let mut sell_count = 0;
        
        for trade in &filtered_trades {
            let volume = trade.quantity as f64 * trade.price;
            total_volume += volume;
            
            match trade.side {
                Side::Buy => {
                    buy_volume += volume;
                    buy_count += 1;
                },
                Side::Sell => {
                    sell_volume += volume;
                    sell_count += 1;
                }
            }
        }
        
        println!("Buy trades: {} (Volume: ${:.2})", buy_count, buy_volume);
        println!("Sell trades: {} (Volume: ${:.2})", sell_count, sell_volume);
        println!("Total volume: ${:.2}", total_volume);
        
        if buy_count > 0 {
            println!("Average buy volume: ${:.2}", buy_volume / buy_count as f64);
        }
        if sell_count > 0 {
            println!("Average sell volume: ${:.2}", sell_volume / sell_count as f64);
        }
    }
}

fn main() {
    let mut repo = TradeRepository::new();

    // Add trades with different dates and types
    repo.add_trade(Trade::new(1, NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(), "AAPL".to_string(), 100, 100.0, Side::Buy));
    repo.add_trade(Trade::new_with_type(2, NaiveDate::from_ymd_opt(2022, 1, 2).unwrap(), "AAPL".to_string(), 50, 110.0, Side::Buy, TradeType::Limit));
    repo.add_trade(Trade::new(3, NaiveDate::from_ymd_opt(2022, 1, 3).unwrap(), "MSFT".to_string(), 200, 150.0, Side::Buy));
    repo.add_trade(Trade::new(4, NaiveDate::from_ymd_opt(2022, 1, 4).unwrap(), "AAPL".to_string(), 20, 120.0, Side::Sell));
    repo.add_trade(Trade::new(5, NaiveDate::from_ymd_opt(2022, 1, 5).unwrap(), "MSFT".to_string(), 50, 160.0, Side::Buy));
    repo.add_trade(Trade::new(6, NaiveDate::from_ymd_opt(2022, 1, 6).unwrap(), "AAPL".to_string(), 30, 125.0, Side::Sell));

    // Set current market prices for P&L calculations
    repo.update_market_price("AAPL", 135.0);
    repo.update_market_price("MSFT", 155.0);

    // Print positions with P&L
    repo.print_position_summary_as_of(NaiveDate::from_ymd_opt(2022, 1, 6).unwrap());

    // Test advanced filtering
    println!("\n=== Advanced Trade Filtering ===");
    
    // Filter by instrument and side
    let filter = TradeFilter::new()
        .instrument("AAPL".to_string())
        .side(Side::Buy);
    
    let aapl_buys = repo.filter_trades(&filter);
    println!("AAPL Buy trades: {}", aapl_buys.len());
    for trade in aapl_buys {
        println!("  {:?}", trade);
    }

    // Filter by date range and quantity
    let filter = TradeFilter::new()
        .date_range(
            NaiveDate::from_ymd_opt(2022, 1, 2).unwrap(),
            NaiveDate::from_ymd_opt(2022, 1, 4).unwrap()
        )
        .quantity_range(50, 200);
    
    repo.print_trade_analysis(&filter);

    // Portfolio P&L analysis
    let (realized, unrealized, market_value) = repo.calculate_portfolio_pnl();
    println!("\n=== Portfolio P&L Summary ===");
    println!("Total Realized P&L: ${:.2}", realized);
    println!("Total Unrealized P&L: ${:.2}", unrealized);
    println!("Total Market Value: ${:.2}", market_value);
    println!("Total P&L: ${:.2}", realized + unrealized);

    // Top performers
    println!("\n=== Top Performers (by Unrealized P&L) ===");
    let top_performers = repo.get_top_performers(5, true);
    for (instrument, realized, unrealized) in top_performers {
        println!("{}: Realized: ${:.2}, Unrealized: ${:.2}, Total: ${:.2}", 
            instrument, realized, unrealized, realized + unrealized);
    }

    // Test amend by date with P&L impact
    println!("\n=== Testing Amend with P&L Impact ===");
    println!("Before amend:");
    repo.print_position_summary_as_of(NaiveDate::from_ymd_opt(2022, 1, 6).unwrap());
    
    match repo.amend_trade_by_date("AAPL", NaiveDate::from_ymd_opt(2022, 1, 2).unwrap(), 75, 115.0) {
        Ok(()) => {
            println!("\nAfter amending AAPL trade from 2022-01-02:");
            repo.print_position_summary_as_of(NaiveDate::from_ymd_opt(2022, 1, 6).unwrap());
        },
        Err(e) => println!("Error: {}", e),
    }

    // Multi-criteria search
    println!("\n=== Multi-Criteria Search ===");
    let multi_results = repo.get_trades_by_criteria(
        vec!["AAPL".to_string(), "MSFT".to_string()],
        Some(Side::Buy),
        Some((
            NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2022, 1, 5).unwrap()
        ))
    );
    
    println!("Buy trades for AAPL/MSFT from Jan 1-5:");
    for trade in multi_results {
        println!("  {} {} {} shares @ ${:.2} on {}", 
            trade.instrument, 
            match trade.side { Side::Buy => "BUY", Side::Sell => "SELL" },
            trade.quantity, 
            trade.price, 
            trade.trade_date
        );
    }
} AAPL trade from 2022-01-02...");
    
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