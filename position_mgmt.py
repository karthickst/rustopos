from datetime import date

class Trade:
    def __init__(self, trade_id, trade_date, instrument, quantity, price, side):
        self.trade_id = trade_id
        self.trade_date = trade_date
        self.instrument = instrument
        self.quantity = quantity
        self.price = price
        self.side = side

class TradePosition:
    def __init__(self, instrument):
        self.instrument = instrument
        self.quantity = 0
        self.average_price = 0.0

    def update_position(self, trade):
        if trade.side == 'buy':
            self.quantity += trade.quantity
            self.average_price = (self.average_price * (self.quantity - trade.quantity) + trade.price * trade.quantity) / self.quantity
        elif trade.side == 'sell':
            self.quantity -= trade.quantity
            if self.quantity == 0:
                self.average_price = 0.0

class TradeRepository:
    def __init__(self):
        self.trades = {}
        self.positions = {}

    def add_trade(self, trade):
        self.trades[trade.trade_id] = trade
        if trade.instrument not in self.positions:
            self.positions[trade.instrument] = TradePosition(trade.instrument)
        self.positions[trade.instrument].update_position(trade)

    def amend_trade(self, trade_id, new_quantity=None, new_price=None):
        if trade_id in self.trades:
            trade = self.trades[trade_id]
            if new_quantity:
                trade.quantity = new_quantity
            if new_price:
                trade.price = new_price
            self.positions[trade.instrument].update_position(trade)

    def cancel_trade(self, trade_id):
        if trade_id in self.trades:
            trade = self.trades[trade_id]
            del self.trades[trade_id]
            self.positions[trade.instrument].quantity -= trade.quantity if trade.side == 'buy' else -trade.quantity
            if self.positions[trade.instrument].quantity == 0:
                self.positions[trade.instrument].average_price = 0.0

    def get_position(self, instrument):
        return self.positions.get(instrument)

# Example usage:
repo = TradeRepository()

# Add trades
repo.add_trade(Trade(1, date(2022, 1, 1), 'AAPL', 100, 100.0, 'buy'))
repo.add_trade(Trade(2, date(2022, 1, 2), 'AAPL', 50, 110.0, 'buy'))
repo.add_trade(Trade(3, date(2022, 1, 3), 'AAPL', 20, 120.0, 'sell'))

# Get position
position = repo.get_position('AAPL')
print(f"Quantity: {position.quantity}, Average Price: {position.average_price}")

# Amend trade
repo.amend_trade(2, new_quantity=70)

# Get updated position
position = repo.get_position('AAPL')
print(f"Quantity: {position.quantity}, Average Price: {position.average_price}")

# Cancel trade
repo.cancel_trade(1)

# Get updated position
position = repo.get_position('AAPL')
print(f"Quantity: {position.quantity}, Average Price: {position.average_price}")
