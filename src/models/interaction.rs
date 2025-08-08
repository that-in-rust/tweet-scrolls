//! Interaction models for tweet and DM events

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents different types of interactions in the system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InteractionType {
    /// Direct message sent by the user
    DmSent,
    /// Direct message received by the user
    DmReceived,
    /// Tweet sent by the user
    TweetSent,
    /// Tweet received by the user
    TweetReceived,
    /// Mention of the user in a tweet
    Mention,
    /// Reply to a tweet
    Reply,
    /// Reply to a tweet by the user
    TweetReply,
    /// Like/favorite of a tweet
    Like,
    /// Retweet of another tweet
    Retweet,
    /// Quote tweet of another tweet
    Quote,
    /// Other type of interaction
    Other,
}

impl fmt::Display for InteractionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DmSent => write!(f, "DM Sent"),
            Self::DmReceived => write!(f, "DM Received"),
            Self::TweetSent => write!(f, "Tweet Sent"),
            Self::TweetReceived => write!(f, "Tweet Received"),
            Self::Mention => write!(f, "Mention"),
            Self::Reply => write!(f, "Reply"),
            Self::TweetReply => write!(f, "Tweet Reply"),
            Self::Like => write!(f, "Like"),
            Self::Retweet => write!(f, "Retweet"),
            Self::Quote => write!(f, "Quote"),
            Self::Other => write!(f, "Other"),
        }
    }
}

/// Represents a single interaction event in the timeline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionEvent {
    /// Unique identifier for the interaction
    pub id: String,
    /// Timestamp of the interaction
    pub timestamp: DateTime<Utc>,
    /// Type of interaction
    pub interaction_type: InteractionType,
    /// Hash of the user who initiated the interaction
    pub user_hash: String,
    /// Content of the interaction (truncated if needed)
    pub content: String,
    /// Additional metadata as key-value pairs
    pub metadata: std::collections::HashMap<String, String>,
}

impl InteractionEvent {
    /// Creates a new interaction event
    pub fn new(
        id: impl Into<String>,
        timestamp: DateTime<Utc>,
        interaction_type: InteractionType,
        user_hash: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            timestamp,
            interaction_type,
            user_hash: user_hash.into(),
            content: content.into(),
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Adds metadata to the interaction
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
    
    /// Creates an InteractionEvent from a DM message
    pub fn from_dm_message(message: &crate::models::direct_message::DmMessage, conversation_id: &str) -> Option<Self> {
        use chrono::DateTime;
        use std::collections::HashMap;
        
        let message_create = message.message_create.as_ref()?;
        let id = message_create.id.as_ref()?;
        let created_at = message_create.created_at.as_ref()?;
        
        // Parse the timestamp
        let timestamp = match DateTime::parse_from_rfc3339(created_at) {
            Ok(dt) => dt.with_timezone(&chrono::Utc),
            Err(_) => return None,
        };
        
        // Extract participants from conversation ID (format: "user1-user2")
        let participants: Vec<String> = conversation_id
            .split('-')
            .map(|id| id.to_string())
            .collect();
            
        if participants.len() != 2 {
            return None;
        }
        
        // Create metadata
        let mut metadata = HashMap::new();
        metadata.insert("message_id".to_string(), id.clone());
        metadata.insert("conversation_id".to_string(), conversation_id.to_string());
        if let Some(text) = &message_create.text {
            metadata.insert("text_length".to_string(), text.len().to_string());
        }
        
        Some(Self {
            id: id.clone(),
            timestamp,
            interaction_type: InteractionType::DmSent, // Default to sent, adjust if needed
            user_hash: participants[0].clone(), // Assuming first participant is the sender
            content: message_create.text.as_deref().unwrap_or("").to_string(),
            metadata,
        })
    }
}

/// Represents a conversation thread
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationThread {
    /// Unique identifier for the thread
    pub id: String,
    /// List of interaction events in chronological order
    pub events: Vec<InteractionEvent>,
    /// Participants in the conversation (hashed user IDs)
    pub participants: Vec<String>,
    /// Timestamp of the first message
    pub started_at: DateTime<Utc>,
    /// Timestamp of the last message
    pub last_activity: DateTime<Utc>,
}

impl ConversationThread {
    /// Creates a new conversation thread
    pub fn new(id: impl Into<String>) -> Self {
        let id = id.into();
        Self {
            id,
            events: Vec::new(),
            participants: Vec::new(),
            started_at: Utc::now(),
            last_activity: Utc::now(),
        }
    }

    /// Adds an event to the thread
    pub fn add_event(&mut self, event: InteractionEvent) {
        // Update timestamps
        if self.events.is_empty() {
            self.started_at = event.timestamp;
        }
        self.last_activity = event.timestamp;

        // Add participant if new
        if !self.participants.contains(&event.user_hash) {
            self.participants.push(event.user_hash.clone());
        }

        self.events.push(event);
    }
}
