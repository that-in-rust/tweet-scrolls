//! Main function integration for relationship analysis

use anyhow::Result;
use crate::models::profile::UserProfile;
use crate::models::interaction::InteractionEvent;
use crate::relationship::file_generation::LLMFileGenerator;

/// Main orchestration function for relationship analysis
pub async fn main_analyze_relationships(
    output_path: &str,
    screen_name: &str,
    timestamp: u64,
    profiles: &[UserProfile],
    interactions: &[InteractionEvent]
) -> Result<()> {
    // Create the file generator
    let generator = LLMFileGenerator::new(
        output_path,
        screen_name,
        timestamp,
    );
    
    // Generate all the relationship analysis files
    generator.generate_all_files(profiles, interactions)?;
    
    Ok(())
}

/// Generate the user prompt for relationship analysis
pub fn generate_relationship_analysis_prompt() -> String {
    "🔍 Tweet-Scrolls can generate comprehensive relationship intelligence profiles\n\
        from your Twitter data, including:\n\
        \n\
        • Individual user interaction profiles\n\
        • Communication pattern analysis\n\
        • Timeline of all interactions\n\
        • Relationship network visualization\n\
        • LLM-ready analysis prompts\n\
        \n\
        This will create detailed files in a 'relationship_profiles' directory.\n\
        \n\
        Would you like to generate relationship intelligence profiles? (y/n): ".to_string()
}

/// Determine if user wants to run relationship analysis
pub fn should_run_relationship_analysis(input: &str) -> bool {
    let trimmed = input.trim().to_lowercase();
    matches!(trimmed.as_str(), "y" | "yes" | "1" | "true")
}