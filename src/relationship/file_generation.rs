use crate::models::profile::UserProfile;
use crate::models::interaction::InteractionEvent;
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

/// LLM File Generator for relationship intelligence profiles
pub struct LLMFileGenerator {
    pub output_dir: String,
    pub screen_name: String,
    pub timestamp: u64,
}

impl LLMFileGenerator {
    /// Create a new LLM file generator
    pub fn new(base_path: &str, screen_name: &str, timestamp: u64) -> Self {
        let output_dir = format!("{}/relationship_profiles_{}_{}", base_path, screen_name, timestamp);
        
        LLMFileGenerator {
            output_dir,
            screen_name: screen_name.to_string(),
            timestamp,
        }
    }

    /// Generate all relationship intelligence files
    pub fn generate_all_files(&self, profiles: &[UserProfile], interactions: &[InteractionEvent]) -> Result<()> {
        // Create output directory
        fs::create_dir_all(&self.output_dir)
            .context("Failed to create relationship profiles directory")?;

        // Generate individual profile files
        for profile in profiles {
            self.generate_individual_profile_file(profile)?;
        }

        // Generate aggregate files
        self.generate_interaction_timeline_file(interactions)?;
        self.generate_communication_patterns_file(profiles)?;
        self.generate_relationship_network_file(profiles)?;
        self.generate_llm_prompts_file(profiles)?;

        Ok(())
    }

    /// Generate individual user profile file
    pub fn generate_individual_profile_file(&self, profile: &UserProfile) -> Result<()> {
        // Create output directory if it doesn't exist
        fs::create_dir_all(&self.output_dir)
            .context("Failed to create output directory")?;
            
        let filename = format!("user_{}_profile.txt", &profile.user_hash[..8]);
        let file_path = Path::new(&self.output_dir).join(filename);
        
        let content = generate_profile_text(profile);
        
        fs::write(file_path, content)
            .context("Failed to write individual profile file")?;
        
        Ok(())
    }

    /// Generate interaction timeline file
    fn generate_interaction_timeline_file(&self, interactions: &[InteractionEvent]) -> Result<()> {
        let file_path = Path::new(&self.output_dir).join("interaction_timeline.txt");
        let content = generate_timeline_text(interactions);
        
        fs::write(file_path, content)
            .context("Failed to write interaction timeline file")?;
        
        Ok(())
    }

    /// Generate communication patterns file
    fn generate_communication_patterns_file(&self, profiles: &[UserProfile]) -> Result<()> {
        let file_path = Path::new(&self.output_dir).join("communication_patterns.txt");
        let content = generate_communication_patterns_text(profiles);
        
        fs::write(file_path, content)
            .context("Failed to write communication patterns file")?;
        
        Ok(())
    }

    /// Generate relationship network file
    fn generate_relationship_network_file(&self, profiles: &[UserProfile]) -> Result<()> {
        let file_path = Path::new(&self.output_dir).join("relationship_network.txt");
        let content = generate_relationship_network_text(profiles);
        
        fs::write(file_path, content)
            .context("Failed to write relationship network file")?;
        
        Ok(())
    }

    /// Generate LLM analysis prompts file
    fn generate_llm_prompts_file(&self, profiles: &[UserProfile]) -> Result<()> {
        let file_path = Path::new(&self.output_dir).join("llm_analysis_prompts.txt");
        let content = generate_llm_analysis_prompts(profiles);
        
        fs::write(file_path, content)
            .context("Failed to write LLM analysis prompts file")?;
        
        Ok(())
    }
}

/// Generate formatted profile text for a user
pub fn generate_profile_text(profile: &UserProfile) -> String {
    let first_interaction_str = profile.first_interaction
        .map(|dt| dt.format("%Y-%m-%d").to_string())
        .unwrap_or_else(|| "Unknown".to_string());
    
    let last_interaction_str = profile.last_interaction
        .map(|dt| dt.format("%Y-%m-%d").to_string())
        .unwrap_or_else(|| "Unknown".to_string());
    
    let first_interaction_full = profile.first_interaction
        .map(|dt| dt.format("%Y-%m-%d %H:%M UTC").to_string())
        .unwrap_or_else(|| "Unknown".to_string());
    
    let last_interaction_full = profile.last_interaction
        .map(|dt| dt.format("%Y-%m-%d %H:%M UTC").to_string())
        .unwrap_or_else(|| "Unknown".to_string());

    format!(
        r#"# USER RELATIONSHIP PROFILE

## BASIC INFORMATION
- User Hash: {}
- Analysis Period: {} to {}
- Total Interactions: {}

## COMMUNICATION STATISTICS
- Total Interaction Types: {}
- Interaction Breakdown: {}

## TEMPORAL PATTERNS
- First Interaction: {}
- Last Interaction: {}

## INTERACTION BREAKDOWN
{}

## RELATIONSHIP INSIGHTS
This user has {} total interactions across {} different interaction types.
Profile shows activity from {} to {}.

## METADATA
{}

---
Generated by Tweet-Scrolls Relationship Intelligence System
"#,
        profile.user_hash,
        first_interaction_str,
        last_interaction_str,
        profile.total_interactions,
        profile.interaction_counts.len(),
        format_interaction_counts(&profile.interaction_counts),
        first_interaction_full,
        last_interaction_full,
        format_interaction_counts(&profile.interaction_counts),
        profile.total_interactions,
        profile.interaction_counts.len(),
        first_interaction_str,
        last_interaction_str,
        format_metadata(&profile.metadata)
    )
}

/// Generate formatted timeline text for interactions
pub fn generate_timeline_text(interactions: &[InteractionEvent]) -> String {
    let mut content = String::from("# INTERACTION TIMELINE\n\n");
    content.push_str("Chronological log of all interactions for relationship analysis.\n\n");
    
    for interaction in interactions {
        content.push_str(&format!(
            "## {} - {:?}\n",
            interaction.timestamp.format("%Y-%m-%d %H:%M UTC"),
            interaction.interaction_type
        ));
        
        content.push_str(&format!("- User: {}\n", &interaction.user_hash[..8]));
        content.push_str(&format!("- ID: {}\n", interaction.id));
        
        if !interaction.content.is_empty() {
            let preview = if interaction.content.len() > 100 {
                format!("{}...", &interaction.content[..100])
            } else {
                interaction.content.clone()
            };
            content.push_str(&format!("- Content: {}\n", preview));
        }
        
        if !interaction.metadata.is_empty() {
            content.push_str("- Metadata:\n");
            for (key, value) in &interaction.metadata {
                content.push_str(&format!("  - {}: {}\n", key, value));
            }
        }
        
        content.push_str("\n");
    }
    
    content.push_str("\n---\nGenerated by Tweet-Scrolls Relationship Intelligence System\n");
    content
}

/// Generate LLM analysis prompts
pub fn generate_llm_analysis_prompts(profiles: &[UserProfile]) -> String {
    format!(
        r#"# LLM ANALYSIS PROMPTS FOR RELATIONSHIP INTELLIGENCE

## OVERVIEW
This file contains suggested prompts for analyzing relationship data with Large Language Models.
Use these prompts with the generated profile files to gain deeper insights into communication patterns.

## RELATIONSHIP HEALTH ANALYSIS

### Primary Questions
1. Which relationships need more attention based on interaction frequency and response times?
2. What communication patterns make conversations most engaging and meaningful?
3. How can I improve my response times without sacrificing thoughtfulness?
4. Which relationships show the strongest mutual engagement patterns?
5. What temporal patterns suggest optimal times for important conversations?

### Deep Analysis Prompts
1. **Communication Balance**: "Analyze the send/receive ratio for each relationship. Which relationships are one-sided and might benefit from more balanced communication?"

2. **Response Time Patterns**: "Examine response time data across relationships. What patterns emerge for different types of relationships (close friends, professional contacts, family)?"

3. **Temporal Insights**: "Based on the most active hours and days for each relationship, when should I schedule important conversations for maximum engagement?"

4. **Relationship Prioritization**: "Using interaction frequency, response times, and engagement patterns, rank relationships by priority for attention and nurturing."

5. **Communication Style Analysis**: "What do the interaction types (DMs, replies, mentions) reveal about communication preferences for each relationship?"

## ACTIONABLE INSIGHTS PROMPTS

### Relationship Improvement
- "Identify 3 relationships that would benefit most from increased attention based on declining interaction patterns."
- "Suggest specific actions to improve response times while maintaining message quality."
- "Recommend optimal communication schedules based on each person's activity patterns."

### Communication Optimization
- "Analyze which communication channels (DMs vs public replies) work best for different relationships."
- "Identify conversation starters that historically lead to longer, more engaging exchanges."
- "Suggest ways to maintain consistent communication without being overwhelming."

### Network Analysis
- "Map the relationship network to identify key connectors and potential introductions."
- "Analyze communication clusters to understand social groups and dynamics."
- "Identify relationships that could benefit from group interactions vs one-on-one communication."

## SAMPLE ANALYSIS FRAMEWORK

When analyzing the relationship data, consider these dimensions:

1. **Frequency**: How often do we interact?
2. **Consistency**: Are interactions regular or sporadic?
3. **Reciprocity**: Is communication balanced between both parties?
4. **Responsiveness**: How quickly do we respond to each other?
5. **Engagement**: Do conversations lead to meaningful exchanges?
6. **Temporal Patterns**: When are we most likely to have quality interactions?

## PRIVACY CONSIDERATIONS

All user identifiers have been anonymized using Blake3 hashing. When discussing insights:
- Refer to users by their hash prefixes (first 8 characters)
- Focus on patterns rather than specific content
- Maintain confidentiality of communication details

## TOTAL RELATIONSHIPS ANALYZED: {}

Use these prompts to generate actionable insights for improving relationship management and communication effectiveness.

---
Generated by Tweet-Scrolls Relationship Intelligence System
Analysis Date: {}
"#,
        profiles.len(),
        chrono::Utc::now().format("%Y-%m-%d %H:%M UTC")
    )
}

/// Format interaction counts for display
fn format_interaction_counts(counts: &std::collections::HashMap<String, u32>) -> String {
    let mut formatted = String::new();
    for (interaction_type, count) in counts {
        formatted.push_str(&format!("- {}: {} interactions\n", interaction_type, count));
    }
    if formatted.is_empty() {
        formatted.push_str("- No interactions recorded\n");
    }
    formatted
}

/// Format metadata for display
fn format_metadata(metadata: &std::collections::HashMap<String, String>) -> String {
    let mut formatted = String::new();
    for (key, value) in metadata {
        formatted.push_str(&format!("- {}: {}\n", key, value));
    }
    if formatted.is_empty() {
        formatted.push_str("- No metadata available\n");
    }
    formatted
}

/// Generate communication patterns analysis text
fn generate_communication_patterns_text(profiles: &[UserProfile]) -> String {
    let mut content = String::from("# COMMUNICATION PATTERNS ANALYSIS\n\n");
    
    // Calculate aggregate statistics
    let total_interactions: u32 = profiles.iter().map(|p| p.total_interactions).sum();
    
    content.push_str(&format!("## AGGREGATE STATISTICS\n"));
    content.push_str(&format!("- Total Relationships: {}\n", profiles.len()));
    content.push_str(&format!("- Total Interactions: {}\n", total_interactions));
    
    if !profiles.is_empty() {
        let avg_interactions = total_interactions as f64 / profiles.len() as f64;
        content.push_str(&format!("- Average Interactions per Relationship: {:.1}\n\n", avg_interactions));
    }
    
    // Interaction type analysis
    let mut all_interaction_types = std::collections::HashMap::new();
    for profile in profiles {
        for (interaction_type, count) in &profile.interaction_counts {
            *all_interaction_types.entry(interaction_type.clone()).or_insert(0) += count;
        }
    }
    
    content.push_str("## INTERACTION TYPE DISTRIBUTION\n");
    let mut sorted_types: Vec<_> = all_interaction_types.iter().collect();
    sorted_types.sort_by(|a, b| b.1.cmp(a.1));
    
    for (interaction_type, count) in sorted_types.iter().take(10) {
        content.push_str(&format!("- {}: {} total interactions\n", interaction_type, count));
    }
    
    content.push_str("\n---\nGenerated by Tweet-Scrolls Relationship Intelligence System\n");
    content
}

/// Generate relationship network analysis text
fn generate_relationship_network_text(profiles: &[UserProfile]) -> String {
    let mut content = String::from("# RELATIONSHIP NETWORK ANALYSIS\n\n");
    
    content.push_str("## NETWORK OVERVIEW\n");
    content.push_str(&format!("- Total Nodes (Relationships): {}\n", profiles.len()));
    
    // Categorize relationships by total interactions
    let high_activity = profiles.iter().filter(|p| p.total_interactions > 50).count();
    let medium_activity = profiles.iter().filter(|p| p.total_interactions > 10 && p.total_interactions <= 50).count();
    let low_activity = profiles.iter().filter(|p| p.total_interactions <= 10).count();
    
    content.push_str(&format!("- High Activity Relationships (>50 interactions): {}\n", high_activity));
    content.push_str(&format!("- Medium Activity Relationships (10-50 interactions): {}\n", medium_activity));
    content.push_str(&format!("- Low Activity Relationships (≤10 interactions): {}\n\n", low_activity));
    
    // Interaction type diversity
    let diverse_relationships = profiles.iter().filter(|p| p.interaction_counts.len() > 2).count();
    let simple_relationships = profiles.iter().filter(|p| p.interaction_counts.len() <= 2).count();
    
    content.push_str("## INTERACTION DIVERSITY\n");
    content.push_str(&format!("- Diverse Communication (>2 interaction types): {}\n", diverse_relationships));
    content.push_str(&format!("- Simple Communication (≤2 interaction types): {}\n\n", simple_relationships));
    
    // Time span analysis
    let mut active_relationships = 0;
    let mut dormant_relationships = 0;
    
    for profile in profiles {
        if let (Some(first), Some(last)) = (profile.first_interaction, profile.last_interaction) {
            let duration = last.signed_duration_since(first);
            if duration.num_days() > 30 {
                active_relationships += 1;
            } else {
                dormant_relationships += 1;
            }
        }
    }
    
    content.push_str("## RELATIONSHIP LONGEVITY\n");
    content.push_str(&format!("- Long-term Relationships (>30 days): {}\n", active_relationships));
    content.push_str(&format!("- Short-term Relationships (≤30 days): {}\n\n", dormant_relationships));
    
    content.push_str("---\nGenerated by Tweet-Scrolls Relationship Intelligence System\n");
    content
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::relationship::anonymization::hash_user_id;
    use std::collections::HashMap;

    #[test]
    fn test_file_generator_creation() {
        let generator = LLMFileGenerator::new("/tmp/test", "testuser", 1234567890);
        assert_eq!(generator.screen_name, "testuser");
        assert_eq!(generator.timestamp, 1234567890);
        assert!(generator.output_dir.contains("testuser"));
    }
}