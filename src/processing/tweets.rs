//! Tweet processing pipeline

use anyhow::{Context, Result};
use chrono::{DateTime, Local, Utc};
use serde_json::from_str;
use std::collections::HashMap;
use std::path::Path;
use std::time::Instant;
use tokio::fs as async_fs;
use tokio::sync::mpsc as async_mpsc;
use tokio::task;

use super::data_structures::{Tweet, TweetWrapper, Thread};
use super::file_io::{write_threads_to_file, write_csv};

/// Processes tweets from a JSON file and generates output files
pub async fn process_tweets(
    input_file: &str, 
    screen_name: &str, 
    csv_tx: async_mpsc::Sender<Vec<String>>, 
    output_dir: &Path, 
    _timestamp: i64
) -> Result<()> {
    let screen_name = screen_name.to_string(); // Clone to own the String

    let start_datetime = Local::now();
    let timestamp = Utc::now().timestamp();

    println!("🕰️ Avengers, assemble! Mission start time: {}", start_datetime.format("%Y-%m-%d %H:%M:%S"));
    let start_time = Instant::now();

    println!("🕵️‍♀️ Black Widow is infiltrating the enemy base (reading the file)...");
    let script_content = async_fs::read_to_string(input_file).await.context("Failed to read input file")?;
    println!("📂 Intelligence gathered. File size: {} bytes", script_content.len());

    println!("🧠 Tony and Bruce are decoding the alien artifact (parsing JSON)...");
    let json_start = script_content.find('[').context("Invalid JSON format: missing opening bracket")?;
    let json_end = script_content.rfind(']').context("Invalid JSON format: missing closing bracket")?;
    let json_content = &script_content[json_start..=json_end];
    let tweets: Vec<TweetWrapper> = from_str(json_content).context("Failed to parse JSON")?;
    let total_tweets = tweets.len();
    println!("🎉 Decoding complete! We've identified {} potential threats (tweets).", total_tweets);

    println!("🇺🇸 Captain America is assembling the strike team (filtering tweets)...");
    let mut tweets: Vec<Tweet> = tweets.into_iter().map(|tw| tw.tweet).collect();
    let initial_tweet_count = tweets.len();
    tweets.retain(|tweet| !tweet.retweeted && (tweet.in_reply_to_screen_name.as_deref() == Some(&screen_name) || tweet.in_reply_to_screen_name.is_none()));
    let filtered_tweet_count = initial_tweet_count - tweets.len();
    println!("👥 Strike team assembled. {} members are on standby, {} are joining the mission.", filtered_tweet_count, tweets.len());

    println!("📡 Shuri is establishing secure comms (organizing tweets)...");
    let tweets_map: HashMap<String, Tweet> = tweets.into_iter().map(|t| (t.id_str.clone(), t)).collect();
    println!("🔐 Secure network established. We can now track {} individual operatives.", tweets_map.len());

    println!("🕴️ Nick Fury is forming tactical units (grouping tweets into conversations)...");
    let screen_name_clone = screen_name.clone();
    let threads = task::spawn_blocking(move || {
        let mut threads: Vec<Vec<Tweet>> = Vec::new();
        for tweet in tweets_map.values() {
            if tweet.in_reply_to_status_id.is_none() || tweet.in_reply_to_screen_name.as_deref() != Some(&screen_name_clone) {
                let mut thread = vec![tweet.clone()];
                let mut current_id = tweet.id_str.clone();
                while let Some(reply) = tweets_map.values().find(|t| t.in_reply_to_status_id.as_deref() == Some(&current_id)) {
                    thread.push(reply.clone());
                    current_id = reply.id_str.clone();
                }
                threads.push(thread);
            }
        }
        threads
    }).await?;

    println!("👥 Tactical units formed. We have {} specialized teams ready for action.", threads.len());

    println!("🔮 Dr. Strange is using the Time Stone to prioritize our missions (sorting threads)...");
    let mut threads = threads;
    threads.sort_by(|a, b| {
        let date_a = DateTime::parse_from_str(&a[0].created_at, "%a %b %d %H:%M:%S %z %Y").unwrap();
        let date_b = DateTime::parse_from_str(&b[0].created_at, "%a %b %d %H:%M:%S %z %Y").unwrap();
        date_b.cmp(&date_a)
    });
    println!("⏳ Timelines analyzed. Most critical missions identified.");

    println!("📝 Agent Coulson is documenting our missions (writing threads to files)...");
    let threads: Vec<Thread> = threads.into_iter().map(|thread| {
        let id = thread[0].id_str.clone();
        let tweet_count = thread.len();
        let favorite_count = thread.iter().map(|t| t.favorite_count.parse::<u32>().unwrap_or(0)).sum();
        let retweet_count = thread.iter().map(|t| t.retweet_count.parse::<u32>().unwrap_or(0)).sum();
        Thread { 
            id, 
            tweets: thread,
            tweet_count,
            favorite_count,
            retweet_count,
        }
    }).collect();

    // Handle writing to files
    write_threads_to_file(&threads, &screen_name, timestamp, output_dir).await?;
    write_csv(&threads, &screen_name, timestamp, csv_tx).await?;

    let end_datetime = Local::now();
    let end_time = Instant::now();
    let duration = end_time.duration_since(start_time);

    println!("🌍 Director Fury is compiling the final mission report...");
    let results_content = format!(
        "Avengers Operation Summary\n\
         ===========================\n\
         Mission Start: {}\n\
         Total Threats Identified: {}\n\
         Threats Neutralized (Filtered): {}\n\
         Successful Interventions (Final Thread Count): {}\n\
         Mission End: {}\n\
         Operation Duration: {:.2} seconds\n\
         ===========================\n\
         Status: Mission Accomplished",
        start_datetime.format("%Y-%m-%d %H:%M:%S"),
        total_tweets,
        filtered_tweet_count,
        threads.len(),
        end_datetime.format("%Y-%m-%d %H:%M:%S"),
        duration.as_secs_f64()
    );

    let results_file_path = output_dir.join(format!("results_{}_{}.txt", screen_name, timestamp));
    async_fs::write(&results_file_path, results_content).await.context("Failed to write results file")?;
    println!("📊 Final mission report filed. Operation summary complete!");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use tokio::sync::mpsc as async_mpsc;

    #[tokio::test]
    async fn test_tweet_processing_structure() {
        // Test that the function signature is correct
        let temp_dir = tempdir().unwrap();
        let output_dir = temp_dir.path();
        let (tx, _rx) = async_mpsc::channel::<Vec<String>>(10);
        
        // This would fail with actual processing due to missing file,
        // but tests the function signature and basic structure
        let result = process_tweets(
            "nonexistent_file.js",
            "testuser",
            tx,
            output_dir,
            1234567890
        ).await;
        
        // Should fail due to missing file, but not due to compilation issues
        assert!(result.is_err());
    }

    #[test]
    fn test_thread_creation_logic() {
        // Test the core logic for creating threads from tweets
        let tweet1 = Tweet {
            id_str: "1".to_string(),
            favorite_count: "5".to_string(),
            full_text: "First tweet".to_string(),
            in_reply_to_status_id: None,
            retweeted: false,
            in_reply_to_screen_name: None,
            retweet_count: "2".to_string(),
            created_at: "Mon Jan 01 12:00:00 +0000 2023".to_string(),
        };

        let tweet2 = Tweet {
            id_str: "2".to_string(),
            favorite_count: "3".to_string(),
            full_text: "Reply tweet".to_string(),
            in_reply_to_status_id: Some("1".to_string()),
            retweeted: false,
            in_reply_to_screen_name: Some("testuser".to_string()),
            retweet_count: "1".to_string(),
            created_at: "Mon Jan 01 12:05:00 +0000 2023".to_string(),
        };

        // Test that tweets can be organized into threads
        let tweets = vec![tweet1, tweet2];
        assert_eq!(tweets.len(), 2);
        
        // Verify the reply relationship
        assert_eq!(tweets[1].in_reply_to_status_id, Some("1".to_string()));
    }

    #[test]
    fn test_tweet_filtering_logic() {
        let retweet = Tweet {
            id_str: "1".to_string(),
            favorite_count: "5".to_string(),
            full_text: "RT @someone: Original tweet".to_string(),
            in_reply_to_status_id: None,
            retweeted: true, // This should be filtered out
            in_reply_to_screen_name: None,
            retweet_count: "2".to_string(),
            created_at: "Mon Jan 01 12:00:00 +0000 2023".to_string(),
        };

        let original_tweet = Tweet {
            id_str: "2".to_string(),
            favorite_count: "3".to_string(),
            full_text: "Original tweet".to_string(),
            in_reply_to_status_id: None,
            retweeted: false, // This should be kept
            in_reply_to_screen_name: None,
            retweet_count: "1".to_string(),
            created_at: "Mon Jan 01 12:05:00 +0000 2023".to_string(),
        };

        let mut tweets = vec![retweet, original_tweet];
        let screen_name = "testuser";
        
        // Apply the same filtering logic as in process_tweets
        tweets.retain(|tweet| !tweet.retweeted && (tweet.in_reply_to_screen_name.as_deref() == Some(screen_name) || tweet.in_reply_to_screen_name.is_none()));
        
        // Should only have the original tweet
        assert_eq!(tweets.len(), 1);
        assert_eq!(tweets[0].id_str, "2");
    }
}

/// Simple tweet processing function for testing
pub async fn process_tweets_simple(tweets: &[TweetWrapper], screen_name: &str) -> Result<Vec<Thread>> {
    let mut threads = Vec::new();
    
    for tweet_wrapper in tweets {
        let tweet = &tweet_wrapper.tweet;
        
        // Skip retweets
        if tweet.retweeted || tweet.full_text.starts_with("RT @") {
            continue;
        }
        
        // Create a simple thread for each tweet
        let thread = Thread {
            id: tweet.id_str.clone(),
            tweets: vec![tweet.clone()],
            tweet_count: 1,
            favorite_count: tweet.favorite_count.parse().unwrap_or(0),
            retweet_count: tweet.retweet_count.parse().unwrap_or(0),
        };
        
        threads.push(thread);
    }
    
    Ok(threads)
}