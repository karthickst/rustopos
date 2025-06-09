import time
import datetime

class TradePerformancePython:
    def __init__(self):
        self.repo = TradeRepository()

    def run(self):
        start_time = time.time()

        # Add 1 million trades
        for i in range(1000000):
            self.repo.add_trade(Trade(i, datetime.date.today(), "AAPL", 100, 100.0, "buy"))

        end_time = time.time()
        print(f"Python - Add trades: {(end_time - start_time) * 1000:.2f} ms")

        # Amend trades
        start_time = time.time()
        for i in range(1000000):
            self.repo.amend_trade(i, 150, 120.0)
        end_time = time.time()
        print(f"Python - Amend trades: {(end_time - start_time) * 1000:.2f} ms")

        # Cancel trades
        start_time = time.time()
        for i in range(1000000):
            self.repo.cancel_trade(i)
        end_time = time.time()
        print(f"Python - Cancel trades: {(end_time - start_time) * 1000:.2f} ms")

if __name__ == "__main__":
    TradePerformancePython().run()
