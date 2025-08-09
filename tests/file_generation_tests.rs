use tweet_scrolls::relationship::file_generation::{LLMFileGenerator, generate_profile_text, generate_timeline_text, generate_llm_analysis_prompts};
use tweet_scrolls::models::profile::UserProfile;
use tweet_scrolls::models::interaction::InteractionEvent;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use tempfile::tempdir;

#[cfg(test)]
mod file_generation_tests {
    use super::*;

    fn create_sample_user_profile() -> UserProfile {
    let user_id = "1132151165410455552".to_string();
    let mut profile = UserProfile::new(user_id.clone());
        profile.total_interactions = 25;
        profile.first_interaction = Some("2023-01-01T10:00:00Z".parse::<DateTime<Utc>>().unwrap());
        profile.last_interaction = Some("2023-12-31T15:30:00Z".parse::<DateTime<Utc>>().unwrap());
        profile.interaction_counts = HashMap::from([
            ("dm_sent".to_string(), 12),
            ("dm_received".to_string(), 8),
            ("tweet_reply".to_string(), 5),
        ]);
        profile.metadata = HashMap::from([
            ("analysis_version".to_string(), "1.0".to_string()),
            ("data_source".to_string(), "twitter_export".to_string()),
        ]);
        profile
    }

    fn create_sample_interactions() -> Vec<InteractionEvent> {
        vec![
            InteractionEvent::new(
                "msg_001",
                "2023-06-15T10:30:00Z".parse::<DateTime<Utc>>().unwrap(),
                tweet_scrolls::models::interaction::InteractionType::DmSent,
                "user1".to_string(),
                "Hey, how are you doing?"
            ).with_metadata("conversation_id", "user1-user2"),
            InteractionEvent::new(
                "msg_002", 
                "2023-06-15T14:45:00Z".parse::<DateTime<Utc>>().unwrap(),
                tweet_scrolls::models::interaction::InteractionType::DmReceived,
                "user2".to_string(),
                "I'm doing great! Thanks for asking."
            ).with_metadata("conversation_id", "user1-user2"),
        ]
    }

    #[test]
    fn test_generate_user_profile_text() {
        let profile = create_sample_user_profile();
        let profile_text = generate_profile_text(&profile);
        
        assert!(profile_text.contains("USER RELATIONSHIP PROFILE"));
        assert!(profile_text.contains("COMMUNICATION STATISTICS"));
        assert!(profile_text.contains("TEMPORAL PATTERNS"));
        assert!(profile_text.contains("Total Interactions: 25"));
        assert!(profile_text.contains("dm_sent: 12 interactions"));
        assert!(profile_text.contains("dm_received: 8 interactions"));
        assert!(profile_text.contains("tweet_reply: 5 interactions"));
        assert!(profile_text.contains("2023-01-01"));
        assert!(profile_text.contains("2023-12-31"));
    }

    #[test]
    fn test_generate_timeline_text() {
        let interactions = create_sample_interactions();
        let timeline_text = generate_timeline_text(&interactions);
        
        assert!(timeline_text.contains("INTERACTION TIMELINE"));
        assert!(timeline_text.contains("2023-06-15"));
        assert!(timeline_text.contains("DmSent"));
        assert!(timeline_text.contains("DmReceived"));
        assert!(timeline_text.contains("Hey, how are you doing?"));
        assert!(timeline_text.contains("I'm doing great! Thanks for asking."));
        assert!(timeline_text.contains("msg_001"));
        assert!(timeline_text.contains("msg_002"));
    }

    #[test]
    fn test_llm_file_generator_creation() {
        let temp_dir = tempdir().unwrap();
        let output_path = temp_dir.path().to_str().unwrap();
        
        let generator = LLMFileGenerator::new(output_path, "testuser", 1234567890);
        
        assert_eq!(generator.screen_name, "testuser");
        assert_eq!(generator.timestamp, 1234567890);
        assert!(generator.output_dir.contains("testuser"));
    }

    #[test]
    fn test_generate_individual_profile_file() {
        let temp_dir = tempdir().unwrap();
        let output_path = temp_dir.path().to_str().unwrap();
        let generator = LLMFileGenerator::new(output_path, "testuser", 1234567890);
        
        let profile = create_sample_user_profile();
        let result = generator.generate_individual_profile_file(&profile);
        
        assert!(result.is_ok());
        
        // Verify file was created
        let expected_filename = format!("user_{}_profile.txt", &profile.user_id);
        let file_path = std::path::Path::new(output_path)
            .join("relationship_profiles_testuser_1234567890")
            .join(&expected_filename);
        println!("Checking file path: {:?}", file_path);
        assert!(file_path.exists());
    }

    #[test]
    fn test_generate_llm_analysis_prompts() {
        let profiles = vec![create_sample_user_profile()];
        let prompts = generate_llm_analysis_prompts(&profiles);
        
        // Check for key sections that should be in the prompts
        assert!(prompts.contains("LLM ANALYSIS PROMPTS"));
        assert!(prompts.contains("RELATIONSHIP HEALTH ANALYSIS"));
        assert!(prompts.contains("Which relationships need more attention"));
        assert!(prompts.contains("communication patterns"));
        assert!(prompts.contains("TOTAL RELATIONSHIPS ANALYZED: 1"));
        assert!(prompts.len() > 500); // Should be a substantial prompt file
    }

    #[test]
    fn test_file_output_structure() {
        let temp_dir = tempdir().unwrap();
        let output_path = temp_dir.path().to_str().unwrap();
        let generator = LLMFileGenerator::new(output_path, "testuser", 1234567890);
        
        let profiles = vec![create_sample_user_profile()];
        let interactions = create_sample_interactions();
        
        let result = generator.generate_all_files(&profiles, &interactions);
        assert!(result.is_ok());
        
        // Verify directory structure
        let base_dir = std::path::Path::new(output_path).join("relationship_profiles_testuser_1234567890");
        assert!(base_dir.exists());
        
        // Verify key files exist
        assert!(base_dir.join("interaction_timeline.txt").exists());
        assert!(base_dir.join("communication_patterns.txt").exists());
        assert!(base_dir.join("relationship_network.txt").exists());
        assert!(base_dir.join("llm_analysis_prompts.txt").exists());
    }
}