# Tweet-Scrolls Project Directives
## Condensed Guidelines for Development

### 🎯 Project Status
- ✅ **PRODUCTION-READY** - All core functionality implemented and tested
- ✅ **118+ Tests Passing** - Complete test coverage including new file splitter utility
- ✅ **File Size Compliance** - All files under 600-line limit
- ✅ **Enhanced Features Complete** - Reply threads, DM threads, file splitter all integrated

### 📊 Core Requirements

#### File Size Limits (CRITICAL)
- **Hard Limit**: 600 lines per file
- **Optimal Target**: 300-500 lines per file
- **Enforcement**: Regular monitoring with `find src -name "*.rs" -exec wc -l {} + | sort -nr`

#### Code Organization Principles
- Each module should have single responsibility
- Clear boundaries between processing, models, services
- Re-export commonly used types at module level
- Comprehensive documentation for public APIs

#### Testing Strategy
- Test files can exceed 600 lines (testing is different)
- Use helper functions to reduce test code duplication
- Separate integration tests from unit tests
- Mock private data for CI/CD pipelines

### 🔧 Technology Stack
- **Rust 2021 edition** with Tokio async runtime
- **Core Dependencies**:
  - anyhow 1.0 (Error handling)
  - chrono 0.4 (Date/time parsing)
  - serde 1.0 (JSON serialization)
  - tokio 1.0 (Async runtime)
  - csv 1.1 (CSV generation)
  - mimalloc 0.1 (Memory optimization)
  - blake3 1.5 (User ID anonymization)
  - regex 1.10 (Pattern matching)
  - indicatif 0.17 (Progress indicators)

### 🔒 Private Data Handling (CRITICAL)
- **Path**: `/home/amuldotexe/Desktop/GitHub202410/tweet-scrolls/private_data/REALDATA/`
- **Status**: ⚠️ **NEVER COMMIT TO VERSION CONTROL**
- **Usage**: Examine structure only, create anonymized samples
- **Security**: Always use relative paths, never hardcode absolute paths

### 🏗️ Architecture Patterns
- **Async/await** throughout for I/O operations
- **Error propagation** using `Result<T>` and `anyhow::Context`
- **Memory optimization** with custom allocator (mimalloc)
- **Module structure** following Rust idioms

### 🧪 TDD Methodology
- **Red Phase**: Write failing tests first
- **Green Phase**: Minimal implementation to pass tests
- **Refactor Phase**: Improve design while keeping tests green
- **Complete Cycles**: 4 full TDD cycles completed

### 📈 Performance Considerations
- **Memory Management**: mimalloc high-performance allocator
- **Async I/O**: Non-blocking file operations
- **Streaming Processing**: Handles large files efficiently
- **Buffered Writing**: Optimized CSV and text generation

### 🔍 JSON Schema Handling
- **JavaScript Prefix**: Remove `window.YTD.*.part0 = [` prefix
- **Field Naming**: Use `#[serde(rename = "camelCaseName")]` for Twitter fields
- **Optional Fields**: Handle with `Option<T>` types
- **Streaming**: Use `serde_json::Deserializer` for large files

### 🔐 Privacy Features
- **Blake3 Anonymization**: Consistent, secure user ID hashing
- **Local Processing**: No data leaves user's machine
- **Hash-Based Filenames**: No personal identifiers in file system
- **Content Masking**: Optional sensitive content protection

### 🚀 Output Generation
```
output_[user]_[timestamp]/
├── threads_[user]_[timestamp].csv          # Structured tweet data
├── threads_[user]_[timestamp].txt          # Human-readable threads
├── dm_conversations_[user]_[timestamp].csv  # DM conversation metadata
├── dm_threads_[user]_[timestamp].csv       # DM conversation threads
├── dm_threads_[user]_[timestamp].txt       # Human-readable DM threads
└── relationship_profiles_[user]_[timestamp]/
    ├── user_[hash]_profile.txt             # Individual relationship profiles
    ├── interaction_timeline.txt            # Chronological interaction log
    ├── communication_patterns.txt          # Behavioral pattern analysis
    ├── relationship_network.txt            # Network topology analysis
    └── llm_analysis_prompts.txt            # LLM-ready analysis questions
```

### 🎯 Current Status - COMPLETE
✅ **All Core Features Implemented**:
- Enhanced thread reconstruction (ALL replies treated as threads)
- DM thread conversion with timing analysis
- Relationship intelligence with LLM-ready outputs
- File splitter utility for large archive management
- Simple CLI interface with automatic file detection

✅ **Quality Assurance Complete**:
- 118+ tests passing across all modules
- Real data testing with 55K+ tweets and 3.6K DM conversations
- Privacy protection with Blake3 anonymization
- Performance optimization with async I/O and memory management

✅ **Production Ready**:
- Comprehensive documentation in README
- Privacy protection with git hooks and safety checks
- File size compliance (all files under 600 lines)
- Idiomatic Rust patterns throughout codebase

### 🚀 Ready for Use
The project is complete and ready for production use. All requested features have been implemented, tested, and integrated.