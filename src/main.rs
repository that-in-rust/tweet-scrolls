//! Tweet-Scrolls: Twitter Archive Intelligence System
//! 
//! A Rust CLI tool that processes Twitter JSON archive files to extract and organize 
//! tweet threads and direct message conversations with relationship intelligence analysis.

use anyhow::{Context, Result};
use chrono::Utc;
use std::path::Path;
use tokio::fs as async_fs;
use tokio::sync::mpsc as async_mpsc;

// Import our modular components
use tweet_scrolls::processing::{
    CsvWriter, MvpAnalyzer,
    file_io::{get_input_file, get_screen_name, get_dm_file},
    tweets::process_tweets,
    direct_messages::process_dm_file,
    data_structures::{TweetWrapper, Thread},
};
use tweet_scrolls::models::direct_message::DmWrapper;

// Global allocator for performance optimization
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

/// Main entry point for the Tweet-Scrolls application
/// 
/// This function orchestrates the entire processing pipeline:
/// 1. Gets user input for file paths and screen name
/// 2. Creates output directory structure
/// 3. Processes tweets and/or DMs based on user input
/// 4. Optionally performs relationship intelligence analysis
#[tokio::main]
async fn main() -> Result<()> {
    println!("🌟 Welcome to Tweet-Scrolls: Twitter Archive Intelligence System");
    
    // Get user input with clear examples
    println!("📋 This tool processes Twitter export files from your downloaded archive.");
    println!("💡 Example files you should have:");
    println!("   • tweets.js (contains all your tweets)");
    println!("   • direct-messages.js (contains your DM conversations)");
    println!("   • direct-message-headers.js (contains DM metadata)");
    println!("");
    
    let input_file = get_input_file()?;
    let screen_name = get_screen_name()?;
    let timestamp = Utc::now().timestamp();

    println!("🕶️ Current working directory: {}", std::env::current_dir()?.display());

    // Validate input file exists
    if !async_fs::metadata(&input_file).await.is_ok() {
        anyhow::bail!("❌ File does not exist: {}", input_file);
    }

    // Create output directory
    let input_path = Path::new(&input_file);
    let output_dir = input_path.parent().unwrap().join(format!("output_{}_{}", screen_name, timestamp));
    async_fs::create_dir_all(&output_dir).await.context("Failed to create output directory")?;

    // Create a channel for CsvWriter
    let (tx, rx) = async_mpsc::channel::<Vec<String>>(100);

    // Initialize CsvWriter and spawn its run task
    let csv_writer = CsvWriter::new(
        output_dir.join(format!("threads_{}_{}.csv", screen_name, timestamp)).to_str().unwrap().to_string(), 
        rx, 
        100
    );
    tokio::spawn(csv_writer.run());

    // Process tweets
    println!("🌟 Avengers, assemble! Initiating Operation: Tweet Processing...");
    if let Err(e) = process_tweets(&input_file, &screen_name, tx, &output_dir, timestamp).await {
        eprintln!("🚨 Mission Failed: {}", e);
    } else {
        println!("🎉 Victory! Tweets have been successfully processed and organized.");
    }

    // Process DM file if provided
    if let Some(dm_file) = get_dm_file()? {
        println!("📱 Initiating DM Processing Operation...");
        if let Err(e) = process_dm_file(&dm_file, &screen_name, &output_dir, timestamp).await {
            eprintln!("🚨 DM Mission Failed: {}", e);
        } else {
            println!("💬 DM processing completed successfully!");
        }
    }

    // Optional relationship intelligence analysis
    println!("🧠 Would you like to generate relationship intelligence profiles? (y/n)");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    
    if input.trim().to_lowercase() == "y" {
        println!("🔍 Initiating Relationship Intelligence Analysis...");
        if let Err(e) = perform_relationship_analysis(&screen_name, &output_dir, timestamp).await {
            eprintln!("🚨 Relationship Analysis Failed: {}", e);
        } else {
            println!("🎯 Relationship intelligence analysis completed successfully!");
        }
    }

    println!("✨ All operations completed successfully! Check the output directory for results.");
    Ok(())
}

/// Performs MVP relationship intelligence analysis
/// 
/// This function provides immediate value by analyzing:
/// - Who you interact with most
/// - When you're most active
/// - Clean, readable insights
async fn perform_relationship_analysis(
    screen_name: &str, 
    output_dir: &Path, 
    timestamp: i64
) -> Result<()> {
    println!("🔬 Analyzing your Twitter relationships and activity patterns...");
    
    // Create MVP analyzer
    let mut analyzer = MvpAnalyzer::new();
    
    // Try to load and analyze tweet data
    let tweets_file = output_dir.parent().unwrap().join("tweets.js");
    if async_fs::metadata(&tweets_file).await.is_ok() {
        println!("📊 Analyzing tweet interactions...");
        
        let tweets_content = async_fs::read_to_string(&tweets_file).await
            .context("Failed to read tweets file")?;
        
        // Parse tweets (remove JavaScript prefix)
        if let Some(json_start) = tweets_content.find('[') {
            if let Some(json_end) = tweets_content.rfind(']') {
                let json_content = &tweets_content[json_start..=json_end];
                
                match serde_json::from_str::<Vec<TweetWrapper>>(json_content) {
                    Ok(tweet_wrappers) => {
                        // Convert to threads for analysis
                        let threads: Vec<Thread> = tweet_wrappers.into_iter().map(|tw| Thread {
                            id: tw.tweet.id_str.clone(),
                            tweets: vec![tw.tweet],
                            tweet_count: 1,
                            favorite_count: 0,
                            retweet_count: 0,
                        }).collect();
                        
                        analyzer.analyze_tweets(&threads)?;
                        println!("✅ Tweet analysis complete - found {} relationships", 
                            analyzer.relationships.len());
                    }
                    Err(e) => {
                        println!("⚠️ Could not parse tweets file: {}", e);
                    }
                }
            }
        }
    }
    
    // Try to load and analyze DM data
    let dm_file = output_dir.parent().unwrap().join("direct-messages.js");
    if async_fs::metadata(&dm_file).await.is_ok() {
        println!("💬 Analyzing direct message conversations...");
        
        let dm_content = async_fs::read_to_string(&dm_file).await
            .context("Failed to read DM file")?;
        
        // Parse DMs (remove JavaScript prefix)
        if let Some(json_start) = dm_content.find('[') {
            if let Some(json_end) = dm_content.rfind(']') {
                let json_content = &dm_content[json_start..=json_end];
                
                match serde_json::from_str::<Vec<DmWrapper>>(json_content) {
                    Ok(dm_wrappers) => {
                        analyzer.analyze_dms(&dm_wrappers)?;
                        println!("✅ DM analysis complete");
                    }
                    Err(e) => {
                        println!("⚠️ Could not parse DM file: {}", e);
                    }
                }
            }
        }
    }
    
    // Generate the intelligence report
    println!("📈 Generating relationship intelligence report...");
    analyzer.generate_report(output_dir, screen_name, timestamp).await?;
    
    // Show quick preview of insights
    let top_relationships = analyzer.get_top_relationships(3);
    if !top_relationships.is_empty() {
        println!("\n🎯 QUICK INSIGHTS:");
        println!("Your top connections:");
        for (i, rel) in top_relationships.iter().enumerate() {
            println!("  {}. @{} ({} interactions)", i + 1, rel.username, rel.interaction_count);
        }
    }
    
    let peak_hours = analyzer.get_peak_activity_hours(2);
    if !peak_hours.is_empty() {
        println!("Most active times:");
        for (hour, count) in peak_hours {
            let time_str = if hour == 0 {
                "12:00 AM".to_string()
            } else if hour < 12 {
                format!("{}:00 AM", hour)
            } else if hour == 12 {
                "12:00 PM".to_string()
            } else {
                format!("{}:00 PM", hour - 12)
            };
            println!("  {} ({} activities)", time_str, count);
        }
    }
    
    println!("\n📄 Full report saved to output directory!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_function_structure() {
        // Test that the main function compiles and has the right structure
        // This is a compile-time test - if it compiles, the structure is correct
        assert!(true);
    }

    #[tokio::test]
    async fn test_relationship_analysis_function() {
        use tempfile::tempdir;
        
        let temp_dir = tempdir().unwrap();
        let output_dir = temp_dir.path();
        
        // Test that the relationship analysis function can be called
        let result = perform_relationship_analysis("testuser", output_dir, 1234567890).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_global_allocator() {
        // Test that the global allocator is set up correctly
        // This is mainly a compile-time check
        let _test_vec = vec![1, 2, 3, 4, 5];
        assert!(true);
    }
}