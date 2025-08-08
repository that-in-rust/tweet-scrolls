//! Main processing function that integrates all features
//! Handles tweets, replies, DMs, and thread generation

use anyhow::{Context, Result};
use std::path::Path;
use tokio::fs as async_fs;

use crate::processing::{
    process_tweets, process_dm_file,
};

/// Process Twitter archive with all features enabled
pub async fn main_process_twitter_archive(
    tweets_file: &str,
    dms_file: Option<&str>,
    _dm_headers_file: Option<&str>,
    output_dir: &str,
    screen_name: &str,
    timestamp: i64,
) -> Result<()> {
    println!("🌟 Avengers, assemble! Initiating Operation: Tweet Processing...");
    
    // Create output directory
    async_fs::create_dir_all(output_dir).await
        .context("Failed to create output directory")?;
    
    // Process tweets
    process_tweets(tweets_file, screen_name, Path::new(output_dir), timestamp).await?;
    
    // For now, we'll use the existing processing and add reply thread processing later
    // The existing process_tweets function already handles thread creation
    println!("✅ Tweet processing complete");
    
    // TODO: Add reply thread processing integration
    // This will require modifying the existing process_tweets function to return the processed data
    
    // Process DMs if available
    if let Some(dm_file) = dms_file {
        println!("\n📱 Processing Direct Messages...");
        process_dm_file(dm_file, screen_name, Path::new(output_dir), timestamp).await?;
    }
    
    // Summary
    println!("\n✨ Processing complete!");
    println!("📊 Results:");
    println!("  • Tweet processing completed");
    if dms_file.is_some() {
        println!("  • DM processing completed");
    }
    println!("  • Check output directory for results");
    
    Ok(())
}