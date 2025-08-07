//! Timeline analysis integration for relationship intelligence

use chrono::{Weekday, Timelike, Datelike};
use std::collections::HashMap;
use crate::models::interaction::InteractionEvent;

/// Analyze hourly activity patterns from interaction events
/// 
/// # Arguments
/// 
/// * `events` - A slice of InteractionEvent objects
/// 
/// # Returns
/// 
/// A vector of 24 elements representing activity count for each hour (0-23)
/// 
/// # Examples
/// 
/// ```
/// use tweet_scrolls::relationship::analyze_hourly_activity;
/// 
/// let events = vec![]; // Your interaction events
/// let hourly_activity = analyze_hourly_activity(&events);
/// assert_eq!(hourly_activity.len(), 24);
/// ```
pub fn analyze_hourly_activity(events: &[InteractionEvent]) -> Vec<usize> {
    let mut hourly_counts = vec![0; 24];
    
    for event in events {
        let hour = event.timestamp.hour() as usize;
        if hour < 24 {
            hourly_counts[hour] += 1;
        }
    }
    
    hourly_counts
}

/// Find the most active day of the week from interaction events
/// 
/// # Arguments
/// 
/// * `events` - A slice of InteractionEvent objects
/// 
/// # Returns
/// 
/// The most active day of the week, or None if no events
/// 
/// # Examples
/// 
/// ```
/// use tweet_scrolls::relationship::find_most_active_day;
/// 
/// let events = vec![]; // Your interaction events
/// let most_active = find_most_active_day(&events);
/// ```
pub fn find_most_active_day(events: &[InteractionEvent]) -> Option<Weekday> {
    let mut day_counts = HashMap::new();
    
    for event in events {
        let weekday = event.timestamp.weekday();
        *day_counts.entry(weekday).or_insert(0) += 1;
    }
    
    day_counts.into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(day, _)| day)
}

/// Calculate activity distribution across days of the week
/// 
/// # Arguments
/// 
/// * `events` - A slice of InteractionEvent objects
/// 
/// # Returns
/// 
/// A HashMap mapping weekdays to activity counts
pub fn calculate_weekly_distribution(events: &[InteractionEvent]) -> HashMap<Weekday, usize> {
    let mut day_counts = HashMap::new();
    
    for event in events {
        let weekday = event.timestamp.weekday();
        *day_counts.entry(weekday).or_insert(0) += 1;
    }
    
    day_counts
}

/// Find peak activity hours (hours with above-average activity)
/// 
/// # Arguments
/// 
/// * `events` - A slice of InteractionEvent objects
/// 
/// # Returns
/// 
/// A vector of hours (0-23) that have above-average activity
pub fn find_peak_activity_hours(events: &[InteractionEvent]) -> Vec<u32> {
    let hourly_activity = analyze_hourly_activity(events);
    
    if events.is_empty() {
        return vec![];
    }
    
    let average_activity = events.len() as f64 / 24.0;
    
    hourly_activity
        .iter()
        .enumerate()
        .filter(|(_, &count)| count as f64 > average_activity)
        .map(|(hour, _)| hour as u32)
        .collect()
}

/// Calculate interaction density over time periods
/// 
/// # Arguments
/// 
/// * `events` - A slice of InteractionEvent objects
/// * `window_hours` - Size of the time window in hours
/// 
/// # Returns
/// 
/// A vector of interaction counts for each time window
pub fn calculate_interaction_density(events: &[InteractionEvent], window_hours: i64) -> Vec<usize> {
    if events.is_empty() {
        return vec![];
    }
    
    let mut sorted_events = events.to_vec();
    sorted_events.sort_by_key(|e| e.timestamp);
    
    let start_time = sorted_events.first().unwrap().timestamp;
    let end_time = sorted_events.last().unwrap().timestamp;
    
    let window_duration = chrono::Duration::hours(window_hours);
    let total_duration = end_time - start_time;
    let num_windows = (total_duration.num_hours() / window_hours).max(1) as usize;
    
    let mut density = vec![0; num_windows];
    
    for event in &sorted_events {
        let elapsed = event.timestamp - start_time;
        let window_index = (elapsed.num_hours() / window_hours).min(num_windows as i64 - 1) as usize;
        density[window_index] += 1;
    }
    
    density
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};
    use crate::models::interaction::{InteractionEvent, InteractionType};

    fn create_test_event(hour: u32, day_offset: i64) -> InteractionEvent {
        let base_date = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap(); // Sunday
        let timestamp = base_date + chrono::Duration::days(day_offset) + chrono::Duration::hours(hour as i64);
        
        InteractionEvent::new(
            format!("event_{}", hour),
            timestamp,
            InteractionType::DmSent,
            "user1",
            "Test message"
        )
    }

    #[test]
    fn test_analyze_hourly_activity_empty() {
        let events: Vec<InteractionEvent> = vec![];
        let hourly_activity = analyze_hourly_activity(&events);
        
        assert_eq!(hourly_activity.len(), 24);
        assert!(hourly_activity.iter().all(|&count| count == 0));
    }

    #[test]
    fn test_analyze_hourly_activity_with_events() {
        let events = vec![
            create_test_event(10, 0), // 10 AM
            create_test_event(10, 0), // 10 AM (another event)
            create_test_event(14, 0), // 2 PM
            create_test_event(22, 0), // 10 PM
        ];
        
        let hourly_activity = analyze_hourly_activity(&events);
        
        assert_eq!(hourly_activity.len(), 24);
        assert_eq!(hourly_activity[10], 2); // Two events at 10 AM
        assert_eq!(hourly_activity[14], 1); // One event at 2 PM
        assert_eq!(hourly_activity[22], 1); // One event at 10 PM
        assert_eq!(hourly_activity[0], 0);  // No events at midnight
    }

    #[test]
    fn test_find_most_active_day_empty() {
        let events: Vec<InteractionEvent> = vec![];
        let most_active = find_most_active_day(&events);
        assert!(most_active.is_none());
    }

    #[test]
    fn test_find_most_active_day_with_events() {
        let events = vec![
            create_test_event(10, 0), // Sunday
            create_test_event(11, 0), // Sunday
            create_test_event(12, 1), // Monday
            create_test_event(13, 2), // Tuesday
            create_test_event(14, 2), // Tuesday
            create_test_event(15, 2), // Tuesday
        ];
        
        let most_active = find_most_active_day(&events);
        assert_eq!(most_active, Some(Weekday::Tue)); // Tuesday has 3 events
    }

    #[test]
    fn test_calculate_weekly_distribution() {
        let events = vec![
            create_test_event(10, 0), // Sunday
            create_test_event(11, 1), // Monday
            create_test_event(12, 1), // Monday
            create_test_event(13, 2), // Tuesday
        ];
        
        let distribution = calculate_weekly_distribution(&events);
        
        assert_eq!(distribution.get(&Weekday::Sun), Some(&1));
        assert_eq!(distribution.get(&Weekday::Mon), Some(&2));
        assert_eq!(distribution.get(&Weekday::Tue), Some(&1));
        assert_eq!(distribution.get(&Weekday::Wed), None);
    }

    #[test]
    fn test_find_peak_activity_hours() {
        let events = vec![
            create_test_event(10, 0),
            create_test_event(10, 1),
            create_test_event(10, 2), // Hour 10 has 3 events
            create_test_event(14, 0),
            create_test_event(14, 1), // Hour 14 has 2 events
            create_test_event(22, 0), // Hour 22 has 1 event
        ];
        
        let peak_hours = find_peak_activity_hours(&events);
        
        // Average activity is 6/24 = 0.25 events per hour
        // Hours 10 (3 events) and 14 (2 events) are above average
        assert!(peak_hours.contains(&10));
        assert!(peak_hours.contains(&14));
        assert!(!peak_hours.contains(&22)); // Only 1 event, below threshold
    }

    #[test]
    fn test_calculate_interaction_density() {
        let events = vec![
            create_test_event(10, 0), // Day 0, hour 10
            create_test_event(11, 0), // Day 0, hour 11
            create_test_event(10, 1), // Day 1, hour 10
            create_test_event(10, 2), // Day 2, hour 10
        ];
        
        let density = calculate_interaction_density(&events, 24); // 24-hour windows
        
        // Should have density data for the time span
        assert!(!density.is_empty());
        
        // First window (day 0) should have 2 events
        assert_eq!(density[0], 2);
        
        // Subsequent windows should have 1 event each
        if density.len() > 1 {
            assert_eq!(density[1], 1);
        }
        if density.len() > 2 {
            assert_eq!(density[2], 1);
        }
    }

    #[test]
    fn test_calculate_interaction_density_empty() {
        let events: Vec<InteractionEvent> = vec![];
        let density = calculate_interaction_density(&events, 24);
        assert!(density.is_empty());
    }

    #[test]
    fn test_edge_cases() {
        // Test with events at hour boundaries
        let events = vec![
            create_test_event(0, 0),  // Midnight
            create_test_event(23, 0), // 11 PM
        ];
        
        let hourly_activity = analyze_hourly_activity(&events);
        assert_eq!(hourly_activity[0], 1);   // Midnight
        assert_eq!(hourly_activity[23], 1);  // 11 PM
        
        // Test weekly distribution with single day
        let distribution = calculate_weekly_distribution(&events);
        assert_eq!(distribution.get(&Weekday::Sun), Some(&2)); // Both events on Sunday
    }
}