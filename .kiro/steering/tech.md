# Technology Stack

## Language & Edition
- **Rust 2021 edition** - Modern Rust with latest language features
- **Tokio async runtime** - Full async/await support for I/O operations

## Core Dependencies
```toml
anyhow = "1.0"           # Error handling with context
chrono = "0.4"           # Date/time parsing and formatting  
serde = "1.0"            # JSON serialization/deserialization
serde_json = "1.0"       # JSON parsing
tokio = "1.0"            # Async runtime (full features)
csv = "1.1"              # CSV file generation
mimalloc = "0.1"         # Memory allocator optimization
```

## Build System
- **Cargo** - Standard Rust package manager and build tool
- Binary target: `tweet-scrolls` pointing to `src/main.rs`

## Common Commands
```bash
# Build and run the application
cargo run

# Build release version
cargo build --release

# Run tests (if any exist)
cargo test

# Check code without building
cargo check

# Format code
cargo fmt

# Lint code
cargo clippy
```

## Architecture Patterns
- **Async/await** throughout for I/O operations
- **Channel-based communication** for CSV writing (mpsc)
- **Buffered I/O** for performance optimization
- **Error propagation** using `Result<T>` and `anyhow::Context`
- **Memory optimization** with custom allocator (mimalloc)

## Performance Considerations
- Uses `tokio::spawn` for concurrent tasks
- Buffered CSV writing (100 record chunks)
- Async file I/O with `tokio::fs`
- Custom global allocator for memory efficiency