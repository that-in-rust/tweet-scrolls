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

## User Journey


### 🏗️ How It Works: From Raw Data to LLM-Ready Gold


```markdown
Step 1: Archive Discovery          Step 2: Intelligent Processing       Step 3: Intelligence Generation
┌─────────────────────┐           ┌─────────────────────────┐           ┌─────────────────────────┐
│                     │           │                         │           │                         │
│ 🔍 Auto-Detection   │           │ 🧠 Thread Reconstruction│           │ 📊 CSV Data Files       │
│   • tweets.js       │ ────────▶ │   • Connect all replies │ ────────▶ │   • Structured data     │
│   • direct-msgs.js  │           │   • Build conversations │           │   • Analysis ready      │
│   • headers.js      │           │                         │           │                         │
│                     │           │ 💬 DM Enhancement       │           │ 📝 Human-Readable       │
│ 📁 Output Setup     │           │   • Relative timestamps │           │   • Natural flow        │
│   • Timestamped dir │           │   • Smart timing        │           │   • Conversation style  │
│   • Safe naming     │           │   • A/B participants    │           │                         │
│                     │           │                         │           │ 🔮 LLM-Ready Insights  │
│ 🛡️ Privacy First    │           │ 🔐 Blake3 Anonymization│           │   • Relationship maps   │
│   • Local only      │           │   • Hash user IDs       │           │   • Behavioral patterns │
│   • No network      │           │   • Protect identity    │           │   • AI analysis prompts │
└─────────────────────┘           └─────────────────────────┘           └─────────────────────────┘

### Thread Compilation Example

Like transforming scattered pages into a coherent storybook, Tweet-Scrolls compiles individual JSON messages
into cohesive conversation threads that are easily digestible by Large Language Models.

```
Raw JSON Messages (Individual DMs)         🔄 Transformation Process           📚 LLM-Ready Thread
┌─────────────────────────────────┐      ┌─────────────────────────┐        ┌─────────────────────────┐
│ {                               │      │                         │        │  💬 Conversation        │
│   "messageCreate": {            │      │  🔍 Message Parsing     │        │     (3 messages)        │
│     "id": "1",                  │ ────▶│   • Extract content     │ ─────▶ │  ────────────────────── │
│     "senderId": "123",          │      │   • Parse timestamps    │        │  A: Hello!              │
│     "recipientId": "456",        │      │   • Identify participants│        │     (5 minutes later)   │
│     "text": "Hello!",           │      │                         │        │  B: Hi there!           │
│     "createdAt": "..."          │      │  🧵 Thread Construction │        │     (5 minutes later)    │
│   }                             │      │   • Order chronologically│       │  A: How are you?        │
│ }                               │      │   • Add relative timing │        │  ────────────────────── │
│                                 │      │   • Format for readability│       │                         │
│ {                               │      │                         │        │  📊 Metadata:           │
│   "messageCreate": {            │      │  🔐 Privacy Protection  │        │     • 3 messages        │
│     "id": "2",                  │ ────▶│   • Hash user IDs       │ ─────▶ │     • 10 min duration    │
│     "senderId": "456",          │      │   • Remove sensitive data│        │     • A/B participants   │
│     "text": "Hi there!",        │      │                         │        │                         │
│     "createdAt": "..."          │      │  🎯 LLM Optimization    │        │                         │
│   }                             │      │   • Clean formatting    │        │                         │
│ }                               │      │   • Preserve context    │        │                         │
│                                 │      │   • Add timing context  │        │                         │
│ {                               │      │                         │        │                         │
│   "messageCreate": {            │      └─────────────────────────┘        └─────────────────────────┘
│     "id": "3",                  │                                                        │
│     "senderId": "123",          │                                           ┌────────────┴────────────┐
│     "text": "How are you?",     │                                           │                         │
│     "createdAt": "..."          │                                           │  These structured       │
│   }                             │                                           │  conversation threads   │
│ }                               │                                           │  are now ready for      │
└─────────────────────────────────┘                                           │  consumption by LLMs    │
                                                                              │  with preserved context │
                                                                              │  and timing information │
                                                                              └─────────────────────────┘
```

### The Magic Happens in 3 Steps

```
Step 1: Archive Discovery          Step 2: Intelligent Processing       Step 3: Intelligence Generation
┌─────────────────────┐           ┌─────────────────────────┐           ┌─────────────────────────┐
│                     │           │                         │           │                         │
│ 🔍 Auto-Detection   │           │ 🧠 Thread Reconstruction│           │ 📊 CSV Data Files       │
│   • tweets.js       │ ────────▶ │   • Connect all replies │ ────────▶ │   • Structured data     │
│   • direct-msgs.js  │           │   • Build conversations │           │   • Analysis ready      │
│   • headers.js      │           │                         │           │                         │
│                     │           │ 💬 DM Enhancement       │           │ 📝 Human-Readable       │
│ 📁 Output Setup     │           │   • Relative timestamps │           │   • Natural flow        │
│   • Timestamped dir │           │   • Smart timing        │           │   • Conversation style  │
│   • Safe naming     │           │   • A/B participants    │           │                         │
│                     │           │                         │           │ 🔮 LLM-Ready Insights  │
│ 🛡️ Privacy First    │           │ 🔐 Blake3 Anonymization│           │   • Relationship maps   │
│   • Local only      │           │   • Hash user IDs       │           │   • Behavioral patterns │
│   • No network      │           │   • Protect identity    │           │   • AI analysis prompts │
└─────────────────────┘           └─────────────────────────┘           └─────────────────────────┘
```

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

## Architecture

```
Tweet-Scrolls Architecture
==========================

┌─────────────────────────────────────────────────────────────────────────────┐
│                              CLI Layer                                      │
├─────────────────────────────────────────────────────────────────────────────┤
│  src/main.rs                    src/cli.rs                                  │
│  • Entry point               • Command line interface                       │
│  • User interaction          • Argument parsing                             │
│  • Output directory creation • Interactive mode                              │
└─────────────────────────────────────────────────────────────────────────────┘
                │
                ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                           Processing Layer                                  │
├─────────────────────────────────────────────────────────────────────────────┤
│  src/processing/                                                            │
│  • data_structures.rs    - Core data structures                             │
│  • file_io.rs           - File input/output operations                      │
│  • tweets.rs            - Tweet parsing and processing                       │
│  • direct_messages.rs   - DM parsing and processing                          │
│  • reply_threads.rs     - Thread reconstruction logic                        │
│  • dm_threads.rs        - DM conversation threading                          │
│  • dm_headers_analyzer.rs - DM header analysis                               │
│  • mvp_analyzer.rs      - Minimal viable product analysis                    │
└─────────────────────────────────────────────────────────────────────────────┘
                │
                ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                            Analysis Layer                                   │
├─────────────────────────────────────────────────────────────────────────────┤
│  src/services/                                                              │
│  • analyzer.rs          - Core analysis engine                              │
│  • timeline.rs          - Timeline data structures                         │
│  • timeline_analyzer.rs  - Timeline pattern analysis                        │
│                                                                             │
│  src/relationship/                                                         │
│  • analyzer.rs          - Relationship intelligence engine                 │
│  • anonymization.rs     - Privacy protection (Blake3 hashing)               │
│  • communication.rs     - Communication pattern analysis                     │
│  • timeline_integration.rs - Timeline integration with relationships         │
└─────────────────────────────────────────────────────────────────────────────┘
                │
                ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                           Output Layer                                      │
├─────────────────────────────────────────────────────────────────────────────┤
│  src/relationship/                                                          │
│  • file_generation.rs    - File generation orchestration                      │
│  • file_writer.rs       - File writing operations                           │
│  • text_generators.rs   - Human-readable text generation                     │
│  • prompts_generator.rs - LLM analysis prompt generation                     │
│  • timeline_text.rs     - Timeline text formatting                           │
│                                                                             │
│  src/utils/                                                                 │
│  • enhanced_csv_writer.rs - CSV output generation                            │
└─────────────────────────────────────────────────────────────────────────────┘
                │
                ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                           Data Models                                     │
├─────────────────────────────────────────────────────────────────────────────┤
│  src/models/                                                                │
│  • direct_message.rs    - DM data structures                                │
│  • dm_headers.rs        - DM header structures                              │
│  • interaction.rs      - Interaction tracking                               │
│  • profile.rs          - User profile structures                            │
│  • statistics.rs       - Statistical data structures                         │
│  • timeline.rs          - Timeline data structures                          │
│  • tweet_classification.rs - Tweet categorization                            │
└─────────────────────────────────────────────────────────────────────────────┘
```

## LLM Assimilation Journey

*"Like the Sorting Hat understanding a student's mind..."*

```
🧠 LLM Perspective                    📊 Tweet-Scrolls Output                    🎯 Insights Revealed
┌─────────────────────┐              ┌─────────────────────────┐                ┌─────────────────────────┐
│                     │              │                         │                │                         │
│  📄 threads_*.csv   │ ◀─────────────│  🧵 Complete Threads   │ ◀──────────────│  📈 Relationship        │
│  (structured data)  │              │     • All replies       │                │     Analysis            │
│                     │              │     • Metadata rich     │                │     • Interaction maps  │
│  📝 threads_*.txt   │ ◀─────────────│                         │ ◀──────────────│                         │
│  (readable format)  │              │  📝 Readable Threads    │                │  🕐 Timeline Patterns   │
│                     │              │     • Natural flow      │                │     • Peak hours        │
│  💬 dm_threads_*.csv│ ◀─────────────│                         │ ◀──────────────│     • Activity rhythms  │
│  (structured DMs)   │              │  💬 DM Conversations    │                │                         │
│                     │              │     • A/B style         │                │  🔍 Behavioral Insights │
│  💭 dm_threads_*.txt│ ◀─────────────│     • Timing context    │ ◀──────────────│     • Response patterns │
│  (readable DMs)     │              │     • Participant IDs   │                │     • Style evolution   │
└─────────────────────┘              │                         │                │                         │
          │                          │  📋 Profile Analysis    │ ◀──────────────│  🎯 Custom Analysis     │
          │                          │     • User interactions │                │     • AI prompts ready  │
          │                          │     • Communication     │                │     • Context rich      │
          │                          │     • Activity timeline │                │                         │
          │                          └─────────────────────────┘                └─────────────────────────┘
          │                                     │                                           │
          │                          ┌──────────┴────────────┐                              │
          │                          │                       │                              │
          │                          ▼                       ▼                              │
          │                📁 Relationship Profiles     🕐 Timeline Analysis                 │
          │               ┌──────────────────────┐    ┌──────────────────────┐               │
          │               │                      │    │                      │               │
          │               │ user_*_profile.txt   │    │ timeline_analysis_*.│               │
          │               │ interaction_timeline.│    │ csv                  │               │
          │               │ txt                  │    │ timeline_analysis_*.│               │
          │               │ llm_analysis_prompts.│    │ txt                  │               │
          │               │ txt                  │    │                      │               │
          │               │                      │    │                      │               │
          │               └──────────────────────┘    └──────────────────────┘               │
          │                          │                              │                        │
          │                          ▼                              │                        │
          │                🎯 AI Analysis Prompts                   │                        │
          │               ┌─────────────────────────────┐           │                        │
          │               │                             │           │                        │
          │               │  "Who does user interact    │           │                        │
          │               │   with most frequently?"    │           │                        │
          │               │                             │           │                        │
          │               │  "What are user's peak      │           │                        │
          │               │   activity hours?"         │           │                        │
          │               │                             │           │                        │
          │               │  "How does user's           │           │                        │
          │               │   communication style       │           │                        │
          │               │   change over time?"        │           │                        │
          │               │                             │           │                        │
          │               └─────────────────────────────┘           │                        │
          └───────────────────────────────────────────────────────────────────────────────────┘
                              These structured insights and analysis prompts are now ready
                              for consumption by Large Language Models, providing rich context
                              for deeper understanding of digital interaction patterns.
```

---

*Like the Marauder's Map, Tweet-Scrolls reveals the hidden patterns in your digital world.*