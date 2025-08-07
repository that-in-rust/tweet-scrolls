//! LLM analysis prompts generator
//! 
//! Creates suggested questions for LLM analysis of relationship data.

use std::collections::HashMap;
use crate::models::profile::UserProfile;

/// Generates LLM analysis prompts for relationship intelligence
pub fn generate_llm_analysis_prompts(profiles: &HashMap<String, UserProfile>) -> String {
    let mut output = String::new();
    
    add_prompts_header(&mut output);
    add_relationship_health_prompts(&mut output);
    add_communication_optimization_prompts(&mut output);
    add_network_insights_prompts(&mut output);
    add_behavioral_patterns_prompts(&mut output);
    add_analysis_context(&mut output, profiles);
    
    output
}

/// Adds prompts header
fn add_prompts_header(output: &mut String) {
    output.push_str("LLM ANALYSIS PROMPTS\n");
    output.push_str("===================\n");
    output.push_str("Suggested questions to ask your LLM when analyzing these relationship profiles:\n\n");
}

/// Adds relationship health analysis prompts
fn add_relationship_health_prompts(output: &mut String) {
    output.push_str("RELATIONSHIP HEALTH ANALYSIS\n");
    output.push_str("---------------------------\n");
    output.push_str("1. Which relationships need more attention?\n");
    output.push_str("2. Are there any relationships showing declining engagement over time?\n");
    output.push_str("3. Which relationships have the most balanced communication patterns?\n");
    output.push_str("4. What relationships show the strongest consistency in interaction?\n\n");
}

/// Adds communication optimization prompts
fn add_communication_optimization_prompts(output: &mut String) {
    output.push_str("COMMUNICATION OPTIMIZATION\n");
    output.push_str("-------------------------\n");
    output.push_str("5. What communication patterns make conversations most engaging?\n");
    output.push_str("6. When are the optimal times to reach out to different people?\n");
    output.push_str("7. Which relationships would benefit from more frequent check-ins?\n");
    output.push_str("8. Are there communication imbalances that should be addressed?\n\n");
}

/// Adds network insights prompts
fn add_network_insights_prompts(output: &mut String) {
    output.push_str("NETWORK INSIGHTS\n");
    output.push_str("---------------\n");
    output.push_str("9. Who are the most important people in this social network?\n");
    output.push_str("10. What does the overall relationship portfolio suggest about social preferences?\n");
    output.push_str("11. Are there clusters of similar relationship patterns?\n");
    output.push_str("12. Which relationships serve different social or professional functions?\n\n");
}

/// Adds behavioral patterns prompts
fn add_behavioral_patterns_prompts(output: &mut String) {
    output.push_str("BEHAVIORAL PATTERNS\n");
    output.push_str("------------------\n");
    output.push_str("13. What do the temporal patterns reveal about communication habits?\n");
    output.push_str("14. How has communication behavior evolved over time?\n");
    output.push_str("15. What relationship maintenance strategies are most effective?\n");
    output.push_str("16. Are there seasonal or cyclical patterns in relationship activity?\n\n");
}

/// Adds analysis context information
fn add_analysis_context(output: &mut String, profiles: &HashMap<String, UserProfile>) {
    output.push_str("ANALYSIS CONTEXT\n");
    output.push_str("---------------\n");
    output.push_str(&format!("Total relationships analyzed: {}\n", profiles.len()));
    
    let total_interactions: u32 = profiles.values().map(|p| p.total_interactions).sum();
    output.push_str(&format!("Total interactions across all relationships: {}\n", total_interactions));
    
    if let Some(most_active) = profiles.values().max_by_key(|p| p.total_interactions) {
        output.push_str(&format!("Most active relationship: {} interactions\n", most_active.total_interactions));
    }
    
    output.push_str("\nNOTE: All user IDs have been anonymized using Blake3 hashing for privacy.\n");
    output.push_str("Focus on patterns and insights rather than identifying specific individuals.\n");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::profile::UserProfile;

    #[test]
    fn test_generate_llm_analysis_prompts() {
        let mut profiles = HashMap::new();
        let mut profile = UserProfile::new("test_user");
        profile.total_interactions = 42;
        profiles.insert("user1".to_string(), profile);
        
        let prompts = generate_llm_analysis_prompts(&profiles);
        
        // Verify essential prompts are present
        assert!(prompts.contains("Which relationships need more attention?"));
        assert!(prompts.contains("What communication patterns make conversations most engaging?"));
        assert!(prompts.contains("Who are the most important people in this social network?"));
        assert!(prompts.contains("What do the temporal patterns reveal about communication habits?"));
        
        // Verify context information
        assert!(prompts.contains("Total relationships analyzed: 1"));
        assert!(prompts.contains("Blake3 hashing for privacy"));
    }

    #[test]
    fn test_analysis_context_with_multiple_profiles() {
        let mut profiles = HashMap::new();
        
        let mut profile1 = UserProfile::new("user1");
        profile1.total_interactions = 50;
        profiles.insert("user1".to_string(), profile1);
        
        let mut profile2 = UserProfile::new("user2");
        profile2.total_interactions = 100;
        profiles.insert("user2".to_string(), profile2);
        
        let prompts = generate_llm_analysis_prompts(&profiles);
        
        assert!(prompts.contains("Total relationships analyzed: 2"));
        assert!(prompts.contains("Most active relationship: 100 interactions"));
    }
}