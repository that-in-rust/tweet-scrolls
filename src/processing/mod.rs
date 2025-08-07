//! Core processing modules for Tweet-Scrolls
//! 
//! This module contains the main processing logic split into focused components:
//! - Tweet processing pipeline
//! - DM processing pipeline  
//! - File I/O operations
//! - Data structures

pub mod tweets;
pub mod direct_messages;
pub mod file_io;
pub mod data_structures;
pub mod mvp_analyzer;

// Re-export commonly used types
pub use data_structures::{Tweet, TweetWrapper, Thread, ProcessedConversation, CsvWriter};
pub use tweets::{process_tweets, process_tweets_simple};
pub use direct_messages::{process_dm_file, process_dm_conversations};
pub use file_io::{write_threads_to_file, write_csv, get_input_file, get_screen_name, get_dm_file};
pub use mvp_analyzer::{MvpAnalyzer, SimpleRelationship, ActivityPattern};