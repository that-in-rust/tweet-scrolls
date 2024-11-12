# Tweet-Scrolls: Twitter Thread Archiver

## Introduction
Tweet-Scrolls is a Rust-based tool designed to process and organize Twitter JSON data files into coherent threads. It efficiently extracts, filters, and sorts tweets, providing a comprehensive view of Twitter conversations.

## Features
- Extracts and organizes tweet threads from JSON files.
- Filters out retweets and irrelevant replies.
- Sorts threads in reverse chronological order.
- Outputs data in CSV and TXT formats within a dedicated output directory.
- Provides a detailed operation summary.

## Installation
To build and run Tweet-Scrolls, ensure you have Rust and Cargo installed. You can install Rust using [rustup](https://rustup.rs/).

## Output
- All output files are stored in a directory named `output_<handle>_<timestamp>` located in the same directory as the input file.
- `threads_<handle>_<timestamp>.csv`: CSV file with thread metrics and full text.
- `threads_<handle>_<timestamp>.txt`: TXT file with formatted thread content.
- `results_<handle>_<timestamp>.txt`: TXT file with operation summary and stats.

## Technologies Used
- Rust
- Tokio for asynchronous I/O
- Serde for JSON parsing
- CSV for data output
- Mimalloc for memory efficiency

## Contributing
Contributions are welcome! Please fork the repository and submit a pull request.

1. Fork the project.
2. Create a new branch: `git checkout -b feature-branch`.
3. Make your changes.
4. Commit your changes: `git commit -m 'Add some feature'`.
5. Push to the branch: `git push origin feature-branch`.
6. Open a pull request.

## License
WIP

## Acknowledgments
Special thanks to the Rust community for their invaluable resources and support.
