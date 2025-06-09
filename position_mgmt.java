import java.time.LocalDate;
import java.util.HashMap;
import java.util.Map;

enum Side {
    BUY,
    SELL
}

class Trade {
    private int tradeId;
    private LocalDate tradeDate;
    private String instrument;
    private int quantity;
    private double price;
    private Side side;

    public Trade(int tradeId, LocalDate tradeDate, String instrument, int quantity, double price, Side side) {
        this.tradeId = tradeId;
        this.tradeDate = tradeDate;
        this.instrument = instrument;
        this.quantity = quantity;
        this.price = price;
        this.side = side;
    }

    // Getters and setters
    public int getTradeId() { return tradeId; }
    public LocalDate getTradeDate() { return tradeDate; }
    public String getInstrument() { return instrument; }
    public int getQuantity() { return quantity; }
    public void setQuantity(int quantity) { this.quantity = quantity; }
    public double getPrice() { return price; }
    public void setPrice(double price) { this.price = price; }
    public Side getSide() { return side; }
}

class TradePosition {
    private String instrument;
    private int quantity;
    private double averagePrice;

    public TradePosition(String instrument) {
        this.instrument = instrument;
        this.quantity = 0;
        this.averagePrice = 0.0;
    }

    public void updatePosition(Trade trade) {
        if (trade.getSide() == Side.BUY) {
            this.quantity += trade.getQuantity();
            this.averagePrice = (this.averagePrice * (this.quantity - trade.getQuantity()) + trade.getPrice() * trade.getQuantity()) / this.quantity;
        } else if (trade.getSide() == Side.SELL) {
            this.quantity -= trade.getQuantity();
            if (this.quantity == 0) {
                this.averagePrice = 0.0;
            }
        }
    }

    public void cancelTrade(Trade trade) {
        if (trade.getSide() == Side.BUY) {
            this.quantity -= trade.getQuantity();
        } else if (trade.getSide() == Side.SELL) {
            this.quantity += trade.getQuantity();
        }
        if (this.quantity == 0) {
            this.averagePrice = 0.0;
        }
    }

    // Getters
    public String getInstrument() { return instrument; }
    public int getQuantity() { return quantity; }
    public double getAveragePrice() { return averagePrice; }
}

class TradeRepository {
    private Map<Integer, Trade> trades;
    private Map<String, TradePosition> positions;

    public TradeRepository() {
        this.trades = new HashMap<>();
        this.positions = new HashMap<>();
    }

    public void addTrade(Trade trade) {
        trades.put(trade.getTradeId(), trade);
        if (!positions.containsKey(trade.getInstrument())) {
            positions.put(trade.getInstrument(), new TradePosition(trade.getInstrument()));
        }
        positions.get(trade.getInstrument()).updatePosition(trade);
    }

    public void amendTrade(int tradeId, int newQuantity, double newPrice) {
        if (trades.containsKey(tradeId)) {
            Trade trade = trades.get(tradeId);
            TradePosition position = positions.get(trade.getInstrument());
            position.cancelTrade(trade);
            trade.setQuantity(newQuantity);
            trade.setPrice(newPrice);
            position.updatePosition(trade);
        }
    }

    public void cancelTrade(int tradeId) {
        if (trades.containsKey(tradeId)) {
            Trade trade = trades.get(tradeId);
            TradePosition position = positions.get(trade.getInstrument());
            position.cancelTrade(trade);
            trades.remove(tradeId);
        }
    }

    public TradePosition getPosition(String instrument) {
        return positions.get(instrument);
    }
}

public class Main {
    public static void main(String[] args) {
        TradeRepository repo = new TradeRepository();

        // Add trades
        repo.addTrade(new Trade(1, LocalDate.of(2022, 1, 1), "AAPL", 100, 100.0, Side.BUY));
        repo.addTrade(new Trade(2, LocalDate.of(2022, 1, 2), "AAPL", 50, 110.0, Side.BUY));
        repo.addTrade(new Trade(3, LocalDate.of(2022, 1, 3), "AAPL", 20, 120.0, Side.SELL));

        // Get position
        TradePosition position = repo.getPosition("AAPL");
        System.out.println("Quantity: " + position.getQuantity() + ", Average Price: " + position.getAveragePrice());

        // Amend trade
        repo.amendTrade(2, 70, 115.0);

        // Get updated position
        position = repo.getPosition("AAPL");
        System.out.println("Quantity: " + position.getQuantity() + ", Average Price: " + position.getAveragePrice());

        // Cancel trade
        repo.cancelTrade(1);

        // Get updated position
        position = repo.getPosition("AAPL");
        System.out.println("Quantity: " + position.getQuantity() + ", Average Price: " + position.getAveragePrice());
    }
}
