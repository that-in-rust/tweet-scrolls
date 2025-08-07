/// Tweet classification types for enhanced processing
#[derive(Debug, Clone, PartialEq)]
pub enum TweetType {
    /// User's original tweets
    Original,
    /// Replies to the user's own tweets  
    ReplyToUser,
    /// Replies to other users' tweets
    ReplyToOthers,
}

impl TweetType {
    /// Convert to string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            TweetType::Original => "original",
            TweetType::ReplyToUser => "reply_to_user", 
            TweetType::ReplyToOthers => "reply_to_others",
        }
    }
}