//! Timeline text generation for interaction analysis
//! 
//! Generates chronological interaction logs optimized for LLM analysis.

use std::collections::HashMap;
use chrono::Datelike;
use crate::models::interaction::InteractionEvent;

/// Generates interaction timeline text for LLM analysis
pub fn generate_timeline_text(timeline: &[InteractionEvent]) -> String {
    let mut output = String::new();
    
    add_timeline_header(&mut output, timeline);
    add_monthly_summary(&mut output, timeline);
    add_recent_activity(&mut output, timeline);
    
    output
}

/// Adds timeline header with basic statistics
fn add_timeline_header(output: &mut String, timeline: &[InteractionEvent]) {
    output.push_str("CHRONOLOGICAL INTERACTION LOG\n");
    output.push_str("============================\n");
    output.push_str(&format!("Total Events: {}\n", timeline.len()));
    
    if let (Some(first), Some(last)) = (timeline.first(), timeline.last()) {
        output.push_str(&format!("Time Range: {} to {}\n", 
                               first.timestamp.format("%Y-%m-%d"), 
                               last.timestamp.format("%Y-%m-%d")));
    }
    
    output.push('\n');
}

/// Adds monthly activity summary
fn add_monthly_summary(output: &mut String, timeline: &[InteractionEvent]) {
    let monthly_summary = build_monthly_summary(timeline);
    
    output.push_str("MONTHLY ACTIVITY SUMMARY\n");
    output.push_str("=======================\n");
    
    let mut months: Vec<_> = monthly_summary.keys().collect();
    months.sort();
    
    for month in months {
        if let Some((total, types)) = monthly_summary.get(month) {
            output.push_str(&format!("{}: {} interactions\n", month, total));
            for (interaction_type, count) in types {
                output.push_str(&format!("  - {:?}: {}\n", interaction_type, count));
            }
        }
    }
    
    output.push('\n');
}

/// Adds recent activity section
fn add_recent_activity(output: &mut String, timeline: &[InteractionEvent]) {
    output.push_str("RECENT ACTIVITY (Last 20 Events)\n");
    output.push_str("================================\n");
    
    for event in timeline.iter().take(20) {
        let content_preview = event.content.chars().take(50).collect::<String>();
        output.push_str(&format!("{} | {:?} | User: {} | {}\n",
                               event.timestamp.format("%Y-%m-%d %H:%M"),
                               event.interaction_type,
                               &event.user_hash[..8],
                               content_preview));
    }
}

/// Builds monthly summary from timeline events
fn build_monthly_summary(timeline: &[InteractionEvent]) -> HashMap<String, (u32, HashMap<crate::models::interaction::InteractionType, u32>)> {
    let mut monthly_summary = HashMap::new();
    
    for event in timeline {
        let month_key = format!("{}-{:02}", event.timestamp.year(), event.timestamp.month());
        let entry = monthly_summary.entry(month_key).or_insert((0, HashMap::new()));
        entry.0 += 1;
        *entry.1.entry(event.interaction_type).or_insert(0) += 1;
    }
    
    monthly_summary
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::interaction::InteractionType;
    use chrono::TimeZone;

    fn create_test_timeline() -> Vec<InteractionEvent> {
        vec![
            InteractionEvent::new(
                "event1",
                chrono::Utc.with_ymd_and_hms(2023, 6, 15, 14, 30, 0).unwrap(),
                InteractionType::DmSent,
                "test_user_hash_123456",
                "Test message content"
            ),
            InteractionEvent::new(
                "event2", 
                chrono::Utc.with_ymd_and_hms(2023, 6, 16, 9, 15, 0).unwrap(),
                InteractionType::DmReceived,
                "test_user_hash_123456",
                "Reply message content"
            ),
        ]
    }

    #[test]
    fn test_generate_timeline_text() {
        let timeline = create_test_timeline();
        let timeline_text = generate_timeline_text(&timeline);
        
        assert!(timeline_text.contains("CHRONOLOGICAL INTERACTION LOG"));
        assert!(timeline_text.contains("MONTHLY ACTIVITY SUMMARY"));
        assert!(timeline_text.contains("RECENT ACTIVITY"));
        assert!(timeline_text.contains("Total Events: 2"));
        assert!(timeline_text.contains("2023-06"));
    }

    #[test]
    fn test_build_monthly_summary() {
        let timeline = create_test_timeline();
        let summary = build_monthly_summary(&timeline);
        
        assert!(summary.contains_key("2023-06"));
        if let Some((total, _)) = summary.get("2023-06") {
            assert_eq!(*total, 2);
        }
    }
}