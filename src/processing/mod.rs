//! Core processing modules for Tweet-Scrolls
//! 
//! This module contains the main processing logic split into focused components:
//! - Tweet processing pipeline
//! - DM processing pipeline  
//! - File I/O operations
//! - Data structures
//! - Reply thread processing
//! - DM thread conversion

pub mod tweets;
pub mod direct_messages;
/// Fast DM analysis using only headers data
pub mod dm_headers_analyzer;
pub mod file_io;
pub mod data_structures;
pub mod mvp_analyzer;
pub mod reply_threads;
pub mod dm_threads;

// Re-export commonly used types
pub use data_structures::{Tweet, TweetWrapper, Thread, ProcessedConversation, CsvWriter};
pub use tweets::{process_tweets, process_tweets_simple};
pub use direct_messages::{process_dm_file, process_dm_conversations};
pub use file_io::{write_threads_to_file, write_csv, get_input_file, get_dm_file};
pub use mvp_analyzer::{MvpAnalyzer, SimpleRelationship, ActivityPattern};
pub use reply_threads::{process_reply_threads, format_thread_as_text};
pub use dm_threads::{convert_dms_to_threads, format_dm_thread_as_text, DmThread};