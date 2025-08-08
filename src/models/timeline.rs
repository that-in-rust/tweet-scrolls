//! Timeline analysis models for the Tweet-Scrolls application

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents different types of patterns that can be detected in a timeline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimelinePattern {
    /// Regular daily activity pattern
    DailyRhythm,
    /// Activity during specific hours of the day
    TimeOfDayPattern { 
        /// The hours of the day (0-23) when activity is highest
        active_hours: Vec<u32> 
    },
    /// Weekly activity pattern (e.g., more active on weekends)
    WeeklyPattern { 
        /// The days of the week when activity is highest
        active_days: Vec<chrono::Weekday> 
    },
    /// Bursty activity pattern (periods of high activity followed by low activity)
    BurstyActivity,
    /// No discernible pattern
    NoPattern,
}

/// Metrics about the timeline density
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineDensity {
    /// Average number of interactions per day
    pub avg_interactions_per_day: f64,
    /// Peak interaction times
    pub peak_hours: Vec<u32>,
    /// Days with highest activity
    pub peak_days: Vec<chrono::Weekday>,
    /// Hour of the day with highest activity (0-23)
    pub peak_hour: u32,
    /// Day of week with highest activity (0-6, where 0 is Sunday)
    pub peak_day: u32,
}

/// Response time statistics for a conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseTimeStats {
    /// Average response time in seconds
    pub average: f64,
    /// Median response time in seconds
    pub median: f64,
    /// Response time percentiles (p50, p90, p95, p99)
    pub percentiles: HashMap<String, f64>,
    /// Fastest response time in seconds
    pub min: f64,
    /// Slowest response time in seconds
    pub max: f64,
}

/// Analysis results for a timeline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineAnalysis {
    /// Detected patterns in the timeline
    pub patterns: Vec<TimelinePattern>,
    /// Density metrics
    pub density: TimelineDensity,
    /// Response time statistics
    pub response_times: ResponseTimeStats,
    /// Start of the analysis period
    pub start_time: DateTime<Utc>,
    /// End of the analysis period
    pub end_time: DateTime<Utc>,
    /// Total number of interactions in the timeline
    pub total_interactions: usize,
    /// Number of unique participants
    pub unique_participants: usize,
}

impl TimelineAnalysis {
    /// Creates a new, empty timeline analysis
    pub fn new(start_time: DateTime<Utc>, end_time: DateTime<Utc>) -> Self {
        TimelineAnalysis {
            patterns: Vec::new(),
            density: TimelineDensity {
                avg_interactions_per_day: 0.0,
                peak_hours: Vec::new(),
                peak_days: Vec::new(),
                peak_hour: 0,
                peak_day: 0,
            },
            response_times: ResponseTimeStats {
                average: 0.0,
                median: 0.0,
                percentiles: HashMap::new(),
                min: 0.0,
                max: 0.0,
            },
            start_time,
            end_time,
            total_interactions: 0,
            unique_participants: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Duration};

    #[test]
    fn test_timeline_analysis_creation() {
        let start = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
        let end = start + Duration::days(7);
        let analysis = TimelineAnalysis::new(start, end);
        
        assert_eq!(analysis.patterns.len(), 0);
        assert_eq!(analysis.density.avg_interactions_per_day, 0.0);
        assert_eq!(analysis.total_interactions, 0);
        assert_eq!(analysis.unique_participants, 0);
    }
}
