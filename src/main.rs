use anyhow::{Context, Result};
use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::Path;
use std::time::Instant;
use tokio::fs as async_fs;
use tokio::sync::mpsc;
use tokio::task;
use csv::Writer as CsvWriterLib;
use blake3;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[tokio::main]
async fn main() -> Result<()> {
    let input_file = get_input_file()?;
    let screen_name = get_screen_name()?;
    let timestamp = Utc::now().timestamp();

    println!("🕶️ Current working directory: {}", std::env::current_dir()?.display());

    if !async_fs::metadata(&input_file).await.is_ok() {
        anyhow::bail!("❌ File does not exist: {}", input_file);
    }
    //  |   |          |          |       |
    //  |   |          |          |       Error message with file path
    //  |   |          |          Check if file exists
    //  |   |          Await async operation
    //  |   Get metadata of input file
    //  Negate the result to check if file does not exist

    // Create output directory
    let input_path = Path::new(&input_file);
    //  |           |      |
    //  |           |      Convert input file path to Path
    //  |           Path::new() creates a new Path instance
    //  Variable to hold the Path instance
    let output_dir = input_path.parent().unwrap().join(format!("output_{}_{}", screen_name, timestamp));
    //  |           |          |      |     |      |
    //  |           |          |      |     |      Timestamp value
    //  |           |          |      |     Screen name value
    //  |           |          |      String formatting with {}
    //  |           |          join() adds path component
    //  |           Gets parent directory, unwraps Option
    //  Output directory PathBuf
    
    // Memory layout:
    // input_path: "~/data/tweets.json"
    //      |
    //      v
    // parent(): "~/data"
    //      |
    //      v 
    // join(): "~/data/output_alice_1234567890"

    async_fs::create_dir_all(&output_dir).await.context("Failed to create output directory")?;

    // Create a channel for CsvWriter
    let (tx, rx) = mpsc::channel::<Vec<String>>(100);

    // Initialize CsvWriter and spawn its run task
    let csv_writer = CsvWriter::new(output_dir.join(format!("threads_{}_{}.csv", screen_name, timestamp)).to_str().unwrap().to_string(), rx, 100);
    tokio::spawn(csv_writer.run());

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

    Ok(())
}

fn get_input_file() -> Result<String> {
    prompt_input("🗂️ Please enter the absolute path to the input JSON file: ")
}

fn get_screen_name() -> Result<String> {
    prompt_input("🕵️‍♂️ Please enter the Twitter handle: ")
}

fn get_dm_file() -> Result<Option<String>> {
    let input = prompt_input("📱 Enter DM file path (optional, press Enter to skip): ")?;
    if input.is_empty() {
        Ok(None)
    } else {
        Ok(Some(input))
    }
}

fn prompt_input(prompt: &str) -> Result<String> {
    print!("{}", prompt);
    io::stdout().flush().context("Failed to flush stdout")?;
    let mut input = String::new();
    io::stdin().read_line(&mut input).context("Failed to read input")?;
    Ok(input.trim().to_string())
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Tweet {
    pub id_str: String,
    pub favorite_count: String,
    pub full_text: String,
    pub in_reply_to_status_id: Option<String>,
    pub retweeted: bool,
    pub in_reply_to_screen_name: Option<String>,
    pub retweet_count: String,
    pub created_at: String,
}

#[derive(Deserialize, Debug)]
pub struct TweetWrapper {
    pub tweet: Tweet,
}

#[derive(Debug)]
pub struct Thread {
    pub id: String,
    pub tweets: Vec<Tweet>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DmMessage {
    #[serde(rename = "messageCreate")]
    pub message_create: Option<DmMessageCreate>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DmMessageCreate {
    pub id: Option<String>,
    pub text: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct DmConversation {
    #[serde(rename = "conversationId")]
    pub conversation_id: String,
    pub messages: Vec<DmMessage>,
}

#[derive(Deserialize, Debug)]
pub struct DmWrapper {
    #[serde(rename = "dmConversation")]
    pub dm_conversation: DmConversation,
}

#[derive(Debug)]
pub struct ProcessedConversation {
    pub id: String,
    pub message_count: usize,
    pub first_message_date: Option<String>,
    pub last_message_date: Option<String>,
}

pub struct CsvWriter {
    output_path: String,
    receiver: mpsc::Receiver<Vec<String>>,
    buffer_size: usize,
}

impl CsvWriter {
    pub fn new(output_path: String, receiver: mpsc::Receiver<Vec<String>>, buffer_size: usize) -> Self {
        Self {
            output_path,
            receiver,
            buffer_size,
        }
    }

    pub async fn run(mut self) -> Result<()> {
        let file = File::create(&self.output_path)
            .with_context(|| format!("Failed to create file: {}", self.output_path))?;
        let mut writer = CsvWriterLib::from_writer(BufWriter::new(file));

        // Write headers
        writer.write_record(&[
            "Thread ID",
            "Date time of first tweet",
            "Number of Tweets in Thread",
            "Likes in first tweet",
            "Retweets in first tweet",
            "Total likes for all tweets",
            "Total retweets for all tweets",
            "Thread Text",
        ])?;

        let mut buffer = Vec::with_capacity(self.buffer_size);

        while let Some(record) = self.receiver.recv().await {
            buffer.push(record);
            if buffer.len() >= self.buffer_size {
                self.flush_buffer(&mut writer, &mut buffer)?;
            }
        }

        if !buffer.is_empty() {
            self.flush_buffer(&mut writer, &mut buffer)?;
        }

        writer.flush()?;
        Ok(())
    }

    fn flush_buffer(&self, writer: &mut CsvWriterLib<BufWriter<File>>, buffer: &mut Vec<Vec<String>>) -> Result<()> {
        for record in buffer.drain(..) {
            writer.write_record(&record)?;
        }
        Ok(())
    }
}

pub async fn process_tweets(input_file: &str, screen_name: &str, csv_tx: mpsc::Sender<Vec<String>>, output_dir: &Path, _timestamp: i64) -> Result<()> {
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
        Thread { id, tweets: thread }
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

async fn write_threads_to_file(threads: &[Thread], screen_name: &str, timestamp: i64, output_dir: &Path) -> Result<()> {
    let file_path = output_dir.join(format!("threads_{}_{}.txt", screen_name, timestamp));
    let file = File::create(&file_path)?;
    let mut writer = BufWriter::new(file);

    for thread in threads {
        writeln!(writer, "--- Start of Thread ---")?;
        writeln!(writer, "Thread ID: {}", thread.id)?;
        writeln!(writer, "Timestamp: {}", thread.tweets[0].created_at)?;
        writeln!(writer, "Public Support: {} retweets, {} likes",
                 thread.tweets[0].retweet_count, thread.tweets[0].favorite_count)?;
        writeln!(writer, "Thread text:")?;

        for (i, tweet) in thread.tweets.iter().enumerate() {
            writeln!(writer, "- Tweet {}:", i + 1)?;
            writeln!(writer, "{}", tweet.full_text)?;
            writeln!(writer)?;
        }

        writeln!(writer, "--- End of Thread ---\n")?;
    }

    writer.flush()?;
    Ok(())
}

async fn write_csv(
    threads: &[Thread],
    _screen_name: &str,
    _timestamp: i64,
    csv_tx: mpsc::Sender<Vec<String>>,
) -> Result<()> {
    for thread in threads {
        let first_tweet = &thread.tweets[0];
        let total_likes: u32 = thread.tweets.iter().filter_map(|t| t.favorite_count.parse::<u32>().ok()).sum();
        let total_retweets: u32 = thread.tweets.iter().filter_map(|t| t.retweet_count.parse::<u32>().ok()).sum();
        let thread_text: String = thread.tweets.iter().map(|t| t.full_text.replace('\n', " ")).collect::<Vec<_>>().join(" ");

        let record = vec![
            thread.id.clone(),
            first_tweet.created_at.clone(),
            thread.tweets.len().to_string(),
            first_tweet.favorite_count.clone(),
            first_tweet.retweet_count.clone(),
            total_likes.to_string(),
            total_retweets.to_string(),
            thread_text,
        ];

        csv_tx.send(record).await?;
    }

    Ok(())
}

/// Anonymizes a user ID using Blake3 hashing
/// 
/// This function takes a user ID string and returns a consistent, anonymized hash.
/// The same user ID will always produce the same hash, but the original ID cannot
/// be recovered from the hash.
pub fn hash_user_id(user_id: &str) -> String {
    let hash = blake3::hash(user_id.as_bytes());
    hash.to_hex().to_string()
}

/// Relationship analyzer for extracting and analyzing user interactions
#[derive(Debug)]
pub struct RelationshipAnalyzer {
    pub profiles: HashMap<String, UserProfile>,
}

impl RelationshipAnalyzer {
    pub fn new() -> Self {
        Self {
            profiles: HashMap::new(),
        }
    }

    /// Extract unique user IDs from DM data
    pub fn extract_users_from_dms(&self, dm_wrappers: &[DmWrapper]) -> std::collections::HashSet<String> {
        let mut users = std::collections::HashSet::new();
        
        for wrapper in dm_wrappers {
            let conversation_id = &wrapper.dm_conversation.conversation_id;
            
            // Extract user IDs from conversation ID (format: "user1-user2")
            if let Some(dash_pos) = conversation_id.find('-') {
                let user1 = &conversation_id[..dash_pos];
                let user2 = &conversation_id[dash_pos + 1..];
                
                users.insert(hash_user_id(user1));
                users.insert(hash_user_id(user2));
            }
        }
        
        users
    }

    /// Extract unique user IDs from tweet data
    pub fn extract_users_from_tweets(&self, tweets: &[Tweet]) -> std::collections::HashSet<String> {
        let mut users = std::collections::HashSet::new();
        
        for tweet in tweets {
            // Add the tweet author (we don't have author ID in current Tweet struct, 
            // but we can extract from mentions and replies)
            if let Some(reply_to_user) = &tweet.in_reply_to_screen_name {
                users.insert(hash_user_id(reply_to_user));
            }
        }
        
        users
    }

    /// Create a basic user profile from conversation data
    pub fn create_user_profile(&self, user_hash: &str, dm_data: &[DmWrapper]) -> UserProfile {
        let dm_stats = self.calculate_dm_statistics(user_hash, dm_data);
        let (first_interaction, last_interaction) = self.find_interaction_timespan(user_hash, dm_data);
        
        UserProfile {
            user_hash: user_hash.to_string(),
            first_interaction,
            last_interaction,
            dm_stats,
        }
    }

    /// Calculate DM statistics for a specific user
    pub fn calculate_dm_statistics(&self, user_hash: &str, dm_data: &[DmWrapper]) -> DmStatistics {
        let mut stats = DmStatistics::default();
        
        for wrapper in dm_data {
            let conversation_id = &wrapper.dm_conversation.conversation_id;
            
            // Check if this user is part of this conversation
            if let Some(dash_pos) = conversation_id.find('-') {
                let user1_hash = hash_user_id(&conversation_id[..dash_pos]);
                let user2_hash = hash_user_id(&conversation_id[dash_pos + 1..]);
                
                if user_hash == user1_hash || user_hash == user2_hash {
                    // Count messages in this conversation
                    for message in &wrapper.dm_conversation.messages {
                        if message.message_create.is_some() {
                            stats.total_messages += 1;
                            // For now, we'll count all messages as "sent" since we don't have sender info
                            // In a real implementation, we'd need sender ID to distinguish sent vs received
                            stats.messages_sent += 1;
                        }
                    }
                }
            }
        }
        
        stats
    }

    /// Find the first and last interaction timestamps for a user
    fn find_interaction_timespan(&self, user_hash: &str, dm_data: &[DmWrapper]) -> (Option<DateTime<Utc>>, Option<DateTime<Utc>>) {
        let mut timestamps = Vec::new();
        
        for wrapper in dm_data {
            let conversation_id = &wrapper.dm_conversation.conversation_id;
            
            // Check if this user is part of this conversation
            if let Some(dash_pos) = conversation_id.find('-') {
                let user1_hash = hash_user_id(&conversation_id[..dash_pos]);
                let user2_hash = hash_user_id(&conversation_id[dash_pos + 1..]);
                
                if user_hash == user1_hash || user_hash == user2_hash {
                    // Collect timestamps from this conversation
                    for message in &wrapper.dm_conversation.messages {
                        if let Some(message_create) = &message.message_create {
                            if let Some(created_at) = &message_create.created_at {
                                if let Ok(timestamp) = DateTime::parse_from_rfc3339(created_at) {
                                    timestamps.push(timestamp.with_timezone(&Utc));
                                }
                            }
                        }
                    }
                }
            }
        }
        
        if timestamps.is_empty() {
            (None, None)
        } else {
            timestamps.sort();
            (timestamps.first().copied(), timestamps.last().copied())
        }
    }

    /// Build a chronological interaction timeline from DM and tweet data
    pub fn build_timeline(&self, dm_data: &[DmWrapper], tweet_data: &[Tweet]) -> Vec<InteractionEvent> {
        let mut timeline = Vec::new();
        
        // Add DM events to timeline
        for wrapper in dm_data {
            let conversation_id = &wrapper.dm_conversation.conversation_id;
            for message in &wrapper.dm_conversation.messages {
                if let Some(event) = InteractionEvent::from_dm_message(message, conversation_id) {
                    timeline.push(event);
                }
            }
        }
        
        // Add tweet events to timeline
        for tweet in tweet_data {
            if let Some(event) = InteractionEvent::from_tweet(tweet) {
                timeline.push(event);
            }
        }
        
        // Sort timeline chronologically (newest first)
        timeline.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        
        timeline
    }
}

/// Basic user profile structure
#[derive(Debug, Clone)]
pub struct UserProfile {
    pub user_hash: String,
    pub first_interaction: Option<DateTime<Utc>>,
    pub last_interaction: Option<DateTime<Utc>>,
    pub dm_stats: DmStatistics,
}

/// DM statistics for a user
#[derive(Debug, Clone, Default)]
pub struct DmStatistics {
    pub total_messages: usize,
    pub messages_sent: usize,
    pub messages_received: usize,
}

/// Represents different types of interactions
#[derive(Debug, Clone, PartialEq)]
pub enum InteractionType {
    DmSent,
    DmReceived,
    TweetMention,
    TweetReply,
    ReactionGiven,
    ReactionReceived,
}

/// Represents a single interaction event in the timeline
#[derive(Debug, Clone)]
pub struct InteractionEvent {
    pub timestamp: DateTime<Utc>,
    pub event_type: InteractionType,
    pub participants: Vec<String>,
    pub metadata: HashMap<String, String>,
}

impl InteractionEvent {
    /// Create an InteractionEvent from a DM message
    pub fn from_dm_message(message: &DmMessage, conversation_id: &str) -> Option<Self> {
        if let Some(message_create) = &message.message_create {
            if let (Some(created_at), Some(message_id)) = (&message_create.created_at, &message_create.id) {
                if let Ok(timestamp) = DateTime::parse_from_rfc3339(created_at) {
                    // Extract participants from conversation ID
                    let participants = if let Some(dash_pos) = conversation_id.find('-') {
                        vec![
                            hash_user_id(&conversation_id[..dash_pos]),
                            hash_user_id(&conversation_id[dash_pos + 1..]),
                        ]
                    } else {
                        vec![]
                    };

                    let mut metadata = HashMap::new();
                    metadata.insert("message_id".to_string(), message_id.clone());
                    metadata.insert("conversation_id".to_string(), conversation_id.to_string());
                    if let Some(text) = &message_create.text {
                        metadata.insert("text_length".to_string(), text.len().to_string());
                    }

                    return Some(InteractionEvent {
                        timestamp: timestamp.with_timezone(&Utc),
                        event_type: InteractionType::DmSent, // For now, assume all are sent
                        participants,
                        metadata,
                    });
                }
            }
        }
        None
    }

    /// Create an InteractionEvent from a tweet
    pub fn from_tweet(tweet: &Tweet) -> Option<Self> {
        if let Ok(timestamp) = DateTime::parse_from_str(&tweet.created_at, "%a %b %d %H:%M:%S %z %Y") {
            let mut participants = vec![];
            let mut metadata = HashMap::new();
            
            metadata.insert("tweet_id".to_string(), tweet.id_str.clone());
            metadata.insert("text_length".to_string(), tweet.full_text.len().to_string());
            metadata.insert("favorite_count".to_string(), tweet.favorite_count.clone());
            metadata.insert("retweet_count".to_string(), tweet.retweet_count.clone());

            let event_type = if let Some(reply_to_user) = &tweet.in_reply_to_screen_name {
                participants.push(hash_user_id(reply_to_user));
                InteractionType::TweetReply
            } else {
                InteractionType::TweetMention // Default for now
            };

            return Some(InteractionEvent {
                timestamp: timestamp.with_timezone(&Utc),
                event_type,
                participants,
                metadata,
            });
        }
        None
    }
}

pub async fn process_dm_file(dm_file: &str, _screen_name: &str, output_dir: &Path, _timestamp: i64) -> Result<()> {
    let start_time = Instant::now();
    
    println!("📱 Reading DM file...");
    let dm_content = async_fs::read_to_string(dm_file).await
        .with_context(|| format!("Failed to read DM file: {}", dm_file))?;
    
    println!("🔍 Parsing DM data...");
    // Remove JavaScript assignment prefix if present (handle both formats)
    let json_content = if let Some(stripped) = dm_content.strip_prefix("window.YTD.direct_messages.part0 = ") {
        stripped
    } else if let Some(stripped) = dm_content.strip_prefix("window.YTD.direct_message_headers.part0 = ") {
        stripped
    } else {
        &dm_content
    };
    
    let dm_wrappers: Vec<DmWrapper> = from_str(json_content)
        .context("Failed to parse DM JSON")?;
    
    println!("💬 Processing {} conversations...", dm_wrappers.len());
    
    let mut conversations: Vec<ProcessedConversation> = dm_wrappers
        .into_iter()
        .map(|wrapper| {
            let conv = wrapper.dm_conversation;
            let valid_messages: Vec<_> = conv.messages
                .iter()
                .filter(|msg| msg.message_create.is_some())
                .collect();
            
            let first_date = valid_messages.first()
                .and_then(|msg| msg.message_create.as_ref())
                .and_then(|mc| mc.created_at.clone());
            
            let last_date = valid_messages.last()
                .and_then(|msg| msg.message_create.as_ref())
                .and_then(|mc| mc.created_at.clone());
            
            ProcessedConversation {
                id: conv.conversation_id,
                message_count: valid_messages.len(),
                first_message_date: first_date,
                last_message_date: last_date,
            }
        })
        .filter(|conv| conv.message_count > 0)
        .collect();
    
    // Sort by message count (descending)
    conversations.sort_by(|a, b| b.message_count.cmp(&a.message_count));
    
    println!("📊 Writing DM results...");
    
    // Write CSV file
    let csv_path = output_dir.join(format!("dm_conversations_{}_{}.csv", _screen_name, _timestamp));
    let csv_file = File::create(&csv_path)?;
    let mut csv_writer = CsvWriterLib::from_writer(BufWriter::new(csv_file));
    
    csv_writer.write_record(&[
        "Conversation ID",
        "Message Count", 
        "First Message Date",
        "Last Message Date",
    ])?;
    
    for conv in &conversations {
        csv_writer.write_record(&[
            &conv.id,
            &conv.message_count.to_string(),
            conv.first_message_date.as_deref().unwrap_or("N/A"),
            conv.last_message_date.as_deref().unwrap_or("N/A"),
        ])?;
    }
    csv_writer.flush()?;
    
    // Write TXT file
    let txt_path = output_dir.join(format!("dm_conversations_{}_{}.txt", _screen_name, _timestamp));
    let txt_file = File::create(&txt_path)?;
    let mut txt_writer = BufWriter::new(txt_file);
    
    for (i, conv) in conversations.iter().enumerate() {
        writeln!(txt_writer, "--- Conversation {} ---", i + 1)?;
        writeln!(txt_writer, "ID: {}", conv.id)?;
        writeln!(txt_writer, "Messages: {}", conv.message_count)?;
        if let Some(first) = &conv.first_message_date {
            writeln!(txt_writer, "First Message: {}", first)?;
        }
        if let Some(last) = &conv.last_message_date {
            writeln!(txt_writer, "Last Message: {}", last)?;
        }
        writeln!(txt_writer)?;
    }
    txt_writer.flush()?;
    
    // Write summary
    let duration = start_time.elapsed();
    let total_messages: usize = conversations.iter().map(|c| c.message_count).sum();
    
    let summary = format!(
        "DM Processing Summary\n\
         ====================\n\
         Total Conversations: {}\n\
         Total Messages: {}\n\
         Processing Time: {:.2} seconds\n\
         ====================\n\
         Status: Complete",
        conversations.len(),
        total_messages,
        duration.as_secs_f64()
    );
    
    let summary_path = output_dir.join(format!("dm_results_{}_{}.txt", _screen_name, _timestamp));
    async_fs::write(&summary_path, summary).await.context("Failed to write DM summary")?;
    
    println!("✅ DM processing complete! Generated {} files", 3);
    Ok(())
}

