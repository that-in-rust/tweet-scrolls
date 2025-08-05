# AI Assistant Guidelines for Tweet-Scrolls Development

## Project Overview
Tweet-Scrolls is a Rust CLI application that processes Twitter archive JSON files to extract and organize tweet threads and direct message conversations. The application follows Test-Driven Development (TDD) principles and maintains comprehensive documentation.

## Core Functionality
1. **Tweet Thread Processing**: Parses Twitter JSON exports, reconstructs conversation threads, filters out retweets, and outputs organized data
2. **Direct Message Processing**: Handles DM conversation analysis with support for multiple JavaScript prefix formats
3. **Dual Output Formats**: Generates both CSV files for data analysis and human-readable TXT files
4. **Performance Optimized**: Uses async I/O, buffered writing, and memory optimization

## Development Guidelines

### Test-Driven Development (TDD)
- **Always write tests first** for new functionality
- Current test coverage includes:
  - `test_dm_processing()` - Core DM functionality
  - `test_dm_processing_with_empty_messages()` - Edge case handling
  - `test_dm_javascript_prefix_removal()` - Original prefix format
  - `test_dm_headers_prefix_removal()` - Headers prefix format
- **Test edge cases**: Empty data, malformed input, different file formats
- **Use tempfile crate** for test isolation and cleanup

### Code Quality Standards
- **Error Handling**: Use `anyhow::Context` for descriptive error messages
- **Async/Await**: Leverage Tokio for I/O operations and concurrent processing
- **Memory Management**: Utilize mimalloc allocator and buffered I/O
- **Code Style**: Run `cargo clippy` and fix all warnings before commits

### Documentation Requirements
- **ReadMe.md**: Harry Potter themed user documentation with usage examples
- **ref01-prd.txt**: Product requirements document with feature specifications
- **ref02-architecture.txt**: Technical architecture and data flow diagrams
- **Inline Comments**: Comprehensive code documentation with memory layout explanations

### Architecture Principles
- **Single-file Design**: Keep main functionality in `src/main.rs` for simplicity
- **Separation of Concerns**: Distinct functions for tweet vs DM processing
- **Channel-based I/O**: Use mpsc channels for async CSV writing
- **Resource Management**: Proper cleanup of file handles and async tasks

### Theme Consistency
- **Progress Messages**: Marvel Avengers themed status updates during processing
- **Documentation**: Harry Potter themed user-facing content
- **Error Messages**: Clear, contextual error reporting with helpful suggestions

### Development Workflow
1. **Write failing tests** for new functionality
2. **Implement minimal code** to make tests pass
3. **Refactor** for clarity and performance
4. **Update documentation** to reflect changes
5. **Run full test suite** and clippy before completion

### Key Dependencies
- `tokio` - Async runtime with full features
- `serde` + `serde_json` - JSON serialization/deserialization
- `anyhow` - Error handling with context
- `csv` - CSV file generation
- `chrono` - Date/time parsing and formatting
- `mimalloc` - Memory allocator optimization
- `tempfile` - Test isolation (dev dependency)

### Performance Considerations
- **Async I/O**: Use `tokio::fs` for file operations
- **Buffered Writing**: Implement chunked CSV writing (100 records)
- **Memory Efficiency**: Avoid unnecessary cloning, use references where possible
- **Concurrent Processing**: Leverage `tokio::spawn_blocking` for CPU-intensive tasks

### Future Enhancement Areas
1. **CLI Arguments**: Add clap for command-line argument parsing
2. **Configuration**: Support for config files (TOML/JSON)
3. **Progress Indicators**: Add progress bars for long operations
4. **Additional Formats**: JSON export, XML output options
5. **Batch Processing**: Handle multiple files in single operation

### Testing Strategy
- **Unit Tests**: Test individual functions with isolated data
- **Integration Tests**: Test complete processing pipelines
- **Edge Case Coverage**: Empty files, malformed JSON, missing fields
- **Performance Tests**: Validate memory usage and processing speed
- **Error Scenario Tests**: Network failures, permission issues, corrupted data

### Debugging Guidelines
- **Use `cargo test`** for comprehensive test validation
- **Run `cargo clippy`** for code quality checks
- **Enable debug logging** with environment variables when needed
- **Profile memory usage** for large file processing
- **Validate output files** in tests to ensure correctness

## Current Status
- ✅ Core tweet threading functionality complete
- ✅ DM processing with dual prefix format support
- ✅ Comprehensive test suite (4 tests, all passing)
- ✅ Production-ready error handling and performance optimization
- ✅ Complete documentation suite with themed content

## Next Development Priorities
1. User experience improvements (CLI args, progress bars)
2. Configuration system for customizable behavior
3. Additional output format support
4. Performance profiling and optimization
5. Enhanced error recovery and partial processing capabilities