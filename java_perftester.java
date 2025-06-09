import java.time.Duration;
import java.time.Instant;
import java.time.LocalDate;
import java.util.HashMap;
import java.util.Map;

public class TradePerformanceJava {
    public static void main(String[] args) {
        TradeRepository repo = new TradeRepository();
        Instant start = Instant.now();

        // Add 1 million trades
        for (int i = 0; i < 1000000; i++) {
            repo.addTrade(new Trade(i, LocalDate.now(), "AAPL", 100, 100.0, Side.BUY));
        }

        Instant end = Instant.now();
        System.out.println("Java - Add trades: " + Duration.between(start, end).toMillis() + " ms");

        // Amend trades
        start = Instant.now();
        for (int i = 0; i < 1000000; i++) {
            repo.amendTrade(i, 150, 120.0);
        }
        end = Instant.now();
        System.out.println("Java - Amend trades: " + Duration.between(start, end).toMillis() + " ms");

        // Cancel trades
        start = Instant.now();
        for (int i = 0; i < 1000000; i++) {
            repo.cancelTrade(i);
        }
        end = Instant.now();
        System.out.println("Java - Cancel trades: " + Duration.between(start, end).toMillis() + " ms");
    }
}
