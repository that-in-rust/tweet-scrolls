//! Analysis of user interactions and patterns

use crate::models::interaction::InteractionEvent;
use crate::models::statistics::InteractionStats;
use chrono::{DateTime, Duration, Utc};
use std::collections::HashMap;

/// Analyzes interaction patterns and generates statistics
#[derive(Debug, Default)]
pub struct InteractionAnalyzer {
    /// All processed events
    events: Vec<InteractionEvent>,
    /// User statistics by user hash
    user_stats: HashMap<String, InteractionStats>,
    /// Conversation threads by ID
    conversations: HashMap<String, Vec<InteractionEvent>>,
}

impl InteractionAnalyzer {
    /// Creates a new InteractionAnalyzer
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            user_stats: HashMap::new(),
            conversations: HashMap::new(),
        }
    }

    /// Adds an event to the analyzer
    pub fn add_event(&mut self, event: InteractionEvent) {
        // Update user stats
        let user_hash = event.user_hash.clone();
        let user_stats = self.user_stats.entry(user_hash).or_default();
        user_stats.add_interaction(&event.interaction_type.to_string(), event.timestamp);
        
        // Add to conversations if it's part of one
        if let Some(conv_id) = event.metadata.get("conversation_id") {
            self.conversations
                .entry(conv_id.clone())
                .or_default()
                .push(event.clone());
        }
        
        self.events.push(event);
    }

    /// Analyzes response times across conversations
    pub fn analyze_response_times(&self) -> HashMap<String, Duration> {
        let mut response_times = HashMap::new();
        
        for (_conv_id, messages) in &self.conversations {
            if messages.len() < 2 {
                continue;
            }
            
            let mut sorted_messages = messages.clone();
            sorted_messages.sort_by_key(|m| m.timestamp);
            
            for window in sorted_messages.windows(2) {
                let time_diff = window[1].timestamp - window[0].timestamp;
                response_times.insert(
                    format!("{}_to_{}", window[0].id, window[1].id),
                    time_diff,
                );
            }
        }
        
        response_times
    }

    /// Identifies active time periods
    pub fn identify_active_periods(
        &self,
        window_size: Duration,
        threshold: usize,
    ) -> Vec<(DateTime<Utc>, DateTime<Utc>)> {
        if self.events.is_empty() {
            return Vec::new();
        }
        
        let mut active_periods = Vec::new();
        let mut events = self.events.clone();
        events.sort_by_key(|e| e.timestamp);
        
        let mut start_time = None;
        let mut count_in_window = 0;
        let mut window_start = 0;
        
        for (i, event) in events.iter().enumerate() {
            // Remove events outside the current window
            while window_start < i {
                if events[window_start].timestamp + window_size < event.timestamp {
                    window_start += 1;
                } else {
                    break;
                }
            }
            
            // Count events in current window
            count_in_window = i - window_start + 1;
            
            // Check if we've crossed the threshold
            if count_in_window >= threshold && start_time.is_none() {
                start_time = Some(events[window_start].timestamp);  // Start at first event in window
            } else if count_in_window < threshold && start_time.is_some() {
                // End period at the last event that was still within threshold
                active_periods.push((start_time.unwrap(), events[i-1].timestamp));
                start_time = None;
            }
        }
        
        // Close any open period
        if let Some(start) = start_time {
            if let Some(last) = events.last() {
                active_periods.push((start, last.timestamp));
            }
        }
        
        active_periods
    }

    /// Gets statistics for a specific user
    pub fn get_user_stats(&self, user_hash: &str) -> Option<&InteractionStats> {
        self.user_stats.get(user_hash)
    }

    /// Gets all user hashes that have been analyzed
    pub fn get_analyzed_users(&self) -> Vec<String> {
        self.user_stats.keys().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

    fn create_test_event(id: &str, user: &str, timestamp: DateTime<Utc>) -> InteractionEvent {
        InteractionEvent {
            id: id.to_string(),
            timestamp,
            interaction_type: crate::models::interaction::InteractionType::DmSent,
            user_hash: user.to_string(),
            content: "Test message".to_string(),
            metadata: [
                ("conversation_id".to_string(), "conv1".to_string()),
            ].into_iter().collect(),
        }
    }

    #[test]
    fn test_analyzer_basics() {
        let mut analyzer = InteractionAnalyzer::new();
        let time1 = Utc.with_ymd_and_hms(2023, 1, 1, 12, 0, 0).unwrap();
        let time2 = Utc.with_ymd_and_hms(2023, 1, 1, 12, 5, 0).unwrap();
        
        analyzer.add_event(create_test_event("1", "user1", time1));
        analyzer.add_event(create_test_event("2", "user2", time2));
        
        assert_eq!(analyzer.events.len(), 2);
        assert_eq!(analyzer.user_stats.len(), 2);
        assert_eq!(analyzer.conversations.len(), 1);
        
        let user1_stats = analyzer.get_user_stats("user1").unwrap();
        assert_eq!(user1_stats.total, 1);
        
        let response_times = analyzer.analyze_response_times();
        assert_eq!(response_times.len(), 1);
    }

    #[test]
    fn test_active_periods() {
        let mut analyzer = InteractionAnalyzer::new();
        let base_time = Utc.with_ymd_and_hms(2023, 1, 1, 12, 0, 0).unwrap();
        
        // Add 3 events within 10 minutes
        for i in 0..3 {
            let time = base_time + Duration::minutes(i * 5); // 0, 5, 10 minutes
            analyzer.add_event(create_test_event(&i.to_string(), "user1", time));
        }
        
        // Should detect one active period with window=15min, threshold=2
        let active = analyzer.identify_active_periods(
            Duration::minutes(15),
            2,
        );
        
        assert_eq!(active.len(), 1);
        assert_eq!(active[0].0, base_time);
        assert_eq!(active[0].1, base_time + Duration::minutes(10));
    }
}
