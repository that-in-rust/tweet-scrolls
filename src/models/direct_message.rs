use serde::Deserialize;

/// Represents a direct message in a conversation
#[derive(Debug, Clone, Deserialize)]
pub struct DmMessage {
    /// The message creation details
    #[serde(rename = "messageCreate")]
    pub message_create: Option<DmMessageCreate>,
}

/// Represents the creation details of a direct message
#[derive(Debug, Clone, Deserialize)]
pub struct DmMessageCreate {
    /// The unique identifier for the message
    pub id: Option<String>,
    /// The text content of the message
    pub text: Option<String>,
    /// When the message was created (ISO 8601 format)
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    /// The ID of the user who sent the message
    #[serde(rename = "senderId")]
    pub sender_id: Option<String>,
    /// The ID of the recipient user
    #[serde(rename = "recipientId")]
    pub recipient_id: Option<String>,
    /// Reactions to this message
    #[serde(default)]
    pub reactions: Vec<DmReaction>,
    /// URLs in this message
    #[serde(default)]
    pub urls: Vec<DmUrl>,
    /// Media URLs in this message
    #[serde(rename = "mediaUrls", default)]
    pub media_urls: Vec<String>,
    /// Edit history for this message
    #[serde(rename = "editHistory", default)]
    pub edit_history: Vec<DmEditHistory>,
}

/// Represents a reaction to a direct message
#[derive(Debug, Clone, Deserialize)]
pub struct DmReaction {
    /// The ID of the user who sent the reaction
    #[serde(rename = "senderId")]
    pub sender_id: Option<String>,
    /// The type of reaction (like, excited, etc.)
    #[serde(rename = "reactionKey")]
    pub reaction_key: Option<String>,
    /// The event ID for this reaction
    #[serde(rename = "eventId")]
    pub event_id: Option<String>,
    /// When the reaction was created
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
}

/// Represents a URL in a direct message
#[derive(Debug, Clone, Deserialize)]
pub struct DmUrl {
    /// The shortened URL
    pub url: String,
    /// The expanded/full URL
    pub expanded: String,
    /// The display text for the URL
    pub display: String,
}

/// Represents an edit history entry for a direct message
#[derive(Debug, Clone, Deserialize)]
pub struct DmEditHistory {
    /// When the edit was made (Unix timestamp as string)
    #[serde(rename = "createdAtSec")]
    pub created_at_sec: Option<String>,
    /// The edited text content
    #[serde(rename = "editedText")]
    pub edited_text: Option<String>,
}

/// Represents a DM conversation wrapper from the Twitter archive
#[derive(Debug, Clone, Deserialize)]
pub struct DmWrapper {
    /// The conversation details
    #[serde(rename = "dmConversation")]
    pub dm_conversation: DmConversation,
}

/// Represents a DM conversation
#[derive(Debug, Clone, Deserialize)]
pub struct DmConversation {
    /// The conversation ID (format: "user1-user2")
    #[serde(rename = "conversationId")]
    pub conversation_id: String,
    /// The messages in the conversation
    pub messages: Vec<DmMessage>,
}
