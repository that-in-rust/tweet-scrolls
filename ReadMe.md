# Tweet-Scrolls: Twitter Archive JSON to CSV / TXT

## Introduction
Tweet-Scrolls is a Rust tool that processes Twitter JSON data into organized threads, with an Avengers-themed interface. It uses asynchronous operations and concurrent processing to efficiently extract, filter, and sort tweets into meaningful conversation threads.

## Features

- **Thread Organization:** Groups tweets into conversation threads based on reply chains
- **Filtering:** Removes retweets and keeps only relevant replies
- **Chronological Sorting:** Orders threads by newest first using the Time Stone

## Installation
Requires Rust and Cargo. Install via [rustup](https://rustup.rs/).

## Usage
Run the program with `cargo run`
