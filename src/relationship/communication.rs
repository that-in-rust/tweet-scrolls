//! Communication pattern analysis and response time calculations

use chrono::{DateTime, Utc, Datelike};
use std::collections::HashMap;
use crate::models::direct_message::DmMessage;

/// Communication frequency analysis
#[derive(Debug, Clone)]
pub struct CommunicationFrequency {
    pub sent_per_month: HashMap<(i32, u32), u32>,
    pub received_per_month: HashMap<(i32, u32), u32>,
    pub avg_per_month_sent: f64,
    pub avg_per_month_received: f64,
}

impl Default for CommunicationFrequency {
    fn default() -> Self {
        Self {
            sent_per_month: HashMap::new(),
            received_per_month: HashMap::new(),
            avg_per_month_sent: 0.0,
            avg_per_month_received: 0.0,
        }
    }
}

/// Calculate response times between consecutive messages in a conversation
/// 
/// # Arguments
/// 
/// * `messages` - A slice of DM messages in chronological order
/// 
/// # Returns
/// 
/// A vector of response times as Duration objects
/// 
/// # Examples
/// 
/// ```
/// use tweet_scrolls::relationship::calculate_response_times;
/// use tweet_scrolls::models::direct_message::{DmMessage, DmMessageCreate};
/// 
/// let messages = vec![
///     // Create test messages with timestamps
/// ];
/// let response_times = calculate_response_times(&messages);
/// assert!(response_times.len() <= messages.len());
/// ```
pub fn calculate_response_times(messages: &[DmMessage]) -> Vec<std::time::Duration> {
    let mut response_times = Vec::new();
    let mut timestamps = Vec::new();
    
    // Extract valid timestamps from messages
    for message in messages {
        if let Some(message_create) = &message.message_create {
            if let Some(created_at) = &message_create.created_at {
                if let Ok(timestamp) = DateTime::parse_from_rfc3339(created_at) {
                    timestamps.push(timestamp.with_timezone(&Utc));
                }
            }
        }
    }
    
    // Calculate time differences between consecutive messages
    for window in timestamps.windows(2) {
        let duration = window[1] - window[0];
        if let Ok(std_duration) = duration.to_std() {
            response_times.push(std_duration);
        }
    }
    
    response_times
}

/// Calculate average response time for a conversation
/// 
/// # Arguments
/// 
/// * `messages` - A slice of DM messages in chronological order
/// 
/// # Returns
/// 
/// The average response time as a Duration, or zero duration if no valid response times
/// 
/// # Examples
/// 
/// ```
/// use tweet_scrolls::relationship::calculate_average_response_time;
/// use tweet_scrolls::models::direct_message::{DmMessage, DmMessageCreate};
/// 
/// let messages = vec![
///     // Create test messages with timestamps
/// ];
/// let avg_time = calculate_average_response_time(&messages);
/// assert!(avg_time >= std::time::Duration::from_secs(0));
/// ```
pub fn calculate_average_response_time(messages: &[DmMessage]) -> std::time::Duration {
    let response_times = calculate_response_times(messages);
    
    if response_times.is_empty() {
        return std::time::Duration::from_secs(0);
    }
    
    let total_nanos: u128 = response_times.iter()
        .map(|d| d.as_nanos())
        .sum();
    
    let avg_nanos = total_nanos / response_times.len() as u128;
    std::time::Duration::from_nanos(avg_nanos as u64)
}

/// Calculate communication frequency metrics for a user
pub fn calculate_communication_frequency(
    user_hash: &str,
    dm_data: &[crate::models::direct_message::DmWrapper]
) -> CommunicationFrequency {
    let mut sent_per_month: HashMap<(i32, u32), u32> = HashMap::new();
    let mut received_per_month: HashMap<(i32, u32), u32> = HashMap::new();
    let mut total_sent = 0;
    let mut total_received = 0;

    for dm_wrapper in dm_data {
        for message in &dm_wrapper.dm_conversation.messages {
            if let Some(create) = &message.message_create {
                if let Some(created_at) = &create.created_at {
                    if let Ok(timestamp) = DateTime::parse_from_rfc3339(created_at) {
                        let datetime = timestamp.with_timezone(&Utc);
                        let month_key = (datetime.year(), datetime.month());

                        // Check if this user sent or received the message
                        if let Some(sender_id) = &create.sender_id {
                            if sender_id == user_hash {
                                *sent_per_month.entry(month_key).or_insert(0) += 1;
                                total_sent += 1;
                            } else {
                                *received_per_month.entry(month_key).or_insert(0) += 1;
                                total_received += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    let avg_per_month_sent = if !sent_per_month.is_empty() {
        total_sent as f64 / sent_per_month.len() as f64
    } else {
        0.0
    };

    let avg_per_month_received = if !received_per_month.is_empty() {
        total_received as f64 / received_per_month.len() as f64
    } else {
        0.0
    };

    CommunicationFrequency {
        sent_per_month,
        received_per_month,
        avg_per_month_sent,
        avg_per_month_received,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::direct_message::{DmMessage, DmMessageCreate};

    fn create_test_message(id: &str, created_at: &str, sender_id: Option<&str>) -> DmMessage {
        DmMessage {
            message_create: Some(DmMessageCreate {
                id: Some(id.to_string()),
                text: Some("Test message".to_string()),
                created_at: Some(created_at.to_string()),
                sender_id: sender_id.map(|s| s.to_string()),
                recipient_id: Some("recipient".to_string()),
            }),
        }
    }

    #[test]
    fn test_calculate_response_times_empty() {
        let messages: Vec<DmMessage> = vec![];
        let response_times = calculate_response_times(&messages);
        assert!(response_times.is_empty());
    }

    #[test]
    fn test_calculate_response_times_single_message() {
        let messages = vec![
            create_test_message("1", "2023-01-01T10:00:00.000Z", Some("user1"))
        ];
        let response_times = calculate_response_times(&messages);
        assert!(response_times.is_empty()); // Need at least 2 messages for response time
    }

    #[test]
    fn test_calculate_response_times_multiple_messages() {
        let messages = vec![
            create_test_message("1", "2023-01-01T10:00:00.000Z", Some("user1")),
            create_test_message("2", "2023-01-01T10:05:00.000Z", Some("user2")),
            create_test_message("3", "2023-01-01T10:10:00.000Z", Some("user1")),
        ];
        
        let response_times = calculate_response_times(&messages);
        assert_eq!(response_times.len(), 2);
        
        // First response time should be 5 minutes (300 seconds)
        assert_eq!(response_times[0].as_secs(), 300);
        
        // Second response time should also be 5 minutes
        assert_eq!(response_times[1].as_secs(), 300);
    }

    #[test]
    fn test_calculate_average_response_time() {
        let messages = vec![
            create_test_message("1", "2023-01-01T10:00:00.000Z", Some("user1")),
            create_test_message("2", "2023-01-01T10:05:00.000Z", Some("user2")),
            create_test_message("3", "2023-01-01T10:10:00.000Z", Some("user1")),
        ];
        
        let avg_time = calculate_average_response_time(&messages);
        assert_eq!(avg_time.as_secs(), 300); // Average of 300 and 300 is 300
    }

    #[test]
    fn test_calculate_average_response_time_empty() {
        let messages: Vec<DmMessage> = vec![];
        let avg_time = calculate_average_response_time(&messages);
        assert_eq!(avg_time.as_secs(), 0);
    }

    #[test]
    fn test_communication_frequency_default() {
        let freq = CommunicationFrequency::default();
        assert!(freq.sent_per_month.is_empty());
        assert!(freq.received_per_month.is_empty());
        assert_eq!(freq.avg_per_month_sent, 0.0);
        assert_eq!(freq.avg_per_month_received, 0.0);
    }

    #[test]
    fn test_invalid_timestamps() {
        let messages = vec![
            DmMessage {
                message_create: Some(DmMessageCreate {
                    id: Some("1".to_string()),
                    text: Some("Test".to_string()),
                    created_at: Some("invalid-timestamp".to_string()), // Invalid timestamp
                    sender_id: Some("user1".to_string()),
                    recipient_id: Some("user2".to_string()),
                }),
            }
        ];
        
        let response_times = calculate_response_times(&messages);
        assert!(response_times.is_empty()); // Should handle invalid timestamps gracefully
    }
}