//! Core data structures for tweet and DM processing

use serde::{Deserialize, Serialize};
use tokio::sync::mpsc as async_mpsc;

/// Represents a tweet from the Twitter archive
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Tweet {
    // Core tweet fields (always present)
    pub id_str: String,
    pub id: String,
    pub full_text: String,
    pub created_at: String,
    pub favorite_count: String,
    pub retweet_count: String,
    pub retweeted: bool,
    pub favorited: bool,
    pub truncated: bool,
    pub lang: String,
    pub source: String,
    pub display_text_range: Vec<String>,
    
    // Reply fields (optional - only present for replies)
    pub in_reply_to_status_id: Option<String>,
    pub in_reply_to_status_id_str: Option<String>,
    pub in_reply_to_user_id: Option<String>,
    pub in_reply_to_user_id_str: Option<String>,
    pub in_reply_to_screen_name: Option<String>,
    
    // Edit information (may be missing in older tweets)
    #[serde(default)]
    pub edit_info: Option<EditInfo>,
    
    // Entities (always present, but may be empty)
    pub entities: TweetEntities,
    
    // Optional fields
    #[serde(default)]
    pub possibly_sensitive: Option<bool>,
}

/// Edit information for tweets
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct EditInfo {
    pub initial: EditInitial,
}

/// Initial edit information
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct EditInitial {
    #[serde(rename = "editTweetIds")]
    pub edit_tweet_ids: Vec<String>,
    #[serde(rename = "editableUntil")]
    pub editable_until: String,
    #[serde(rename = "editsRemaining")]
    pub edits_remaining: String,
    #[serde(rename = "isEditEligible")]
    pub is_edit_eligible: bool,
}

/// Tweet entities (mentions, hashtags, etc.)
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct TweetEntities {
    pub hashtags: Vec<Hashtag>,
    pub symbols: Vec<Symbol>,
    pub user_mentions: Vec<UserMention>,
    pub urls: Vec<TweetUrl>,
}

/// Hashtag in tweet
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Hashtag {
    pub text: String,
    pub indices: Vec<String>,
}

/// Symbol in tweet (cashtags)
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Symbol {
    pub text: String,
    pub indices: Vec<String>,
}

/// User mention in tweet
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct UserMention {
    pub name: String,
    pub screen_name: String,
    pub indices: Vec<String>,
    pub id_str: String,
    pub id: String,
}

/// URL in tweet
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct TweetUrl {
    pub url: String,
    pub expanded_url: String,
    pub display_url: String,
    pub indices: Vec<String>,
}

/// Wrapper for tweet data from JSON
#[derive(Deserialize, Debug, Clone)]
pub struct TweetWrapper {
    pub tweet: Tweet,
}

/// Represents a conversation thread
#[derive(Debug)]
pub struct Thread {
    pub id: String,
    pub tweets: Vec<Tweet>,
    pub tweet_count: usize,
    pub favorite_count: u32,
    pub retweet_count: u32,
}

/// Represents a processed DM conversation
#[derive(Debug)]
pub struct ProcessedConversation {
    pub conversation_id: String,
    pub message_count: u32,
    pub participants: Vec<String>,
    pub first_message_date: Option<String>,
    pub last_message_date: Option<String>,
}

/// CSV writer for async processing
pub struct CsvWriter {
    pub output_path: String,
    pub receiver: async_mpsc::Receiver<Vec<String>>,
    pub buffer_size: usize,
}

impl CsvWriter {
    /// Creates a new CsvWriter instance
    pub fn new(output_path: String, receiver: async_mpsc::Receiver<Vec<String>>, buffer_size: usize) -> Self {
        Self {
            output_path,
            receiver,
            buffer_size,
        }
    }
}

