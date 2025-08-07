//! Utility functions and helpers

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::Path;

/// Reads a file into a string with proper error context
pub fn read_file_to_string<P: AsRef<Path>>(path: P) -> Result<String> {
    let file = File::open(&path).with_context(|| format!("Failed to open file: {:?}", path.as_ref()))?;
    let mut reader = BufReader::new(file);
    let mut content = String::new();
    reader
        .read_to_string(&mut content)
        .with_context(|| format!("Failed to read file: {:?}", path.as_ref()))?;
    Ok(content)
}

/// Formats a duration in a human-readable way
pub fn format_duration(duration: chrono::Duration) -> String {
    let secs = duration.num_seconds();
    if secs < 60 {
        format!("{}s", secs)
    } else if secs < 3600 {
        format!("{}m {}s", secs / 60, secs % 60)
    } else if secs < 86400 {
        format!("{}h {}m", secs / 3600, (secs % 3600) / 60)
    } else {
        format!("{}d {}h", secs / 86400, (secs % 86400) / 3600)
    }
}

/// Formats a timestamp in a human-readable relative format
pub fn format_timestamp(timestamp: &DateTime<Utc>) -> String {
    let now = Utc::now();
    let duration = now.signed_duration_since(*timestamp);
    
    if duration.num_seconds() < 60 {
        "just now".to_string()
    } else if duration.num_minutes() < 60 {
        format!("{}m ago", duration.num_minutes())
    } else if duration.num_hours() < 24 {
        format!("{}h ago", duration.num_hours())
    } else if duration.num_days() < 30 {
        format!("{}d ago", duration.num_days())
    } else {
        timestamp.format("%b %d, %Y").to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, TimeZone, Utc};
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_read_file_to_string() -> Result<()> {
        let mut temp_file = NamedTempFile::new()?;
        write!(temp_file, "test content")?;
        let content = read_file_to_string(temp_file.path())?;
        assert_eq!(content, "test content");
        Ok(())
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(Duration::seconds(30)), "30s");
        assert_eq!(format_duration(Duration::minutes(5) + Duration::seconds(30)), "5m 30s");
        assert_eq!(format_duration(Duration::hours(2) + Duration::minutes(30)), "2h 30m");
        assert_eq!(format_duration(Duration::days(3) + Duration::hours(5)), "3d 5h");
    }

    #[test]
    fn test_format_timestamp() {
        let now = Utc::now();
        let one_min_ago = now - Duration::minutes(1);
        let one_hour_ago = now - Duration::hours(1);
        let one_day_ago = now - Duration::days(1);
        let one_month_ago = now - Duration::days(35);
        
        assert_eq!(format_timestamp(&now), "just now");
        assert!(format_timestamp(&one_min_ago).ends_with("m ago"));
        assert!(format_timestamp(&one_hour_ago).ends_with("h ago"));
        assert!(format_timestamp(&one_day_ago).ends_with("d ago"));
        assert!(format_timestamp(&one_month_ago).contains(", 20"));
    }
}
