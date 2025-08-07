//! File writing service for relationship intelligence files
//! 
//! Handles directory creation and file I/O operations.

use anyhow::{Context, Result};
use std::collections::HashMap;
use tokio::fs as async_fs;

use crate::models::profile::UserProfile;
use crate::models::interaction::InteractionEvent;
use super::text_generators::generate_user_profile_text;
use super::timeline_text::generate_timeline_text;
use super::prompts_generator::generate_llm_analysis_prompts;

/// File writing service for relationship intelligence
pub struct FileWriter {
    output_dir: String,
    screen_name: String,
    timestamp: i64,
}

impl FileWriter {
    /// Creates a new file writer
    pub fn new(output_dir: impl Into<String>, screen_name: impl Into<String>, timestamp: i64) -> Self {
        Self {
            output_dir: output_dir.into(),
            screen_name: screen_name.into(),
            timestamp,
        }
    }

    /// Creates the relationship profiles directory structure
    pub async fn create_directory_structure(&self) -> Result<String> {
        let profiles_dir = format!("{}/relationship_profiles_{}_{}", 
                                 self.output_dir, self.screen_name, self.timestamp);
        
        async_fs::create_dir_all(&profiles_dir).await
            .with_context(|| format!("Failed to create directory: {}", profiles_dir))?;
        
        Ok(profiles_dir)
    }

    /// Writes all relationship intelligence files
    pub async fn write_all_files(&self, profiles: &HashMap<String, UserProfile>, timeline: &[InteractionEvent]) -> Result<()> {
        let profiles_dir = self.create_directory_structure().await?;
        
        self.write_user_profiles(&profiles_dir, profiles, timeline).await?;
        self.write_timeline_file(&profiles_dir, timeline).await?;
        self.write_prompts_file(&profiles_dir, profiles).await?;
        self.write_summary_file(&profiles_dir, profiles, timeline).await?;
        
        println!("✅ LLM-ready relationship intelligence files generated in: {}", profiles_dir);
        Ok(())
    }

    /// Writes individual user profile files
    async fn write_user_profiles(&self, profiles_dir: &str, profiles: &HashMap<String, UserProfile>, timeline: &[InteractionEvent]) -> Result<()> {
        for (user_hash, profile) in profiles {
            let user_timeline: Vec<_> = timeline.iter()
                .filter(|event| event.user_hash == *user_hash)
                .cloned()
                .collect();
            
            let profile_text = generate_user_profile_text(profile, &user_timeline);
            let profile_file = format!("{}/user_{}_profile.txt", profiles_dir, &user_hash[..16]);
            
            async_fs::write(&profile_file, profile_text).await
                .with_context(|| format!("Failed to write profile file: {}", profile_file))?;
        }
        Ok(())
    } 
   /// Writes interaction timeline file
    async fn write_timeline_file(&self, profiles_dir: &str, timeline: &[InteractionEvent]) -> Result<()> {
        let timeline_text = generate_timeline_text(timeline);
        let timeline_file = format!("{}/interaction_timeline.txt", profiles_dir);
        async_fs::write(&timeline_file, timeline_text).await
            .with_context(|| format!("Failed to write timeline file: {}", timeline_file))?;
        Ok(())
    }

    /// Writes LLM analysis prompts file
    async fn write_prompts_file(&self, profiles_dir: &str, profiles: &HashMap<String, UserProfile>) -> Result<()> {
        let prompts_text = generate_llm_analysis_prompts(profiles);
        let prompts_file = format!("{}/llm_analysis_prompts.txt", profiles_dir);
        async_fs::write(&prompts_file, prompts_text).await
            .with_context(|| format!("Failed to write prompts file: {}", prompts_file))?;
        Ok(())
    }

    /// Writes summary file
    async fn write_summary_file(&self, profiles_dir: &str, profiles: &HashMap<String, UserProfile>, timeline: &[InteractionEvent]) -> Result<()> {
        let summary_text = self.generate_summary_text(profiles, timeline);
        let summary_file = format!("{}/relationship_intelligence_summary.txt", profiles_dir);
        async_fs::write(&summary_file, summary_text).await
            .with_context(|| format!("Failed to write summary file: {}", summary_file))?;
        Ok(())
    }

    /// Generates summary file content
    fn generate_summary_text(&self, profiles: &HashMap<String, UserProfile>, timeline: &[InteractionEvent]) -> String {
        let mut output = String::new();
        
        output.push_str("RELATIONSHIP INTELLIGENCE SUMMARY\n");
        output.push_str("=================================\n");
        output.push_str(&format!("Generated: {}\n", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")));
        output.push_str(&format!("Screen Name: {}\n\n", self.screen_name));
        
        self.add_overview_statistics(&mut output, profiles, timeline);
        self.add_top_relationships(&mut output, profiles);
        self.add_files_info(&mut output);
        self.add_usage_instructions(&mut output);
        
        output
    }

    /// Adds overview statistics to summary
    fn add_overview_statistics(&self, output: &mut String, profiles: &HashMap<String, UserProfile>, timeline: &[InteractionEvent]) {
        output.push_str("OVERVIEW STATISTICS\n");
        output.push_str("==================\n");
        output.push_str(&format!("Total relationships analyzed: {}\n", profiles.len()));
        output.push_str(&format!("Total interaction events: {}\n", timeline.len()));
        
        let total_interactions: u32 = profiles.values().map(|p| p.total_interactions).sum();
        output.push_str(&format!("Total interactions across all relationships: {}\n", total_interactions));
        
        if !profiles.is_empty() {
            let avg_interactions = total_interactions as f64 / profiles.len() as f64;
            output.push_str(&format!("Average interactions per relationship: {:.1}\n\n", avg_interactions));
        }
    }

    /// Adds top relationships to summary
    fn add_top_relationships(&self, output: &mut String, profiles: &HashMap<String, UserProfile>) {
        output.push_str("TOP RELATIONSHIPS (by interaction count)\n");
        output.push_str("=======================================\n");
        
        let mut sorted_profiles: Vec<_> = profiles.iter().collect();
        sorted_profiles.sort_by(|a, b| b.1.total_interactions.cmp(&a.1.total_interactions));
        
        for (i, (user_hash, profile)) in sorted_profiles.iter().take(10).enumerate() {
            output.push_str(&format!("{}. {} - {} interactions\n", 
                                   i + 1, &user_hash[..16], profile.total_interactions));
        }
        
        output.push('\n');
    }

    /// Adds files information to summary
    fn add_files_info(&self, output: &mut String) {
        output.push_str("FILES GENERATED\n");
        output.push_str("==============\n");
        output.push_str("- user_[hash]_profile.txt - Individual relationship profiles\n");
        output.push_str("- interaction_timeline.txt - Chronological interaction log\n");
        output.push_str("- llm_analysis_prompts.txt - Suggested analysis questions\n");
        output.push_str("- relationship_intelligence_summary.txt - This overview file\n\n");
    }

    /// Adds usage instructions to summary
    fn add_usage_instructions(&self, output: &mut String) {
        output.push_str("USAGE INSTRUCTIONS\n");
        output.push_str("==================\n");
        output.push_str("1. Upload these files to your preferred LLM (ChatGPT, Claude, NotebookLM)\n");
        output.push_str("2. Use the suggested prompts in llm_analysis_prompts.txt\n");
        output.push_str("3. Focus on patterns and insights for relationship improvement\n");
        output.push_str("4. All user data has been anonymized for privacy\n");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use crate::models::interaction::InteractionType;
    use chrono::TimeZone;

    fn create_test_profile() -> UserProfile {
        let mut profile = UserProfile::new("test_user_hash_123456");
        profile.total_interactions = 42;
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
        ]
    }

    #[tokio::test]
    async fn test_create_directory_structure() {
        let temp_dir = tempdir().unwrap();
        let writer = FileWriter::new(temp_dir.path().to_str().unwrap(), "testuser", 1234567890);
        
        let profiles_dir = writer.create_directory_structure().await.unwrap();
        
        assert!(tokio::fs::metadata(&profiles_dir).await.is_ok());
        assert!(profiles_dir.contains("relationship_profiles_testuser_1234567890"));
    }

    #[tokio::test]
    async fn test_write_all_files() {
        let temp_dir = tempdir().unwrap();
        let writer = FileWriter::new(temp_dir.path().to_str().unwrap(), "testuser", 1234567890);
        
        let mut profiles = HashMap::new();
        profiles.insert("test_user_hash_123456".to_string(), create_test_profile());
        let timeline = create_test_timeline();
        
        let result = writer.write_all_files(&profiles, &timeline).await;
        assert!(result.is_ok());
    }
}