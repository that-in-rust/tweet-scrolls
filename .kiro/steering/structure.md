# Project Structure

## Root Directory Layout
```
tweet-scrolls/
├── src/
│   ├── main.rs              # Core application logic (MAX 800 lines)
│   └── integration_tests.rs # All test code moved here
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

## Code Organization Principles

### File Size Limits (CRITICAL)
- **main.rs**: Maximum 800 lines - LLMs cannot handle larger files effectively
- **integration_tests.rs**: All test code goes here, no size limit
- **Any file**: If approaching 800 lines, refactor into smaller functions or modules

### TDD-Driven Modular Design

#### Test-Driven Development Cycle
1. **Red**: Write a failing test first
2. **Green**: Write minimal code to make test pass
3. **Refactor**: Clean up code while keeping tests passing
4. **Repeat**: Continue with next small feature

#### Function Design Principles
- **Small, Focused Functions**: Each function should do one thing well (10-50 lines max)
- **Pure Functions**: Prefer functions without side effects when possible
- **Clear Function Names**: Function names should describe exactly what they do
- **Single Responsibility**: One function = one responsibility
- **Testable Units**: Every function should be easily testable in isolation

#### Code Quality Standards
- **No Function Over 50 Lines**: Break down complex functions into smaller ones
- **Descriptive Variable Names**: `user_hash` not `uh`, `response_times` not `rt`
- **Error Handling**: Use `Result<T>` and `anyhow::Context` consistently
- **Documentation**: Public functions must have doc comments with examples

#### Modular Organization Within Single File
```rust
// 1. Imports and global allocator
// 2. Data structures (structs, enums)
// 3. Core business logic functions
// 4. I/O and file processing functions
// 5. Helper and utility functions
// 6. Main function (orchestration only)
```

#### Test Organization (in integration_tests.rs)
```rust
// 1. Helper functions for creating test data
// 2. Unit tests grouped by functionality
// 3. Integration tests for end-to-end workflows
// 4. Property-based tests for complex logic
```

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