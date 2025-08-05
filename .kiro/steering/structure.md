# Project Structure

## Root Directory Layout
```
tweet-scrolls/
├── src/
│   └── main.rs              # Single-file application with all logic
├── zzBackup/                # Archive folder for old code/references
├── .kiro/                   # Kiro AI assistant configuration
├── Cargo.toml               # Rust package manifest
├── Cargo.lock               # Dependency lock file
├── ReadMe.md                # Main documentation (Harry Potter themed)
├── AGENT.md                 # AI assistant task list
├── ref01-prd.txt            # Product requirements document
├── ref02-architecture.txt   # Architecture and data flow diagrams
└── visualASCII202410.txt    # ASCII art or visual documentation
```

## Code Organization
- **Single binary structure** - All code in `src/main.rs`
- **Monolithic design** - Functions organized by responsibility within single file
- **No modules** - Simple structure for a focused CLI tool

## Key Data Structures
```rust
Tweet           # Individual tweet data from JSON
TweetWrapper    # JSON wrapper containing tweet
Thread          # Collection of related tweets
CsvWriter       # Async CSV writing handler
```

## Function Organization (in main.rs)
- `main()` - Entry point and orchestration
- `get_input_file()` / `get_screen_name()` - User input handling
- `process_tweets()` - Core processing pipeline
- `write_threads_to_file()` - Text output generation
- `write_csv()` - CSV output generation
- `CsvWriter` impl - Async CSV writing logic

## Output Structure
Generated in input file's parent directory:
```
output_<handle>_<timestamp>/
├── threads_<handle>_<timestamp>.csv    # Structured data
├── threads_<handle>_<timestamp>.txt    # Human-readable threads
└── results_<handle>_<timestamp>.txt    # Operation summary
```

## Documentation Files
- `ref01-prd.txt` - Product requirements and specifications
- `ref02-architecture.txt` - Data flow and architecture diagrams
- `ReadMe.md` - User-facing documentation with whimsical theming