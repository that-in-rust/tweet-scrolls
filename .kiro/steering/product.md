# Product Overview

Tweet-Scrolls is a Rust CLI tool that processes Twitter JSON archive files to extract and organize tweet threads into readable formats.

## Core Purpose
- Transforms chaotic Twitter JSON archives into organized conversation threads
- Filters out retweets and noise, keeping only meaningful discourse
- Provides both CSV data files and human-readable text outputs
- Focuses on thread reconstruction and conversation flow analysis

## Key Features
- **Thread Weaving**: Reconstructs reply chains into complete conversations
- **Temporal Analysis**: Identifies patterns in interaction timing
  - Active hours detection (identifies peak interaction times)
  - Weekly patterns (most active days of the week)
  - Bursty activity detection (periods of high activity)
- **Conversation Metrics**:
  - Response time statistics (average, median, percentiles)
  - Interaction density analysis
  - Participant engagement levels
- **Dual Output Formats**: 
  - CSV for data analysis
  - TXT for human-readable reports
- **Performance Optimized**: 
  - Async processing with buffered I/O
  - Efficient memory usage with streaming where possible

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