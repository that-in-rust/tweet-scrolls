#!/usr/bin/env cargo
//! Simple test of MVP relationship analysis

use anyhow::Result;
use tweet_scrolls::processing::{MvpAnalyzer, data_structures::{TweetWrapper, Thread}};
use tweet_scrolls::models::direct_message::DmWrapper;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🧪 Testing MVP Relationship Analysis (Simple)");
    
    // Test 1: Load and analyze sample tweets
    println!("\n📋 Test 1: Analyzing sample tweets");
    let tweets_content = std::fs::read_to_string("private_data/sample_tweets_realistic.js")?;
    
    // Remove JavaScript prefix
    let json_start = tweets_content.find('[').expect("Missing opening bracket");
    let json_end = tweets_content.rfind(']').expect("Missing closing bracket");
    let json_content = &tweets_content[json_start..=json_end];
    
    let tweet_wrappers: Vec<TweetWrapper> = serde_json::from_str(json_content)?;
    println!("✅ Loaded {} tweets", tweet_wrappers.len());
    
    // Convert to threads for analysis
    let threads: Vec<Thread> = tweet_wrappers.into_iter().map(|tw| Thread {
        id: tw.tweet.id_str.clone(),
        tweets: vec![tw.tweet],
        tweet_count: 1,
        favorite_count: 0,
        retweet_count: 0,
    }).collect();
    
    // Create analyzer and analyze tweets
    let mut analyzer = MvpAnalyzer::new();
    analyzer.analyze_tweets(&threads)?;
    
    println!("✅ Tweet analysis complete - found {} relationships", 
        analyzer.relationships.len());
    
    // Test 2: Load and analyze sample DMs
    println!("\n📋 Test 2: Analyzing sample DMs");
    let dm_content = std::fs::read_to_string("private_data/sample_direct-messages_realistic.js")?;
    
    // Remove JavaScript prefix
    let json_start = dm_content.find('[').expect("Missing opening bracket");
    let json_end = dm_content.rfind(']').expect("Missing closing bracket");
    let json_content = &dm_content[json_start..=json_end];
    
    let dm_wrappers: Vec<DmWrapper> = serde_json::from_str(json_content)?;
    println!("✅ Loaded {} DM conversations", dm_wrappers.len());
    
    analyzer.analyze_dms(&dm_wrappers)?;
    println!("✅ DM analysis complete");
    
    // Test 3: Generate insights
    println!("\n📋 Test 3: Generating insights");
    
    let top_relationships = analyzer.get_top_relationships(5);
    println!("Top relationships:");
    for (i, rel) in top_relationships.iter().enumerate() {
        println!("  {}. @{} - {} interactions ({})", 
            i + 1, rel.username, rel.interaction_count, rel.interaction_type);
    }
    
    let peak_hours = analyzer.get_peak_activity_hours(3);
    println!("Peak activity hours:");
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
        println!("  {} - {} activities", time_str, count);
    }
    
    let active_days = analyzer.get_most_active_days();
    println!("Most active days:");
    for (day, count) in active_days {
        println!("  {} - {} activities", day, count);
    }
    
    println!("\n🎉 All MVP tests completed successfully!");
    println!("📊 Total relationships found: {}", analyzer.relationships.len());
    println!("⏰ Total activity hours tracked: {}", analyzer.hourly_activity.len());
    println!("📅 Total active days tracked: {}", analyzer.daily_activity.len());
    
    Ok(())
}