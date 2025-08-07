//! Tests for timeline generation and analysis

use tweet_scrolls::models::interaction::*;
use tweet_scrolls::services::timeline::*;
use chrono::{TimeZone, Utc};

/// Helper function to create test interaction events
fn create_test_events() -> Vec<InteractionEvent> {
    let user1 = "user1".to_string();
    let user2 = "user2".to_string();
    
    vec![
        InteractionEvent::new(
            "1",
            Utc.with_ymd_and_hms(2024, 1, 1, 12, 0, 0).unwrap(),
            InteractionType::DmSent,
            &user1,
            "Hello"
        ),
        InteractionEvent::new(
            "2",
            Utc.with_ymd_and_hms(2024, 1, 1, 12, 5, 0).unwrap(),
            InteractionType::DmReceived,
            &user2,
            "Hi there!"
        ),
        InteractionEvent::new(
            "3",
            Utc.with_ymd_and_hms(2024, 1, 1, 12, 10, 0).unwrap(),
            InteractionType::DmSent,
            &user1,
            "How are you?"
        ),
    ]
}

#[test]
fn test_build_timeline() {
    let mut events = create_test_events();
    let timeline = build_timeline(&mut events);
    
    // Verify events are in reverse chronological order (newest first)
    assert!(timeline.windows(2).all(|w| w[0].timestamp >= w[1].timestamp));
}

#[test]
fn test_group_into_conversations() {
    let events = create_test_events();
    let conversations = group_into_conversations(events, 3600); // 1-hour window
    
    // Should group all events into one conversation
    assert_eq!(conversations.len(), 1);
    assert_eq!(conversations[0].events.len(), 3);
}

#[test]
fn test_calculate_response_times() {
    let mut conversation = ConversationThread::new("test");
    for event in create_test_events() {
        conversation.add_event(event);
    }
    
    let response_times = calculate_response_times(&conversation);
    
    // Should have one less response time than number of events
    assert_eq!(response_times.len(), conversation.events.len() - 1);
    // First response time should be 5 minutes (300 seconds)
    assert_eq!(response_times[0].num_seconds(), 300);
}
