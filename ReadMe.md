 // Start of Selection
# Tweet-Scrolls: Twitter Thread Archiver

## Introduction
Tweet-Scrolls is a Rust tool that processes Twitter JSON data into organized threads. It uses asynchronous operations to efficiently extract, filter, and sort tweets, providing a clear view of Twitter conversations.

## Features
- **Async Processing:** Leveraging Tokio for efficient, non-blocking I/O.
- **Thread Extraction:** Organizes tweet threads based on conversation flows.
- **Filtering:** Removes retweets and irrelevant replies.
- **Chronological Sorting:** Orders threads newest first.
- **Concurrent CSV Writing:** Buffered channels for efficient data output.
- **Reporting:** Generates operation summaries and thread metrics.
- **Memory Efficiency:** Utilizes Mimalloc for optimized memory usage.

## Installation
Ensure Rust and Cargo are installed. Install Rust via [rustup](https://rustup.rs/).

```bash
cargo build --release
```

## Usage
Run the application and follow the prompts for the JSON file path and Twitter handle.

```bash
cargo run --release
```

## Output
Outputs are stored in `output_<handle>_<timestamp>` in the input file directory.

- `threads_<handle>_<timestamp>.csv`: Thread metrics and full text.
- `threads_<handle>_<timestamp>.txt`: Formatted thread content.
- `results_<handle>_<timestamp>.txt`: Operation statistics and summary.

## Technologies
- **Rust**
- **Tokio**
- **Serde**
- **CSV**
- **Mimalloc**

## Contributing
1. Fork the project.
2. Create a branch: `git checkout -b feature-branch`.
3. Make changes.
4. Commit: `git commit -m 'Add feature'`.
5. Push: `git push origin feature-branch`.
6. Open a pull request.

## License
WIP

## Acknowledgments
Thanks to the Rust community for their support.
