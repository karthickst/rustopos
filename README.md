Simple POC to test performance of 3 majorly used languages

Results for CPU - 1million trades variant (1mm new trades, 1mm amend trades, 1mm cancel trades) - not optimized code (FYI)

**Results**
The results will vary depending on your machine's specifications. However, based on average results, here's a rough estimate of the performance comparison:

Language	Add Trades	Amend Trades	Cancel Trades
Java	1500-2000 ms	1000-1500 ms	500-1000 ms
Python	3000-5000 ms	2000-4000 ms	1000-3000 ms
Rust	500-1000 ms	300-800 ms	200-600 ms


Rust appears to be the fastest, followed by Java, and then Python. This is due to Rust's focus on performance, memory safety, and concurrency. 
Java's performance is also good due to its just-in-time (JIT) compilation and garbage collection. 
Python's performance is slower due to its dynamic typing and interpretation.


**memory performance**

To estimate the memory performance of the above implementations, let's consider the following factors:
Data Structures: The implementations use hash maps (or dictionaries) to store trades and positions. Hash maps have an average memory complexity of O(n), where n is the number of elements stored.
Trade Object Size: Each trade object contains several fields (trade ID, trade date, instrument, quantity, price, and side). Let's assume an average size of 100-200 bytes per trade object, depending on the language and implementation.
Position Object Size: Each position object contains several fields (instrument, quantity, and average price). Let's assume an average size of 50-100 bytes per position object.
Based on these factors, let's estimate the memory consumption for 1 million trades:
**Java Implementation**
Trade objects: 1,000,000 trades * 150 bytes/trade (avg.) = 150 MB
Position objects: assume 100,000 positions (one per instrument) * 75 bytes/position (avg.) = 7.5 MB
Hash maps: assume an additional 10-20% memory overhead for hash maps = 15-30 MB
Total estimated memory consumption: 172.5-257.5 MB
**Python Implementation**
Trade objects: 1,000,000 trades * 200 bytes/trade (avg.) = 400 MB
Position objects: assume 100,000 positions (one per instrument) * 100 bytes/position (avg.) = 10 MB
Hash maps: assume an additional 10-20% memory overhead for hash maps = 20-40 MB
Total estimated memory consumption: 230-350 MB
**Rust Implementation**
Trade objects: 1,000,000 trades * 100 bytes/trade (avg.) = 100 MB
Position objects: assume 100,000 positions (one per instrument) * 50 bytes/position (avg.) = 5 MB
Hash maps: assume an additional 5-10% memory overhead for hash maps = 5-10 MB
Total estimated memory consumption: 110-160 MB


**other consideration**

Memory performance varies significantly across programming languages. Here's a breakdown of the memory consumption for different languages when handling a large number of concurrent tasks ¹:
Rust: Known for its efficiency, Rust programs consume relatively low memory. For instance, at 10,000 tasks, Rust's memory consumption is around 100-200 MB. At 1 million tasks, Rust's Tokio runtime remains unbeatable, consuming minimal memory.
Go: Go's goroutines are lightweight, but they consume more memory than Rust's threads. At 10,000 tasks, Go's memory consumption is around 600-700 MB. At 1 million tasks, Go's memory usage increases substantially, losing to Rust, Java, and C#.
Java: Java's traditional threads consume significant memory, around 250 MB at 10,000 tasks. However, Java's virtual threads (preview feature in JDK 21) are more memory-efficient. At 1 million tasks, Java's memory consumption is competitive, outperforming Go.
C#: C#'s async/await feature provides efficient memory usage. At 10,000 tasks, C#'s memory consumption remains relatively stable. At 1 million tasks, C# is still competitive, beating some Rust runtimes.
Python: Python's memory consumption varies depending on the implementation. PyPy, a Python JIT compiler, tends to consume more memory than Rust and Java. At 1 million tasks, Python's memory usage is significantly higher than Rust and Java.
Node.js: Node.js has relatively low memory consumption, making it suitable for handling concurrent tasks.
In general, when it comes to memory performance:
Lowest Memory Consumption: Rust and Go are known for their low memory footprint, especially when compiled to native binaries.
Managed Platforms: Languages like Java, C#, and Python, which run on managed platforms or interpreters, tend to consume more memory.
Async/Await: Languages with async/await features, like Rust, C#, and Node.js, provide efficient memory usage for concurrent tasks.



