//! Relationship intelligence analysis for Tweet-Scrolls
//! 
//! This module provides comprehensive relationship analysis capabilities:
//! - User anonymization and profiling
//! - Communication pattern analysis
//! - Timeline analysis integration
//! - Network topology mapping

pub mod analyzer;
pub mod anonymization;
pub mod communication;
pub mod timeline_integration;
pub mod file_generation;

// Re-export commonly used types
pub use analyzer::RelationshipAnalyzer;
pub use anonymization::hash_user_id;
pub use communication::{CommunicationFrequency, calculate_response_times, calculate_average_response_time};
pub use timeline_integration::{analyze_hourly_activity, find_most_active_day};
pub use file_generation::{LLMFileGenerator, generate_profile_text, generate_timeline_text, generate_llm_analysis_prompts};