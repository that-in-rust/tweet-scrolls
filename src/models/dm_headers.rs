use serde::Deserialize;

/// Represents a direct message header (metadata only)
#[derive(Debug, Deserialize, Clone)]
pub struct DmHeaderMessage {
    /// The message creation details (headers only)
    #[serde(rename = "messageCreate")]
    pub message_create: DmHeaderMessageCreate,
}

/// Represents the creation details of a direct message header
#[derive(Debug, Deserialize, Clone)]
pub struct DmHeaderMessageCreate {
    /// The unique identifier for the message
    pub id: String,
    /// When the message was created (ISO 8601 format)
    #[serde(rename = "createdAt")]
    pub created_at: String,
    /// The ID of the user who sent the message
    #[serde(rename = "senderId")]
    pub sender_id: String,
    /// The ID of the recipient user
    #[serde(rename = "recipientId")]
    pub recipient_id: String,
}

/// Represents a DM conversation wrapper from the Twitter archive (headers only)
#[derive(Debug, Deserialize, Clone)]
pub struct DmHeaderWrapper {
    /// The conversation details (headers only)
    #[serde(rename = "dmConversation")]
    pub dm_conversation: DmHeaderConversation,
}

/// Represents a DM conversation (headers only)
#[derive(Debug, Deserialize, Clone)]
pub struct DmHeaderConversation {
    /// The conversation ID (format: "user1-user2")
    #[serde(rename = "conversationId")]
    pub conversation_id: String,
    /// The message headers in the conversation
    pub messages: Vec<DmHeaderMessage>,
}