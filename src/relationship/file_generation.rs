//! LLM-ready file generation orchestrator
//! 
//! Minimal orchestration layer that delegates to specialized modules.

use anyhow::Result;
use std::collections::HashMap;

use crate::models::profile::UserProfile;
use crate::models::interaction::InteractionEvent;
use super::file_writer::FileWriter;
use super::text_generators::generate_user_profile_text;
use super::timeline_text::generate_timeline_text;
use super::prompts_generator::generate_llm_analysis_prompts;

/// LLM file generator - minimal orchestrator
pub struct LLMFileGenerator {
    file_writer: FileWriter,
}

impl LLMFileGenerator {
    /// Creates a new LLM file generator
    pub fn new(output_dir: impl Into<String>, screen_name: impl Into<String>, timestamp: i64) -> Self {
        Self {
            file_writer: FileWriter::new(output_dir, screen_name, timestamp),
        }
    }

    /// Creates directory structure (delegates to file writer)
    pub async fn create_directory_structure(&self) -> Result<String> {
        self.file_writer.create_directory_structure().await
    }

    /// Generates user profile text (delegates to text generator)
    pub fn generate_user_profile_text(&self, profile: &UserProfile, timeline: &[InteractionEvent]) -> String {
        generate_user_profile_text(profile, timeline)
    }

    /// Generates timeline text (delegates to timeline generator)
    pub fn generate_timeline_text(&self, timeline: &[InteractionEvent]) -> String {
        generate_timeline_text(timeline)
    }

    /// Generates LLM analysis prompts (delegates to prompts generator)
    pub fn generate_llm_analysis_prompts(&self, profiles: &HashMap<String, UserProfile>) -> String {
        generate_llm_analysis_prompts(profiles)
    }

    /// Writes all files (delegates to file writer)
    pub async fn write_all_files(&self, profiles: &HashMap<String, UserProfile>, timeline: &[InteractionEvent]) -> Result<()> {
        self.file_writer.write_all_files(profiles, timeline).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::interaction::InteractionType;
    use chrono::TimeZone;
    use tempfile::tempdir;

    fn create_test_profile() -> UserProfile {
        let mut profile = UserProfile::new("test_user_hash_123456");
        profile.total_interactions = 42;
        profile.first_interaction = Some(chrono::Utc.with_ymd_and_hms(2023, 1, 1, 10, 0, 0).unwrap());
        profile.last_interaction = Some(chrono::Utc.with_ymd_and_hms(2023, 12, 31, 15, 30, 0).unwrap());
        profile.interaction_counts.insert("dm_messages".to_string(), 25);
        profile.interaction_counts.insert("dm_received".to_string(), 17);
        profile
    }

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
    fn test_llm_file_generator_creation() {
        let generator = LLMFileGenerator::new("/tmp/test", "testuser", 1234567890);
        // Test that generator can be created successfully
        // Internal fields are private, so we just test creation doesn't panic
        assert!(true);
    }

    #[test]
    fn test_generate_user_profile_text() {
        let generator = LLMFileGenerator::new("/tmp/test", "testuser", 1234567890);
        let profile = create_test_profile();
        let timeline = create_test_timeline();
        
        let profile_text = generator.generate_user_profile_text(&profile, &timeline);
        
        // Verify essential sections are present
        assert!(profile_text.contains("USER RELATIONSHIP PROFILE"));
        assert!(profile_text.contains("COMMUNICATION STATISTICS"));
        assert!(profile_text.contains("TEMPORAL PATTERNS"));
        assert!(profile_text.contains("RELATIONSHIP INSIGHTS"));
        
        // Verify data is included
        assert!(profile_text.contains("test_user_hash_123456"));
        assert!(profile_text.contains("Total Interactions: 42"));
        assert!(profile_text.contains("dm_messages: 25"));
        assert!(profile_text.contains("First Interaction: 2023-01-01"));
        assert!(profile_text.contains("Last Interaction: 2023-12-31"));
    }

    #[test]
    fn test_generate_timeline_text() {
        let generator = LLMFileGenerator::new("/tmp/test", "testuser", 1234567890);
        let timeline = create_test_timeline();
        
        let timeline_text = generator.generate_timeline_text(&timeline);
        
        // Verify essential sections are present
        assert!(timeline_text.contains("CHRONOLOGICAL INTERACTION LOG"));
        assert!(timeline_text.contains("MONTHLY ACTIVITY SUMMARY"));
        assert!(timeline_text.contains("RECENT ACTIVITY"));
        
        // Verify data is included
        assert!(timeline_text.contains("Total Events: 2"));
        assert!(timeline_text.contains("2023-06"));
        assert!(timeline_text.contains("DmSent"));
        assert!(timeline_text.contains("DmReceived"));
    }

    #[test]
    fn test_generate_llm_analysis_prompts() {
        let generator = LLMFileGenerator::new("/tmp/test", "testuser", 1234567890);
        let mut profiles = HashMap::new();
        profiles.insert("user1".to_string(), create_test_profile());
        
        let prompts = generator.generate_llm_analysis_prompts(&profiles);
        
        // Verify essential prompts are present
        assert!(prompts.contains("Which relationships need more attention"));
        assert!(prompts.contains("What communication patterns make conversations most engaging"));
        assert!(prompts.contains("Who are the most important people in this social network"));
        assert!(prompts.contains("What do the temporal patterns reveal about communication habits"));
        
        // Verify context information
        assert!(prompts.contains("Total relationships analyzed: 1"));
        assert!(prompts.contains("Blake3 hashing for privacy"));
    }

    #[tokio::test]
    async fn test_create_directory_structure() {
        let temp_dir = tempdir().unwrap();
        let generator = LLMFileGenerator::new(temp_dir.path().to_str().unwrap(), "testuser", 1234567890);
        
        let profiles_dir = generator.create_directory_structure().await.unwrap();
        
        // Verify directory was created
        assert!(tokio::fs::metadata(&profiles_dir).await.is_ok());
        assert!(profiles_dir.contains("relationship_profiles_testuser_1234567890"));
    }

    #[tokio::test]
    async fn test_write_all_files() {
        let temp_dir = tempdir().unwrap();
        let generator = LLMFileGenerator::new(temp_dir.path().to_str().unwrap(), "testuser", 1234567890);
        
        let mut profiles = HashMap::new();
        profiles.insert("test_user_hash_123456".to_string(), create_test_profile());
        let timeline = create_test_timeline();
        
        let result = generator.write_all_files(&profiles, &timeline).await;
        assert!(result.is_ok());
        
        // Verify files were created
        let profiles_dir = format!("{}/relationship_profiles_testuser_1234567890", temp_dir.path().display());
        
        let profile_file = format!("{}/user_test_user_hash_1_profile.txt", profiles_dir);
        assert!(tokio::fs::metadata(&profile_file).await.is_ok());
        
        let timeline_file = format!("{}/interaction_timeline.txt", profiles_dir);
        assert!(tokio::fs::metadata(&timeline_file).await.is_ok());
        
        let prompts_file = format!("{}/llm_analysis_prompts.txt", profiles_dir);
        assert!(tokio::fs::metadata(&prompts_file).await.is_ok());
        
        let summary_file = format!("{}/relationship_intelligence_summary.txt", profiles_dir);
        assert!(tokio::fs::metadata(&summary_file).await.is_ok());
    }

    #[test]
    fn test_relationship_strength_calculation() {
        let generator = LLMFileGenerator::new("/tmp/test", "testuser", 1234567890);
        
        // Test high interaction profile
        let mut high_profile = UserProfile::new("high_user");
        high_profile.total_interactions = 150;
        let high_text = generator.generate_user_profile_text(&high_profile, &[]);
        assert!(high_text.contains("Relationship strength: High"));
        
        // Test medium interaction profile
        let mut medium_profile = UserProfile::new("medium_user");
        medium_profile.total_interactions = 50;
        let medium_text = generator.generate_user_profile_text(&medium_profile, &[]);
        assert!(medium_text.contains("Relationship strength: Medium"));
        
        // Test low interaction profile
        let mut low_profile = UserProfile::new("low_user");
        low_profile.total_interactions = 10;
        let low_text = generator.generate_user_profile_text(&low_profile, &[]);
        assert!(low_text.contains("Relationship strength: Low"));
        
        // Test minimal interaction profile
        let mut minimal_profile = UserProfile::new("minimal_user");
        minimal_profile.total_interactions = 2;
        let minimal_text = generator.generate_user_profile_text(&minimal_profile, &[]);
        assert!(minimal_text.contains("Relationship strength: Minimal"));
    }
}