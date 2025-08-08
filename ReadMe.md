# Tweet-Scrolls 📜
*Transform Twitter archives into organized conversation intelligence*

**Tweet-Scrolls processes your Twitter archive files and generates structured conversation data with relationship analysis.** Like the Marauder's Map, it reveals hidden patterns in your digital interactions.

## Input → Output

```mermaid
flowchart TD
    subgraph input ["📥 What You Provide"]
        A[📂 Twitter Archive]
        A1[📄 tweets.js]
        A2[💬 direct-messages.js]
        A3[📋 headers.js]
    end
    
    input --> process
    
    subgraph process ["⚡ Tweet-Scrolls"]
        P[🔄 Process & Analyze]
    end
    
    process --> output
    
    subgraph output ["📤 What You Get"]
        B[📊 Structured Data]
        B1[📝 Human Readable] 
        B2[🤖 LLM Analysis]
    end
    
    output --> details
    
    subgraph details ["📋 File Details"]
        B3[threads_*.csv<br/>dm_threads_*.csv<br/>timeline_*.csv]
        B4[threads_*.txt<br/>profiles_*/<br/>analysis_*.txt]
        B5[llm_prompts.txt<br/>relationship_maps<br/>behavioral_patterns]
    end
    
    style input fill:#e8f4fd
    style process fill:#fff8e1
    style output fill:#f1f8e9
    style details fill:#fdf2f8
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

## User Journey

### 🏗️ How It Works: From Raw Data to LLM-Ready Gold

```mermaid
flowchart TD
    A1["🔍 Discovery<br/>📂 Auto-detect files<br/>📁 Setup directories"]
    A2["🧵 Thread Building<br/>💬 Connect replies<br/>🔗 Build conversations"]
    A3["💬 DM Organization<br/>⏰ Add timestamps<br/>👥 A/B participants"]
    A4["🔐 Anonymization<br/>🔒 Blake3 hashing<br/>🛡️ Protect identity"]
    A5["📊 Data Generation<br/>📈 CSV files<br/>📝 Human-readable"]
    A6["🤖 LLM Preparation<br/>🎯 Analysis prompts<br/>🔮 Intelligence ready"]
    
    A1 --> A2
    A2 --> A3
    A3 --> A4
    A4 --> A5
    A5 --> A6
    
    style A1 fill:#e8f5e8
    style A2 fill:#e8f5e8  
    style A3 fill:#fff3e0
    style A4 fill:#fff3e0
    style A5 fill:#f3e5f5
    style A6 fill:#f3e5f5
```

**The Magic**: Like a digital archaeologist, Tweet-Scrolls discovers your Twitter archive files, intelligently reconstructs conversation threads, and transforms them into LLM-ready insights - all while keeping your data safe and local.

### Thread Compilation Example

Like transforming scattered pages into a coherent storybook, Tweet-Scrolls compiles individual JSON messages into cohesive conversation threads that are easily digestible by Large Language Models.

```mermaid
flowchart TD
    subgraph Input ["📄 Raw JSON Messages"]
        A1["msg1: 'Hello!'<br/>sender: A, id: 1"]
        A2["msg2: 'Hi there!'<br/>sender: B, id: 2"]
        A3["msg3: 'How are you?'<br/>sender: A, id: 3"]
    end
    
    Input --> Processing
    
    subgraph Processing ["🧠 Transformation Engine"]
        B1["🔍 Parse Content<br/>Extract text & metadata"]
        B2["⏰ Add Timestamps<br/>Calculate relative timing"]
        B3["🧵 Thread Assembly<br/>Order chronologically"]
        B4["🔐 Anonymization<br/>Hash user identifiers"]
    end
    
    Processing --> Output
    
    subgraph Output ["💬 LLM-Ready Thread"]
        C1["A: Hello!<br/>(5 minutes later)<br/>B: Hi there!<br/>(5 minutes later)<br/>A: How are you?"]
    end
    
    Output --> Metadata
    
    subgraph Metadata ["📊 Metadata"]
        C2["• 3 messages<br/>• 10 min duration<br/>• A ↔ B participants<br/>• Blake3 anonymized"]
    end
    
    style Input fill:#ffe0e0
    style Processing fill:#fff3e0
    style Output fill:#e8f5e8
    style Metadata fill:#f0f9ff
```

**The Transformation**: Individual JSON objects become natural conversation flow with timing context and participant anonymization - perfect for LLM analysis and pattern recognition.

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

```mermaid
graph TD
    subgraph CLI ["🖥️ CLI Layer"]
        A1["main.rs<br/>Entry point<br/>User interaction"]
        A2["cli.rs<br/>Command line interface<br/>Argument parsing<br/>Interactive mode"]
    end
    
    subgraph Processing ["⚙️ Processing Layer"]
        B1["data_structures.rs<br/>Core data structures"]
        B2["file_io.rs<br/>File input/output"]
        B3["tweets.rs<br/>Tweet parsing"]
        B4["direct_messages.rs<br/>DM parsing"]
        B5["reply_threads.rs<br/>Thread reconstruction"]
        B6["dm_threads.rs<br/>DM threading"]
    end
    
    subgraph Analysis ["🔍 Analysis Layer"]
        C1["analyzer.rs<br/>Core analysis engine"]
        C2["timeline_analyzer.rs<br/>Timeline patterns"]
        C3["relationship/analyzer.rs<br/>Relationship intelligence"]
        C4["anonymization.rs<br/>Privacy protection"]
    end
    
    subgraph Output ["📤 Output Layer"]
        D1["file_generation.rs<br/>File orchestration"]
        D2["text_generators.rs<br/>Human-readable text"]
        D3["prompts_generator.rs<br/>LLM analysis prompts"]
        D4["enhanced_csv_writer.rs<br/>CSV output"]
    end
    
    subgraph Models ["📦 Data Models"]
        E1["direct_message.rs<br/>DM structures"]
        E2["profile.rs<br/>User profiles"]
        E3["statistics.rs<br/>Statistical data"]
        E4["timeline.rs<br/>Timeline structures"]
    end
    
    CLI --> Processing
    Processing --> Analysis
    Analysis --> Output
    Models -.-> Processing
    Models -.-> Analysis
    Models -.-> Output
    
    style CLI fill:#e3f2fd
    style Processing fill:#fff3e0
    style Analysis fill:#f3e5f5
    style Output fill:#e8f5e8
    style Models fill:#fce4ec
```

## LLM Assimilation Journey

*"Like the Sorting Hat understanding a student's mind..."*

```mermaid
flowchart TD
    subgraph Files ["📊 Generated Data Files"]
        A1["threads_*.csv<br/>Tweet conversations"]
        A2["dm_threads_*.csv<br/>DM conversations"]
        A3["timeline_analysis_*.csv<br/>Activity patterns"]
        A4["*.txt files<br/>Human-readable formats"]
    end
    
    Files --> Questions
    
    subgraph Questions ["🧠 Ready-Made Analysis Questions"]
        B1["'Who do I interact with most?'"]
        B2["'When am I most active?'"]
        B3["'How has my style evolved?'"]
        B4["'What are my patterns?'"]
    end
    
    Questions --> Intelligence
    
    subgraph Intelligence ["🎯 LLM-Ready Intelligence"]
        C1["📈 Relationship Analysis<br/>• Interaction frequency<br/>• Communication patterns"]
        C2["⏰ Timeline Patterns<br/>• Peak activity hours<br/>• Response timing"]
    end
    
    Intelligence --> Privacy
    
    subgraph Privacy ["🔐 Privacy Protected"]
        D1["🔒 Blake3 anonymization<br/>🛡️ Local processing<br/>🚫 No network calls"]
    end
    
    style Files fill:#e3f2fd
    style Questions fill:#fff3e0
    style Intelligence fill:#e8f5e8
    style Privacy fill:#fdf2f8
```

**The Result**: Your digital conversations become structured intelligence that LLMs can analyze for relationship patterns, behavioral insights, and communication evolution - all with privacy-first anonymization.

---

*Like the Marauder's Map, Tweet-Scrolls reveals the hidden patterns in your digital world.*