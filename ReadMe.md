# Tweet-Scrolls 📜
*Transform Twitter archives into organized conversation intelligence*

**Tweet-Scrolls processes your Twitter archive files and generates structured conversation data with relationship analysis.** Like the Marauder's Map, it reveals hidden patterns in your digital interactions.

## Input → Output

### What You Provide
```
your-twitter-archive/
├── tweets.js                    # Your tweet history
├── direct-messages.js           # Private conversations  
└── direct-message-headers.js    # Conversation metadata
```

### What You Get
```
output_username_timestamp/
├── threads_*.csv               # Tweet conversations (structured data)
├── threads_*.txt               # Tweet conversations (readable format)
├── dm_threads_*.csv           # DM conversations (structured data)
├── dm_threads_*.txt           # DM conversations (readable format)
├── timeline_analysis_*.csv    # Activity patterns (structured data)
├── timeline_analysis_*.txt    # Activity patterns (readable format)
└── relationship_profiles_*/   # Individual relationship analysis
    ├── user_*_profile.txt     # Per-person interaction history
    ├── interaction_timeline.txt # Chronological activity log
    └── llm_analysis_prompts.txt # AI-ready analysis questions
```

### Key Capabilities
- **Thread Reconstruction**: Connects all replies into complete conversations
- **DM Organization**: Converts message threads into readable conversation flows
- **Relationship Mapping**: Identifies your most frequent interaction partners
- **Timeline Analysis**: Shows when you're most active and interaction patterns
- **Privacy Protection**: All processing happens locally, user IDs are anonymized

## Installation & Usage

### Requirements
- Rust 1.70+ ([install here](https://rustup.rs))
- Your Twitter archive (download from Twitter/X settings)

### Quick Start
```bash
git clone https://github.com/yourusername/tweet-scrolls.git
cd tweet-scrolls
cargo build --release

# Process your archive
./target/release/tweet-scrolls /path/to/your/twitter/archive
```

### Usage Options
```bash
# Basic usage (recommended)
./target/release/tweet-scrolls /path/to/archive

# Custom output location  
./target/release/tweet-scrolls /path/to/archive /path/to/output

# Interactive mode
./target/release/tweet-scrolls
```

## File Details

| File | Content | Purpose |
|------|---------|---------|
| `threads_*.csv` | Tweet conversations with metadata | Data analysis |
| `threads_*.txt` | Human-readable tweet threads | Review conversations |
| `dm_threads_*.csv` | DM conversations with timing | Data analysis |
| `dm_threads_*.txt` | Human-readable DM threads | Review private messages |
| `timeline_analysis_*.csv` | Activity patterns and statistics | Behavioral analysis |
| `timeline_analysis_*.txt` | Activity insights and summaries | Understanding patterns |
| `relationship_profiles_*/` | Individual relationship analysis | AI-ready insights |

## Privacy & Security

**All processing happens locally** - your data never leaves your machine. User IDs are anonymized using Blake3 hashing for privacy protection.

### Built-in Safety Features
- Local processing only (no network connections)
- Automatic git protection for private data
- Blake3 anonymization for user identifiers
- Comprehensive .gitignore protection

```bash
# Safety check before commits
./check_data_safety.sh
```

## Performance

- Processes 50,000+ tweets efficiently
- Handles large DM archives with streaming
- Parallel processing for optimal speed
- Memory-efficient design

## Development

```bash
# Run tests
cargo test

# Check code quality
cargo clippy
```

### Architecture
- `models/` - Data structures for tweets, DMs, and analysis
- `processing/` - JSON parsing and data transformation  
- `relationship/` - Intelligence extraction and report generation
- `services/` - Timeline analysis and pattern detection

## File Splitter Utility

Split large archive files into manageable chunks:

```bash
cargo build --release --bin file-splitter
./target/release/file-splitter large_archive.js

# Custom options
./target/release/file-splitter -i tweets.js -s 5M -o chunks/
```

## License

MIT License

---

*Like the Marauder's Map, Tweet-Scrolls reveals the hidden patterns in your digital world.*