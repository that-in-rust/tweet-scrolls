//! Text generation utilities for relationship intelligence profiles
//! 
//! Focused on generating human-readable content optimized for LLM analysis.

use chrono::{Datelike, Timelike};
use std::collections::HashMap;

use crate::models::profile::UserProfile;
use crate::models::interaction::InteractionEvent;

/// Generates user profile text optimized for LLM analysis
pub fn generate_user_profile_text(profile: &UserProfile, timeline: &[InteractionEvent]) -> String {
    let mut output = String::new();
    
    // Header section
    output.push_str("USER RELATIONSHIP PROFILE\n");
    output.push_str("========================\n");
    output.push_str(&format!("User ID: {}\n", profile.user_id));
    
    add_temporal_info(&mut output, profile);
    add_communication_stats(&mut output, profile);
    add_temporal_patterns(&mut output, timeline);
    add_relationship_insights(&mut output, profile);
    
    output
}

/// Adds temporal information to profile text
fn add_temporal_info(output: &mut String, profile: &UserProfile) {
    if let Some(first) = profile.first_interaction {
        output.push_str(&format!("First Interaction: {}\n", first.format("%Y-%m-%d")));
    }
    
    if let Some(last) = profile.last_interaction {
        output.push_str(&format!("Last Interaction: {}\n", last.format("%Y-%m-%d")));
    }
    
    if let (Some(first), Some(last)) = (profile.first_interaction, profile.last_interaction) {
        let days = (last - first).num_days();
        output.push_str(&format!("Total Interaction Days: {}\n", days));
    }
    
    output.push('\n');
}

/// Adds communication statistics to profile text
fn add_communication_stats(output: &mut String, profile: &UserProfile) {
    output.push_str("COMMUNICATION STATISTICS\n");
    output.push_str("========================\n");
    output.push_str(&format!("Total Interactions: {}\n", profile.total_interactions));
    
    if !profile.interaction_counts.is_empty() {
        output.push_str("Interaction Types:\n");
        for (interaction_type, count) in &profile.interaction_counts {
            let percentage = calculate_percentage(*count, profile.total_interactions);
            output.push_str(&format!("- {}: {} ({:.1}%)\n", interaction_type, count, percentage));
        }
    }
    
    output.push('\n');
}

/// Adds temporal patterns analysis to profile text
fn add_temporal_patterns(output: &mut String, timeline: &[InteractionEvent]) {
    output.push_str("TEMPORAL PATTERNS\n");
    output.push_str("================\n");
    
    if timeline.is_empty() {
        output.push_str("No timeline data available\n\n");
        return;
    }
    
    let (hourly_activity, daily_activity) = analyze_activity_patterns(timeline);
    
    if let Some((most_active_hour, _)) = hourly_activity.iter().enumerate()
        .max_by_key(|(_, &count)| count) {
        output.push_str(&format!("Most active hour: {}:00-{}:59\n", most_active_hour, most_active_hour));
    }
    
    if let Some((most_active_day, _)) = daily_activity.iter()
        .max_by_key(|(_, &count)| count) {
        output.push_str(&format!("Most active day: {:?}\n", most_active_day));
    }
    
    output.push_str(&format!("Total timeline events: {}\n\n", timeline.len()));
}

/// Adds relationship insights to profile text
fn add_relationship_insights(output: &mut String, profile: &UserProfile) {
    output.push_str("RELATIONSHIP INSIGHTS\n");
    output.push_str("====================\n");
    
    let strength = calculate_relationship_strength(profile.total_interactions);
    output.push_str(&format!("- Relationship strength: {} ({} interactions)\n", strength, profile.total_interactions));
    
    add_communication_balance(output, profile);
    add_interaction_consistency(output, profile);
}

/// Calculates percentage with zero division protection
fn calculate_percentage(count: u32, total: u32) -> f64 {
    if total > 0 {
        (count as f64 / total as f64) * 100.0
    } else {
        0.0
    }
}

/// Analyzes activity patterns from timeline
fn analyze_activity_patterns(timeline: &[InteractionEvent]) -> (Vec<u32>, HashMap<chrono::Weekday, u32>) {
    let mut hourly_activity = vec![0; 24];
    let mut daily_activity = HashMap::new();
    
    for event in timeline {
        let hour = event.timestamp.hour() as usize;
        if hour < 24 {
            hourly_activity[hour] += 1;
        }
        
        let weekday = event.timestamp.weekday();
        *daily_activity.entry(weekday).or_insert(0) += 1;
    }
    
    (hourly_activity, daily_activity)
}

/// Calculates relationship strength based on interaction count
fn calculate_relationship_strength(interactions: u32) -> &'static str {
    match interactions {
        0..=5 => "Minimal",
        6..=20 => "Low", 
        21..=100 => "Medium",
        _ => "High",
    }
}

/// Adds communication balance analysis
fn add_communication_balance(output: &mut String, profile: &UserProfile) {
    if let (Some(dm_sent), Some(dm_received)) = (
        profile.interaction_counts.get("dm_messages"),
        profile.interaction_counts.get("dm_received")
    ) {
        let total_dm = dm_sent + dm_received;
        if total_dm > 0 {
            let sent_percentage = (*dm_sent as f64 / total_dm as f64) * 100.0;
            let balance = classify_communication_balance(sent_percentage);
            output.push_str(&format!("- Communication balance: {} ({:.0}% you / {:.0}% them)\n", 
                                   balance, sent_percentage, 100.0 - sent_percentage));
        }
    }
}

/// Adds interaction consistency analysis
fn add_interaction_consistency(output: &mut String, profile: &UserProfile) {
    if let (Some(first), Some(last)) = (profile.first_interaction, profile.last_interaction) {
        let days = (last - first).num_days().max(1);
        let interactions_per_day = profile.total_interactions as f64 / days as f64;
        
        let consistency = classify_interaction_consistency(interactions_per_day);
        output.push_str(&format!("- Interaction consistency: {} ({:.2} per day)\n", 
                               consistency, interactions_per_day));
    }
}

/// Classifies communication balance
fn classify_communication_balance(sent_percentage: f64) -> &'static str {
    if sent_percentage > 60.0 {
        "You-initiated"
    } else if sent_percentage < 40.0 {
        "They-initiated"
    } else {
        "Balanced"
    }
}

/// Classifies interaction consistency
fn classify_interaction_consistency(interactions_per_day: f64) -> &'static str {
    if interactions_per_day > 1.0 {
        "Very High"
    } else if interactions_per_day > 0.5 {
        "High"
    } else if interactions_per_day > 0.1 {
        "Medium"
    } else {
        "Low"
    }
}