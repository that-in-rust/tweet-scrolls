# Tweet-Scrolls: Twitter Thread Archiver

## Introduction
Tweet-Scrolls is a Rust tool that processes Twitter JSON data into organized threads, with an Avengers-themed interface. It uses asynchronous operations and concurrent processing to efficiently extract, filter, and sort tweets into meaningful conversation threads.

## Features
- **Async Processing:** Leveraging Tokio runtime for non-blocking I/O
- **Thread Organization:** Groups tweets into conversation threads based on reply chains
- **Smart Filtering:** Removes retweets and keeps only relevant replies
- **Chronological Sorting:** Orders threads by newest first using the Time Stone
- **Buffered Output:** Concurrent CSV writing with 100-record chunks
- **Memory Optimization:** Uses mimalloc allocator for better performance
- **Fun Interface:** Marvel Avengers themed progress messages and emojis

## Installation
Requires Rust and Cargo. Install via [rustup](https://rustup.rs/).

## Usage
Run the program with `cargo run`
