use anyhow::Result;
use std::path::Path;
use tempfile::tempdir;
use tweet_scrolls::models::profile::UserProfile;
use tweet_scrolls::models::interaction::InteractionEvent;

/// Test the main relationship analysis integration function
#[tokio::test]
async fn test_main_relationship_analysis_integration() -> Result<()> {
    let temp_dir = tempdir()?;
    let output_path = temp_dir.path().to_str().unwrap();
    let screen_name = "testuser";
    let timestamp = 1234567890;
    
    // Create sample data
    let profiles = create_sample_profiles();
    let interactions = create_sample_interactions();
    
    // Test the main integration function
    let result = main_analyze_relationships(
        output_path,
        screen_name,
        timestamp,
        &profiles,
        &interactions
    ).await;
    
    assert!(result.is_ok());
    
    // Verify output directory structure
    let relationship_dir = Path::new(output_path).join(format!("relationship_profiles_{}_{}", screen_name, timestamp));
    assert!(relationship_dir.exists());
    
    // Verify key files were created
    assert!(relationship_dir.join("interaction_timeline.txt").exists());
    assert!(relationship_dir.join("communication_patterns.txt").exists());
    assert!(relationship_dir.join("relationship_network.txt").exists());
    assert!(relationship_dir.join("llm_analysis_prompts.txt").exists());
    
    Ok(())
}

/// Test relationship analysis with user input simulation
#[test]
fn test_relationship_analysis_user_prompt() {
    let prompt_text = generate_relationship_analysis_prompt();
    
    assert!(prompt_text.contains("relationship intelligence profiles"));
    assert!(prompt_text.contains("(y/n)"));
    assert!(prompt_text.len() > 50);
}

/// Test the orchestration function handles errors gracefully
#[tokio::test]
async fn test_main_relationship_analysis_error_handling() -> Result<()> {
    let invalid_path = "/invalid/nonexistent/path";
    let screen_name = "testuser";
    let timestamp = 1234567890;
    
    let profiles = create_sample_profiles();
    let interactions = create_sample_interactions();
    
    let result = main_analyze_relationships(
        invalid_path,
        screen_name,
        timestamp,
        &profiles,
        &interactions
    ).await;
    
    // Should handle error gracefully
    assert!(result.is_err());
    
    Ok(())
}

/// Test integration with existing main function flow
#[test]
fn test_main_function_integration_flow() {
    // Test that we can determine if user wants relationship analysis
    let user_input_yes = "y\n";
    let user_input_no = "n\n";
    
    assert!(should_run_relationship_analysis(user_input_yes));
    assert!(!should_run_relationship_analysis(user_input_no));
    assert!(!should_run_relationship_analysis("invalid\n"));
}

// Helper functions for testing

fn create_sample_profiles() -> Vec<UserProfile> {
    vec![
        UserProfile::new("abcd1234efgh5678"),
        UserProfile::new("wxyz9876abcd5432"),
    ]
}

fn create_sample_interactions() -> Vec<InteractionEvent> {
    use chrono::Utc;
    use tweet_scrolls::models::interaction::{InteractionEvent, InteractionType};
    
    vec![
        InteractionEvent {
            id: "msg1".to_string(),
            user_hash: "abcd1234efgh5678".to_string(),
            interaction_type: InteractionType::DmSent,
            timestamp: Utc::now(),
            content: "Test message".to_string(),
            metadata: std::collections::HashMap::new(),
        },
        InteractionEvent {
            id: "msg2".to_string(),
            user_hash: "wxyz9876abcd5432".to_string(),
            interaction_type: InteractionType::DmReceived,
            timestamp: Utc::now(),
            content: "Test reply".to_string(),
            metadata: std::collections::HashMap::new(),
        },
    ]
}

// Import functions from main codebase
use tweet_scrolls::main_integration::{
    main_analyze_relationships,
    generate_relationship_analysis_prompt,
    should_run_relationship_analysis,
};