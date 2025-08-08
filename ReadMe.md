# Tweet-Scrolls 🧙‍♂️📜
### *"The Archive Reveals Its Secrets"* - A Magical Twitter Intelligence Tool

> *"It is our choices that show what we truly are, far more than our abilities."* - Albus Dumbledore

**Tweet-Scrolls transforms your Twitter archive into actionable relationship intelligence using the power of Rust magic.** Like the Marauder's Map revealing hidden passages, this enchanted tool unveils the secret patterns in your digital conversations, creating comprehensive reports that would make even Hermione proud.

## ⚡ The Magic at a Glance

**One simple incantation processes your entire Twitter archive:**
```bash
./target/release/tweet-scrolls /path/to/your/twitter/archive
```

**What emerges from the magical processing:**
- 📊 **Relationship Intelligence** - Your digital social network mapped like the Great Hall's house tables
- 🧵 **Thread Reconstruction** - Every conversation thread woven together like Hermione's Time-Turner timeline
- 💬 **DM Conversations** - Private messages organized as readable scrolls
- 📈 **Activity Patterns** - When your digital quill is most active
- 🔮 **LLM-Ready Insights** - Structured data perfect for AI divination

## 🏰 Proven Results from the Digital Hogwarts Archives

*Real processing results from actual Twitter data:*
- **55,598** tweets analyzed *(more than pages in the Hogwarts library)*
- **22,004** meaningful conversations extracted
- **3,599** private message threads decoded
- **2,469** unique magical connections mapped
- **210,974** total interactions catalogued

### 📜 Sample Prophecy Output:
```
🎯 YOUR DIGITAL SOCIAL NETWORK REVEALED
=======================================

👥 YOUR CLOSEST DIGITAL COMPANIONS
----------------------------------
1. @user_1132151165410455552 - 74,899 interactions (like Ron's loyalty)
2. @amuldotexe - 6,435 interactions (like Hermione's wisdom)
3. @user_1446445479068241923 - 5,514 interactions (like Harry's courage)

⏰ WHEN YOUR DIGITAL QUILL IS MOST ACTIVE
-----------------------------------------
Peak Magical Hours:
  5:00 PM - 18,201 activities (Evening study sessions)
  4:00 PM - 16,819 activities (After-class discussions)
  
Most Productive Days:
  Tuesday - 40,132 activities (Transfiguration day energy)
  Friday - 39,883 activities (Weekend anticipation magic)
```

## 🪄 Summoning the Magic (Installation)

### 📚 Required Spellbooks
- **Rust 1.70+** *(Your magical compiler - get it from [rustup.rs](https://rustup.rs))*
- **Your Twitter Archive** *(Download from Twitter/X settings - your personal Pensieve)*

### ⚡ Quick Incantation
```bash
# Summon the repository to your magical workspace
git clone https://github.com/yourusername/tweet-scrolls.git
cd tweet-scrolls

# Forge the magical artifact
cargo build --release

# Cast the primary spell - point your wand at your Twitter archive
./target/release/tweet-scrolls /path/to/your/twitter/archive

# Advanced spell with custom destination
./target/release/tweet-scrolls /path/to/archive /path/to/output
```

## 🎭 Casting the Spell (Usage)

### 🪄 The Simple Incantation (Recommended)
*Like saying "Alohomora" - one word unlocks everything:*

```bash
# Point your wand at your Twitter archive folder
./target/release/tweet-scrolls /path/to/your/twitter/archive

# The magic automatically finds and processes:
# - tweets.js (your public thoughts)
# - direct-messages.js (your private conversations)  
# - direct-message-headers.js (conversation metadata)
# - Creates a timestamped Room of Requirement for results
```

### 🔮 Advanced Divination
```bash
# Direct the magic to a specific location
./target/release/tweet-scrolls /path/to/archive /path/to/custom/output

# Interactive mode (for those who prefer the Sorting Hat approach)
./target/release/tweet-scrolls
```

### 🏰 The Magic Happens Automatically
*No need for complex potions or lengthy incantations:*
- ✨ **Auto-Detection** - Finds all archive scrolls like the Marauder's Map
- 🧵 **Thread Weaving** - Connects every reply like Hermione's logic puzzles
- 💬 **Message Decoding** - Organizes private conversations like Tom Riddle's diary
- 📊 **Intelligence Gathering** - Maps relationships like the Order of the Phoenix
- 📁 **Secure Storage** - Creates protected results like Gringotts vaults
- 🚫 **No Interruptions** - Works silently like a House Elf

## 📜 The Magical Scrolls Created

*Your enchanted results appear in a timestamped Room of Requirement:*

| Magical Scroll | What Secrets It Contains | Format |
|----------------|--------------------------|--------|
| `threads_*.csv` | Tweet conversations mapped like the Marauder's Map | CSV |
| `threads_*.txt` | Human-readable threads like Daily Prophet articles | Text |
| `dm_conversations_*.csv` | Private message metadata like Hogwarts letters | CSV |
| `dm_threads_*.csv` | DM conversations structured like Tom Riddle's diary | CSV |
| `dm_threads_*.txt` | Readable private conversations like personal journals | Text |
| `timeline_analysis_*.csv` | Activity patterns like Professor Trelawney's charts | CSV |
| `timeline_analysis_*.txt` | Behavioral insights like Hermione's research notes | Text |
| `dm_results_*.txt` | Private message summary like Dumbledore's pensieve | Text |
| `results_*.txt` | Complete analysis like the Sorting Hat's wisdom | Text |

## 🛡️ Magical Protection & Privacy

*Your secrets are guarded like the Chamber of Secrets:*

- **🏰 Fortress-Level Security** - All magic happens within your castle walls (100% local processing)
- **🎭 Identity Protection** - Blake3 anonymization charms protect sensitive names
- **📚 Automatic Concealment** - Your data scrolls are hidden from prying eyes (gitignore protected)
- **🚫 No Dark Arts** - Zero external connections, no network spells or API summons

## 🏗️ The Magical Architecture

*Built with Rust's protective enchantments for performance and safety:*

### ⚡ Core Magical Properties
- **🔄 Parallel Processing** - Utilizes all magical cores like multiple House Elves working together
- **💾 Memory Efficiency** - Streams data like the Floo Network instead of Apparating everything at once
- **🛡️ Type Safety** - Rust's protective spells prevent common magical accidents
- **🏛️ Modular Design** - Clean separation like the four Hogwarts houses

### 🏰 The Four Houses of Code
- **`models/`** - *Ravenclaw House* - Data structures for tweets, DMs, and analysis
- **`processing/`** - *Hufflepuff House* - JSON parsing and data transformation (the hard work)
- **`relationship/`** - *Gryffindor House* - Intelligence extraction and brave report generation
- **`services/`** - *Slytherin House* - Timeline analysis and cunning pattern detection

## 🧪 Magical Experiments (Development)

### 🔬 Testing Your Potions
```bash
# Test all magical formulas
cargo test

# Test with real magical ingredients (if available)
./test_real_data.sh
```

### ✨ Code Quality Standards
*Maintained to Hermione's exacting standards:*
- All scrolls under 600 lines *(max: 494 - even shorter than O.W.L.S. essays)*
- Comprehensive test coverage *(more thorough than N.E.W.T. exams)*
- Documented public APIs *(clearer than Lockhart's books)*
- Idiomatic Rust patterns *(more elegant than Snape's potions)*

## 📈 Performance Magic

*Optimized for the largest magical archives:*
- Processes 50,000+ tweets faster than a Golden Snitch
- Handles millions of DMs more efficiently than Howler deliveries
- Lower memory footprint than a Remembrall
- Parallel thread reconstruction like multiple Pensieves working together

## 🤝 Join the Order of the Phoenix (Contributing)

*Help us fight the forces of disorganized data!*

1. **Fork the Repository** *(Create your own Horcrux)*
2. **Create a Feature Branch** *(Start your own magical research)*
3. **Add Tests** *(Prove your spells work reliably)*
4. **Ensure All Tests Pass** *(No broken wands allowed)*
5. **Submit a Pull Request** *(Present your work to the Order)*

## 📜 The Magical License

MIT License - *As free as a House Elf after receiving clothes* - see LICENSE file for details

## 🙏 Acknowledgments to Fellow Wizards

*Built with magical components from:*
- [Serde](https://serde.rs/) - *For JSON transfiguration spells*
- [Chrono](https://github.com/chronotope/chrono) - *For time-turner date handling*
- [Rayon](https://github.com/rayon-rs/rayon) - *For parallel processing like multiple wands*
- [Blake3](https://github.com/BLAKE3-team/BLAKE3) - *For protective anonymization charms*

---

## 🚨 Critical Privacy Protection

**⚠️ Your Data Security is Paramount**: 

- **🏰 Local Processing Only** - All magic happens on your machine, nothing leaves your castle
- **🛡️ Git Protection** - Your `REALDATA/` folder is automatically protected from accidental commits
- **🔍 Safety Checks** - Run `./check_data_safety.sh` before any git operations
- **🚫 Never Share** - Your Twitter archive contains more secrets than the Chamber of Secrets

### 🛠️ Built-in Safety Features
```bash
# Automatic safety check before commits
git commit  # Will block if private data detected

# Manual safety verification
./check_data_safety.sh

# Your REALDATA folder is protected by:
# - .gitignore entries
# - Pre-commit hooks  
# - Comprehensive file pattern matching
```

**⚠️ Important Magical Safety Notice**: Always download your Twitter archive from official Twitter/X settings (like getting your wand from Ollivanders). Never share your archive files publicly - they contain more secrets than Tom Riddle's diary.

*"After all this time?"* - *"Always."* 🦌✨