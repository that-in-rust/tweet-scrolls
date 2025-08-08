# Product Overview

Tweet-Scrolls is a Rust CLI tool that processes Twitter JSON archive files to extract and organize tweet threads into readable formats with advanced conversation intelligence.

## Core Purpose
- Transforms chaotic Twitter JSON archives into organized conversation threads
- Filters out retweets and noise, keeping only meaningful discourse
- Provides both CSV data files and human-readable text outputs
- Focuses on thread reconstruction and conversation flow analysis
- Generates relationship intelligence with LLM-ready insights

## Key Features

### **Enhanced Thread Processing**
- **Thread Weaving**: Reconstructs ALL reply chains into complete conversations (not just self-replies)
- **DM Thread Conversion**: Transforms private message threads into readable conversation flows
- **Relationship Intelligence**: Maps social connections with interaction analysis
- **File Splitter Utility**: Splits large archives into manageable 1MB chunks

### **Advanced DM Thread Format (v2.0)**
- **Relative Timestamps**: Shows "2 hours later", "3 days later" instead of absolute times
- **Conversation-Focused Layout**: A/B participant labels instead of hash fragments
- **Reduced Metadata Density**: 62% size reduction while maintaining all information
- **Natural Flow**: Conversations read like chat logs with meaningful timing context
- **Smart Timing**: Only shows gaps >5 minutes to reduce noise

### **Temporal Analysis**
- Active hours detection (identifies peak interaction times)
- Weekly patterns (most active days of the week)
- Bursty activity detection (periods of high activity)
- Response time patterns and conversation rhythm analysis

### **Conversation Metrics**
- Response time statistics (average, median, percentiles)
- Interaction density analysis
- Participant engagement levels
- Relationship strength mapping

### **Multi-Format Output**
- **CSV**: Structured data for analysis and processing
- **TXT**: Human-readable reports optimized for conversation flow
- **LLM-Ready**: Formatted for AI analysis and relationship insights
- **Chunked Files**: Large outputs automatically split into 1MB parts

### **Performance & Privacy**
- **Async Processing**: Buffered I/O with streaming for large files
- **Memory Efficient**: Handles 50K+ tweets and 3K+ DM conversations
- **Local Processing**: 100% local, no data leaves your machine
- **Blake3 Anonymization**: User IDs hashed for privacy protection

## Target Users
- Twitter power users wanting to organize their archives
- Researchers analyzing Twitter conversation patterns
- Content creators reviewing their thread performance
- Anyone needing structured data from Twitter JSON exports

## Unique Characteristics
- Harry Potter themed UI/UX in documentation
- Marvel Avengers themed progress messages during processing
- Whimsical but professional tone throughout
- Focus on conversation threads rather than individual tweets