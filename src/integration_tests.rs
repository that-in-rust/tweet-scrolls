//! Integration tests for tweet-scrolls
//! 
//! This module contains integration tests that were moved from main.rs
//! to keep the main module clean and focused.

use super::*;
use anyhow::{Context, Result};
use std::fs;
use tempfile::tempdir;
use std::time::Duration;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_dm_processing_integration() -> Result<()> {
        let temp_dir = tempdir().context("Failed to create temp directory")?;
        let output_dir = temp_dir.path();
        
        // Create test DM content
        let test_dm_content = r#"window.YTD.direct_messages.part0 = [
  {
    "dmConversation": {
      "conversationId": "123-456",
      "messages": [
        {
          "messageCreate": {
            "id": "msg1",
            "text": "Hello there!",
            "createdAt": "2023-01-01T10:00:00.000Z"
          }
        },
        {
          "messageCreate": {
            "id": "msg2",
            "text": "How are you?",
            "createdAt": "2023-01-01T10:05:00.000Z"
          }
        }
      ]
    }
  },
  {
    "dmConversation": {
      "conversationId": "789-012",
      "messages": [
        {
          "messageCreate": {
            "id": "msg3",
            "text": "Test message",
            "createdAt": "2023-01-02T15:30:00.000Z"
          }
        }
      ]
    }
  }
]"#;
        
        let dm_file_path = output_dir.join("test_dm.js");
        fs::write(&dm_file_path, test_dm_content)
            .context("Failed to write test DM file")?;
        
        // Process the DM file
        process_dm_file(
            dm_file_path.to_str().context("Invalid file path")?,
            "testuser",
            output_dir,
            1234567890
        ).await
        .context("DM processing should succeed")?;
        
        // Verify output files were created
        let csv_file = output_dir.join("dm_conversations_testuser_1234567890.csv");
        let txt_file = output_dir.join("dm_conversations_testuser_1234567890.txt");
        let summary_file = output_dir.join("dm_results_testuser_1234567890.txt");
        
        assert!(csv_file.exists(), "CSV file should be created");
        assert!(txt_file.exists(), "TXT file should be created");
        assert!(summary_file.exists(), "Summary file should be created");
        
        // Verify CSV content
        let csv_content = fs::read_to_string(&csv_file)
            .context("Failed to read CSV file")?;
        assert!(csv_content.contains("Conversation ID"), "CSV should have headers");
        assert!(csv_content.contains("123-456"), "CSV should contain conversation 1");
        assert!(csv_content.contains("789-012"), "CSV should contain conversation 2");
        
        // Verify TXT content
        let txt_content = fs::read_to_string(&txt_file)
            .context("Failed to read TXT file")?;
        assert!(txt_content.contains("--- Conversation 1 ---"), "TXT should have conversation headers");
        assert!(txt_content.contains("Messages: 2"), "TXT should show message count");
        
        // Verify summary content
        let summary_content = fs::read_to_string(&summary_file)
            .context("Failed to read summary file")?;
        assert!(summary_content.contains("Total Conversations: 2"), "Summary should show conversation count");
        assert!(summary_content.contains("Total Messages: 3"), "Summary should show total message count");
        
        Ok(())
    }

    #[tokio::test]
    async fn test_dm_javascript_prefix_removal() -> Result<()> {
        let temp_dir = tempdir().context("Failed to create temp directory")?;
        let output_dir = temp_dir.path();
        
        // Test with different JavaScript prefix variations
        let test_dm_content = r#"window.YTD.direct_messages.part0 = [
  {
    "dmConversation": {
      "conversationId": "prefix-test",
      "messages": [
        {
          "messageCreate": {
            "id": "msg1",
            "text": "Testing prefix removal",
            "createdAt": "2023-01-01T10:00:00.000Z"
          }
        }
      ]
    }
  }
]"#;
        
        let dm_file_path = output_dir.join("prefix_test.js");
        fs::write(&dm_file_path, test_dm_content)
            .context("Failed to write test DM file")?;
        
        process_dm_file(
            dm_file_path.to_str().context("Invalid file path")?,
            "testuser",
            output_dir,
            1234567890
        ).await
        .context("Should handle JavaScript prefix correctly")?;
        
        let csv_file = output_dir.join("dm_conversations_testuser_1234567890.csv");
        let csv_content = fs::read_to_string(&csv_file)
            .context("Failed to read CSV file")?;
        assert!(csv_content.contains("prefix-test"), "Should parse conversation after prefix removal");
        
        Ok(())
    }

    #[tokio::test]
    async fn test_dm_headers_prefix_removal() -> Result<()> {
        let temp_dir = tempdir().context("Failed to create temp directory")?;
        let output_dir = temp_dir.path();
        
        // Test with headers JavaScript prefix format
        let test_dm_content = r#"window.YTD.direct_message_headers.part0 = [
  {
    "dmConversation": {
      "conversationId": "headers-test",
      "messages": [
        {
          "messageCreate": {
            "id": "msg1",
            "text": "Testing headers prefix removal",
            "createdAt": "2023-01-01T10:00:00.000Z"
          }
        }
      ]
    }
  }
]"#;
        
        let dm_file_path = output_dir.join("headers_test.js");
        fs::write(&dm_file_path, test_dm_content)
            .context("Failed to write test DM file")?;
        
        process_dm_file(
            dm_file_path.to_str().context("Invalid file path")?,
            "testuser",
            output_dir,
            1234567890
        ).await
        .context("Should handle headers JavaScript prefix correctly")?;
        
        let csv_file = output_dir.join("dm_conversations_testuser_1234567890.csv");
        let csv_content = fs::read_to_string(&csv_file)
            .context("Failed to read CSV file")?;
        assert!(csv_content.contains("headers-test"), "Should parse conversation after headers prefix removal");
        
        Ok(())
    }

    #[test]
    fn test_user_id_anonymization() -> Result<()> {
        let user_id = "1132151165410455552";
        let hash1 = hash_user_id(user_id);
        let hash2 = hash_user_id(user_id);
        
        assert_eq!(hash1, hash2, "Hashing should be consistent");
        assert_ne!(hash1, user_id, "Hash should be different from original");
        assert_eq!(hash1.len(), 64, "Blake3 hash should be 64 characters");
        
        Ok(())
    }

    #[test]
    fn test_user_id_anonymization_different_inputs() -> Result<()> {
        let user_id1 = "1132151165410455552";
        let user_id2 = "9876543210123456789";
        
        let hash1 = hash_user_id(user_id1);
        let hash2 = hash_user_id(user_id2);
        
        // Different inputs should produce different hashes
        assert_ne!(hash1, hash2, "Different inputs should produce different hashes");
        
        // Both should be properly formatted hashes
        assert_eq!(hash1.len(), 64, "First hash should be 64 characters");
        assert_eq!(hash2.len(), 64, "Second hash should be 64 characters");
        
        // Both should be hex strings (only contain 0-9, a-f)
        assert!(hash1.chars().all(|c| c.is_ascii_hexdigit()), "First hash should be valid hex");
        assert!(hash2.chars().all(|c| c.is_ascii_hexdigit()), "Second hash should be valid hex");
        
        Ok(())
    }

    #[test]
    fn test_user_id_anonymization_edge_cases() -> Result<()> {
        // Test empty string
        let empty_hash = hash_user_id("");
        assert_eq!(empty_hash.len(), 64, "Empty string hash should be 64 characters");
        
        // Test very long string
        let long_id = "a".repeat(1000);
        let long_hash = hash_user_id(&long_id);
        assert_eq!(long_hash.len(), 64, "Long string hash should be 64 characters");
        
        // Test special characters
        let special_id = "user@123!#$%";
        let special_hash = hash_user_id(special_id);
        assert_eq!(special_hash.len(), 64, "Special characters hash should be 64 characters");
        assert_ne!(special_hash, special_id, "Hash should be different from original");
        
        Ok(())
    }

    // Helper function to create sample DM data for testing
    fn create_sample_dm_data() -> Vec<DmWrapper> {
        vec![
            DmWrapper {
                dm_conversation: DmConversation {
                    conversation_id: "3382-1132151165410455552".to_string(),
                    messages: vec![
                        DmMessage {
                            message_create: Some(DmMessageCreate {
                                id: Some("msg1".to_string()),
                                text: Some("Hello there!".to_string()),
                                created_at: Some("2023-01-01T10:00:00.000Z".to_string()),
                            }),
                        },
                    ],
                },
            },
            DmWrapper {
                dm_conversation: DmConversation {
                    conversation_id: "1132151165410455552-9876543210".to_string(),
                    messages: vec![
                        DmMessage {
                            message_create: Some(DmMessageCreate {
                                id: Some("msg2".to_string()),
                                text: Some("How are you?".to_string()),
                                created_at: Some("2023-01-01T10:05:00.000Z".to_string()),
                            }),
                        },
                    ],
                },
            },
        ]
    }

    // Helper function to create sample tweet data for testing
    fn create_sample_tweet_data() -> Vec<Tweet> {
        vec![
            Tweet {
                id_str: "tweet1".to_string(),
                favorite_count: "5".to_string(),
                full_text: "Hello world!".to_string(),
                in_reply_to_status_id: None,
                retweeted: false,
                in_reply_to_screen_name: Some("alice".to_string()),
                retweet_count: "2".to_string(),
                created_at: "Mon Jan 01 10:00:00 +0000 2023".to_string(),
            },
            Tweet {
                id_str: "tweet2".to_string(),
                favorite_count: "3".to_string(),
                full_text: "Another tweet".to_string(),
                in_reply_to_status_id: None,
                retweeted: false,
                in_reply_to_screen_name: Some("bob".to_string()),
                retweet_count: "1".to_string(),
                created_at: "Mon Jan 01 11:00:00 +0000 2023".to_string(),
            },
        ]
    }

    #[test]
    fn test_extract_unique_users_from_dms() -> Result<()> {
        let sample_dm_data = create_sample_dm_data();
        let analyzer = RelationshipAnalyzer::new();
        
        let users = analyzer.extract_users_from_dms(&sample_dm_data);
        
        // Should extract 3 unique users: "3382", "1132151165410455552", "9876543210"
        assert_eq!(users.len(), 3, "Should extract 3 unique users");
        assert!(users.contains(&hash_user_id("3382")), "Should contain first user");
        assert!(users.contains(&hash_user_id("1132151165410455552")), "Should contain second user");
        assert!(users.contains(&hash_user_id("9876543210")), "Should contain third user");
        
        Ok(())
    }

    #[test]
    fn test_extract_users_from_tweets() -> Result<()> {
        let sample_tweet_data = create_sample_tweet_data();
        let analyzer = RelationshipAnalyzer::new();
        
        let users = analyzer.extract_users_from_tweets(&sample_tweet_data);
        
        // Should extract 2 unique users from in_reply_to_screen_name: "alice", "bob"
        assert_eq!(users.len(), 2, "Should extract 2 unique users");
        assert!(users.contains(&hash_user_id("alice")), "Should contain alice");
        assert!(users.contains(&hash_user_id("bob")), "Should contain bob");
        
        Ok(())
    }

    #[test]
    fn test_handle_empty_data_gracefully() -> Result<()> {
        let analyzer = RelationshipAnalyzer::new();
        
        // Test with empty DM data
        let empty_dm_data: Vec<DmWrapper> = vec![];
        let dm_users = analyzer.extract_users_from_dms(&empty_dm_data);
        assert_eq!(dm_users.len(), 0, "Empty DM data should return no users");
        
        // Test with empty tweet data
        let empty_tweet_data: Vec<Tweet> = vec![];
        let tweet_users = analyzer.extract_users_from_tweets(&empty_tweet_data);
        assert_eq!(tweet_users.len(), 0, "Empty tweet data should return no users");
        
        Ok(())
    }

    #[test]
    fn test_extract_users_from_malformed_conversation_ids() -> Result<()> {
        let malformed_dm_data = vec![
            DmWrapper {
                dm_conversation: DmConversation {
                    conversation_id: "no_dash_here".to_string(), // No dash separator
                    messages: vec![],
                },
            },
            DmWrapper {
                dm_conversation: DmConversation {
                    conversation_id: "user1-user2".to_string(), // Valid format
                    messages: vec![],
                },
            },
        ];
        
        let analyzer = RelationshipAnalyzer::new();
        let users = analyzer.extract_users_from_dms(&malformed_dm_data);
        
        // Should only extract from the valid conversation ID
        assert_eq!(users.len(), 2, "Should extract 2 users from valid conversation ID");
        assert!(users.contains(&hash_user_id("user1")), "Should contain user1");
        assert!(users.contains(&hash_user_id("user2")), "Should contain user2");
        
        Ok(())
    }
}

mod integration_tests;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_dm_processing_with_empty_messages() {
        let temp_dir = tempdir().unwrap();
        let output_dir = temp_dir.path();
        
        // Test with conversation that has empty messages
        let test_dm_content = r#"window.YTD.direct_messages.part0 = [
  {
    "dmConversation": {
      "conversationId": "empty-123",
      "messages": []
    }
  }
]"#;
        
        let dm_file_path = output_dir.join("empty_dm.js");
        fs::write(&dm_file_path, test_dm_content).unwrap();
        
        let result = process_dm_file(
            dm_file_path.to_str().unwrap(),
            "testuser",
            output_dir,
            1234567890
        ).await;
        
        assert!(result.is_ok(), "Should handle empty conversations gracefully");
        
        // Should create files but with no conversations
        let summary_file = output_dir.join("dm_results_testuser_1234567890.txt");
        let summary_content = fs::read_to_string(&summary_file).unwrap();
        assert!(summary_content.contains("Total Conversations: 0"));
    }

    #[tokio::test]
    async fn test_dm_processing() {
        // Create a temporary directory for output
        let temp_dir = tempdir().unwrap();
        let output_dir = temp_dir.path();
        
        // Create test DM file
        let test_dm_content = r#"window.YTD.direct_messages.part0 = [
  {
    "dmConversation": {
      "conversationId": "123-456",
      "messages": [
        {
          "messageCreate": {
            "id": "msg1",
            "text": "Hello there!",
            "createdAt": "2023-01-01T10:00:00.000Z"
          }
        },
        {
          "messageCreate": {
            "id": "msg2",
            "text": "How are you?",
            "createdAt": "2023-01-01T10:05:00.000Z"
          }
        }
      ]
    }
  },
  {
    "dmConversation": {
      "conversationId": "789-012",
      "messages": [
        {
          "messageCreate": {
            "id": "msg3",
            "text": "Test message",
            "createdAt": "2023-01-02T15:30:00.000Z"
          }
        }
      ]
    }
  }
]"#;
        
        let dm_file_path = output_dir.join("test_dm.js");
        fs::write(&dm_file_path, test_dm_content).unwrap();
        
        // Process the DM file
        let result = process_dm_file(
            dm_file_path.to_str().unwrap(),
            "testuser",
            output_dir,
            1234567890
        ).await;
        
        // Verify processing succeeded
        assert!(result.is_ok(), "DM processing should succeed: {:?}", result.err());
        
        // Verify output files were created
        let csv_file = output_dir.join("dm_conversations_testuser_1234567890.csv");
        let txt_file = output_dir.join("dm_conversations_testuser_1234567890.txt");
        let summary_file = output_dir.join("dm_results_testuser_1234567890.txt");
        
        assert!(csv_file.exists(), "CSV file should be created");
        assert!(txt_file.exists(), "TXT file should be created");
        assert!(summary_file.exists(), "Summary file should be created");
        
        // Verify CSV content
        let csv_content = fs::read_to_string(&csv_file).unwrap();
        assert!(csv_content.contains("Conversation ID"), "CSV should have headers");
        assert!(csv_content.contains("123-456"), "CSV should contain conversation 1");
        assert!(csv_content.contains("789-012"), "CSV should contain conversation 2");
        
        // Verify TXT content
        let txt_content = fs::read_to_string(&txt_file).unwrap();
        assert!(txt_content.contains("--- Conversation 1 ---"), "TXT should have conversation headers");
        assert!(txt_content.contains("Messages: 2"), "TXT should show message count");
        
        // Verify summary content
        let summary_content = fs::read_to_string(&summary_file).unwrap();
        assert!(summary_content.contains("Total Conversations: 2"), "Summary should show conversation count");
        assert!(summary_content.contains("Total Messages: 3"), "Summary should show total message count");
    }

    #[tokio::test]
    async fn test_dm_javascript_prefix_removal() {
        let temp_dir = tempdir().unwrap();
        let output_dir = temp_dir.path();
        
        // Test with different JavaScript prefix variations
        let test_dm_content = r#"window.YTD.direct_messages.part0 = [
  {
    "dmConversation": {
      "conversationId": "prefix-test",
      "messages": [
        {
          "messageCreate": {
            "id": "msg1",
            "text": "Testing prefix removal",
            "createdAt": "2023-01-01T10:00:00.000Z"
          }
        }
      ]
    }
  }
]"#;
        
        let dm_file_path = output_dir.join("prefix_test.js");
        fs::write(&dm_file_path, test_dm_content).unwrap();
        
        let result = process_dm_file(
            dm_file_path.to_str().unwrap(),
            "testuser",
            output_dir,
            1234567890
        ).await;
        
        assert!(result.is_ok(), "Should handle JavaScript prefix correctly");
        
        let csv_file = output_dir.join("dm_conversations_testuser_1234567890.csv");
        let csv_content = fs::read_to_string(&csv_file).unwrap();
        assert!(csv_content.contains("prefix-test"), "Should parse conversation after prefix removal");
    }

    #[tokio::test]
    async fn test_dm_headers_prefix_removal() {
        let temp_dir = tempdir().unwrap();
        let output_dir = temp_dir.path();
        
        // Test with headers JavaScript prefix format
        let test_dm_content = r#"window.YTD.direct_message_headers.part0 = [
  {
    "dmConversation": {
      "conversationId": "headers-test",
      "messages": [
        {
          "messageCreate": {
            "id": "msg1",
            "text": "Testing headers prefix removal",
            "createdAt": "2023-01-01T10:00:00.000Z"
          }
        }
      ]
    }
  }
]"#;
        
        let dm_file_path = output_dir.join("headers_test.js");
        fs::write(&dm_file_path, test_dm_content).unwrap();
        
        let result = process_dm_file(
            dm_file_path.to_str().unwrap(),
            "testuser",
            output_dir,
            1234567890
        ).await;
        
        assert!(result.is_ok(), "Should handle headers JavaScript prefix correctly");
        
        let csv_file = output_dir.join("dm_conversations_testuser_1234567890.csv");
        let csv_content = fs::read_to_string(&csv_file).unwrap();
        assert!(csv_content.contains("headers-test"), "Should parse conversation after headers prefix removal");
    }

    #[test]
    fn test_user_id_anonymization() {
        let user_id = "1132151165410455552";
        let hash1 = hash_user_id(user_id);
        let hash2 = hash_user_id(user_id);
        
        assert_eq!(hash1, hash2); // Consistent hashing
        assert_ne!(hash1, user_id); // Actually anonymized
        assert_eq!(hash1.len(), 64); // Blake3 hash length
    }

    #[test]
    fn test_user_id_anonymization_different_inputs() {
        let user_id1 = "1132151165410455552";
        let user_id2 = "9876543210123456789";
        
        let hash1 = hash_user_id(user_id1);
        let hash2 = hash_user_id(user_id2);
        
        // Different inputs should produce different hashes
        assert_ne!(hash1, hash2);
        
        // Both should be properly formatted hashes
        assert_eq!(hash1.len(), 64);
        assert_eq!(hash2.len(), 64);
        
        // Both should be hex strings (only contain 0-9, a-f)
        assert!(hash1.chars().all(|c| c.is_ascii_hexdigit()));
        assert!(hash2.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_user_id_anonymization_edge_cases() {
        // Test empty string
        let empty_hash = hash_user_id("");
        assert_eq!(empty_hash.len(), 64);
        
        // Test very long string
        let long_id = "a".repeat(1000);
        let long_hash = hash_user_id(&long_id);
        assert_eq!(long_hash.len(), 64);
        
        // Test special characters
        let special_id = "user@123!#$%";
        let special_hash = hash_user_id(special_id);
        assert_eq!(special_hash.len(), 64);
        assert_ne!(special_hash, special_id);
    }

    // Helper function to create sample DM data for testing
    fn create_sample_dm_data() -> Vec<DmWrapper> {
        vec![
            DmWrapper {
                dm_conversation: DmConversation {
                    conversation_id: "3382-1132151165410455552".to_string(),
                    messages: vec![
                        DmMessage {
                            message_create: Some(DmMessageCreate {
                                id: Some("msg1".to_string()),
                                text: Some("Hello there!".to_string()),
                                created_at: Some("2023-01-01T10:00:00.000Z".to_string()),
                            }),
                        },
                    ],
                },
            },
            DmWrapper {
                dm_conversation: DmConversation {
                    conversation_id: "1132151165410455552-9876543210".to_string(),
                    messages: vec![
                        DmMessage {
                            message_create: Some(DmMessageCreate {
                                id: Some("msg2".to_string()),
                                text: Some("How are you?".to_string()),
                                created_at: Some("2023-01-01T10:05:00.000Z".to_string()),
                            }),
                        },
                    ],
                },
            },
        ]
    }

    // Helper function to create sample tweet data for testing
    fn create_sample_tweet_data() -> Vec<Tweet> {
        vec![
            Tweet {
                id_str: "tweet1".to_string(),
                favorite_count: "5".to_string(),
                full_text: "Hello world!".to_string(),
                in_reply_to_status_id: None,
                retweeted: false,
                in_reply_to_screen_name: Some("alice".to_string()),
                retweet_count: "2".to_string(),
                created_at: "Mon Jan 01 10:00:00 +0000 2023".to_string(),
            },
            Tweet {
                id_str: "tweet2".to_string(),
                favorite_count: "3".to_string(),
                full_text: "Another tweet".to_string(),
                in_reply_to_status_id: None,
                retweeted: false,
                in_reply_to_screen_name: Some("bob".to_string()),
                retweet_count: "1".to_string(),
                created_at: "Mon Jan 01 11:00:00 +0000 2023".to_string(),
            },
        ]
    }

    #[test]
    fn test_extract_unique_users_from_dms() {
        let sample_dm_data = create_sample_dm_data();
        let analyzer = RelationshipAnalyzer::new();
        
        let users = analyzer.extract_users_from_dms(&sample_dm_data);
        
        // Should extract 3 unique users: "3382", "1132151165410455552", "9876543210"
        assert_eq!(users.len(), 3);
        assert!(users.contains(&hash_user_id("3382")));
        assert!(users.contains(&hash_user_id("1132151165410455552")));
        assert!(users.contains(&hash_user_id("9876543210")));
    }

    #[test]
    fn test_extract_users_from_tweets() {
        let sample_tweet_data = create_sample_tweet_data();
        let analyzer = RelationshipAnalyzer::new();
        
        let users = analyzer.extract_users_from_tweets(&sample_tweet_data);
        
        // Should extract 2 unique users from in_reply_to_screen_name: "alice", "bob"
        assert_eq!(users.len(), 2);
        assert!(users.contains(&hash_user_id("alice")));
        assert!(users.contains(&hash_user_id("bob")));
    }

    #[test]
    fn test_handle_empty_data_gracefully() {
        let analyzer = RelationshipAnalyzer::new();
        
        // Test with empty DM data
        let empty_dm_data: Vec<DmWrapper> = vec![];
        let dm_users = analyzer.extract_users_from_dms(&empty_dm_data);
        assert_eq!(dm_users.len(), 0);
        
        // Test with empty tweet data
        let empty_tweet_data: Vec<Tweet> = vec![];
        let tweet_users = analyzer.extract_users_from_tweets(&empty_tweet_data);
        assert_eq!(tweet_users.len(), 0);
    }

    #[test]
    fn test_extract_users_from_malformed_conversation_ids() {
        let malformed_dm_data = vec![
            DmWrapper {
                dm_conversation: DmConversation {
                    conversation_id: "no_dash_here".to_string(), // No dash separator
                    messages: vec![],
                },
            },
            DmWrapper {
                dm_conversation: DmConversation {
                    conversation_id: "user1-user2".to_string(), // Valid format
                    messages: vec![],
                },
            },
        ];
        
        let analyzer = RelationshipAnalyzer::new();
        let users = analyzer.extract_users_from_dms(&malformed_dm_data);
        
        // Should only extract from the valid conversation ID
        assert_eq!(users.len(), 2);
        assert!(users.contains(&hash_user_id("user1")));
        assert!(users.contains(&hash_user_id("user2")));
    }

    // Helper function to create sample conversation data for testing
    fn create_sample_conversation_data() -> Vec<DmWrapper> {
        vec![
            DmWrapper {
                dm_conversation: DmConversation {
                    conversation_id: "user123-target_user".to_string(),
                    messages: vec![
                        DmMessage {
                            message_create: Some(DmMessageCreate {
                                id: Some("msg1".to_string()),
                                text: Some("First message".to_string()),
                                created_at: Some("2023-01-01T10:00:00.000Z".to_string()),
                            }),
                        },
                        DmMessage {
                            message_create: Some(DmMessageCreate {
                                id: Some("msg2".to_string()),
                                text: Some("Second message".to_string()),
                                created_at: Some("2023-01-01T11:00:00.000Z".to_string()),
                            }),
                        },
                        DmMessage {
                            message_create: Some(DmMessageCreate {
                                id: Some("msg3".to_string()),
                                text: Some("Third message".to_string()),
                                created_at: Some("2023-01-01T12:00:00.000Z".to_string()),
                            }),
                        },
                    ],
                },
            },
        ]
    }

    #[test]
    fn test_create_basic_user_profile() {
        let sample_data = create_sample_conversation_data();
        let analyzer = RelationshipAnalyzer::new();
        let user_hash = hash_user_id("target_user");
        
        let profile = analyzer.create_user_profile(&user_hash, &sample_data);
        
        assert_eq!(profile.user_hash, user_hash);
        assert!(profile.dm_stats.total_messages > 0);
        assert!(profile.first_interaction.is_some());
        assert!(profile.last_interaction.is_some());
        assert!(profile.first_interaction <= profile.last_interaction);
    }

    #[test]
    fn test_dm_statistics_calculation() {
        let sample_data = create_sample_conversation_data();
        let analyzer = RelationshipAnalyzer::new();
        let user_hash = hash_user_id("target_user");
        
        let stats = analyzer.calculate_dm_statistics(&user_hash, &sample_data);
        
        assert_eq!(stats.total_messages, 3);
        assert_eq!(stats.messages_sent, 3); // All counted as sent for now
        assert_eq!(stats.messages_received, 0); // Not implemented yet
    }

    #[test]
    fn test_profile_with_no_interactions() {
        let empty_data: Vec<DmWrapper> = vec![];
        let analyzer = RelationshipAnalyzer::new();
        let user_hash = hash_user_id("nonexistent_user");
        
        let profile = analyzer.create_user_profile(&user_hash, &empty_data);
        
        assert_eq!(profile.user_hash, user_hash);
        assert_eq!(profile.dm_stats.total_messages, 0);
        assert!(profile.first_interaction.is_none());
        assert!(profile.last_interaction.is_none());
    }

    #[test]
    fn test_interaction_timespan_calculation() {
        let sample_data = create_sample_conversation_data();
        let analyzer = RelationshipAnalyzer::new();
        let user_hash = hash_user_id("target_user");
        
        let (first, last) = analyzer.find_interaction_timespan(&user_hash, &sample_data);
        
        assert!(first.is_some());
        assert!(last.is_some());
        
        let first_time = first.unwrap();
        let last_time = last.unwrap();
        
        // Should be chronologically ordered
        assert!(first_time <= last_time);
        
        // Should match our test data timestamps
        assert_eq!(first_time.format("%Y-%m-%d").to_string(), "2023-01-01");
        assert_eq!(last_time.format("%Y-%m-%d").to_string(), "2023-01-01");
    }

    // Phase 2, TDD Cycle 5: Response Time Analysis
    #[test]
    fn test_response_time_calculation() -> Result<()> {
        let conversation = create_sample_conversation_with_timestamps();
        let response_times = calculate_response_times(&conversation);
        
        assert_eq!(response_times.len(), 2, "Expected 2 response times for 3 messages");
        assert!(
            response_times[0] > Duration::from_secs(0),
            "Response time should be positive"
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_dm_processing_integration() -> Result<()> {
        let temp_dir = tempdir().context("Failed to create temp directory")?;
        let output_dir = temp_dir.path();
        
        // Create test DM content
        let test_dm_content = r#"window.YTD.direct_messages.part0 = [
  {
    "dmConversation": {
      "conversationId": "123-456",
      "messages": [
        {
          "messageCreate": {
            "id": "msg1",
            "text": "Hello there!",
            "createdAt": "2023-01-01T10:00:00.000Z"
          }
        },
        {
          "messageCreate": {
            "id": "msg2",
            "text": "How are you?",
            "createdAt": "2023-01-01T10:05:00.000Z"
          }
        }
      ]
    }
  },
  {
    "dmConversation": {
      "conversationId": "789-012",
      "messages": [
        {
          "messageCreate": {
            "id": "msg3",
            "text": "Test message",
            "createdAt": "2023-01-02T15:30:00.000Z"
          }
        }
      ]
    }
  }
]"#;
        
        let dm_file_path = output_dir.join("test_dm.js");
        fs::write(&dm_file_path, test_dm_content)
            .context("Failed to write test DM file")?;
        
        // Process the DM file
        process_dm_file(
            dm_file_path.to_str().context("Invalid file path")?,
            "testuser",
            output_dir,
            1234567890
        ).await
        .context("DM processing should succeed")?;
        
        // Verify output files were created
        let csv_file = output_dir.join("dm_conversations_testuser_1234567890.csv");
        let txt_file = output_dir.join("dm_conversations_testuser_1234567890.txt");
        let summary_file = output_dir.join("dm_results_testuser_1234567890.txt");
        
        assert!(csv_file.exists(), "CSV file should be created");
        assert!(txt_file.exists(), "TXT file should be created");
        assert!(summary_file.exists(), "Summary file should be created");
        
        // Verify CSV content
        let csv_content = fs::read_to_string(&csv_file)
            .context("Failed to read CSV file")?;
        assert!(csv_content.contains("Conversation ID"), "CSV should have headers");
        assert!(csv_content.contains("123-456"), "CSV should contain conversation 1");
        assert!(csv_content.contains("789-012"), "CSV should contain conversation 2");
        
        // Verify TXT content
        let txt_content = fs::read_to_string(&txt_file)
            .context("Failed to read TXT file")?;
        assert!(txt_content.contains("--- Conversation 1 ---"), "TXT should have conversation headers");
        assert!(txt_content.contains("Messages: 2"), "TXT should show message count");
        
        // Verify summary content
        let summary_content = fs::read_to_string(&summary_file)
            .context("Failed to read summary file")?;
        assert!(summary_content.contains("Total Conversations: 2"), "Summary should show conversation count");
        assert!(summary_content.contains("Total Messages: 3"), "Summary should show total message count");
        
        Ok(())
    }

    #[tokio::test]
    async fn test_dm_javascript_prefix_removal() {
        let temp_dir = tempdir().unwrap();
        let output_dir = temp_dir.path();
        
        // Test with different JavaScript prefix variations
        let test_dm_content = r#"window.YTD.direct_messages.part0 = [
  {
    "dmConversation": {
      "conversationId": "prefix-test",
      "messages": [
        {
          "messageCreate": {
            "id": "msg1",
            "text": "Testing prefix removal",
            "createdAt": "2023-01-01T10:00:00.000Z"
          }
        }
      ]
    }
  }
]"#;
        
        let dm_file_path = output_dir.join("prefix_test.js");
        fs::write(&dm_file_path, test_dm_content).unwrap();
        
        let result = process_dm_file(
            dm_file_path.to_str().unwrap(),
            "testuser",
            output_dir,
            1234567890
        ).await;
        
        assert!(result.is_ok(), "Should handle JavaScript prefix correctly");
        
        let csv_file = output_dir.join("dm_conversations_testuser_1234567890.csv");
        let csv_content = fs::read_to_string(&csv_file).unwrap();
        assert!(csv_content.contains("prefix-test"), "Should parse conversation after prefix removal");
    }

    #[tokio::test]
    async fn test_dm_headers_prefix_removal() {
        let temp_dir = tempdir().unwrap();
        let output_dir = temp_dir.path();
        
        // Test with headers JavaScript prefix format
        let test_dm_content = r#"window.YTD.direct_message_headers.part0 = [
  {
    "dmConversation": {
      "conversationId": "headers-test",
      "messages": [
        {
          "messageCreate": {
            "id": "msg1",
            "text": "Testing headers prefix removal",
            "createdAt": "2023-01-01T10:00:00.000Z"
          }
        }
      ]
    }
  }
]"#;
        
        let dm_file_path = output_dir.join("headers_test.js");
        fs::write(&dm_file_path, test_dm_content).unwrap();
        
        let result = process_dm_file(
            dm_file_path.to_str().unwrap(),
            "testuser",
            output_dir,
            1234567890
        ).await;
        
        assert!(result.is_ok(), "Should handle headers JavaScript prefix correctly");
        
        let csv_file = output_dir.join("dm_conversations_testuser_1234567890.csv");
        let csv_content = fs::read_to_string(&csv_file).unwrap();
        assert!(csv_content.contains("headers-test"), "Should parse conversation after headers prefix removal");
    }

    #[test]
    fn test_user_id_anonymization() {
        let user_id = "1132151165410455552";
        let hash1 = hash_user_id(user_id);
        let hash2 = hash_user_id(user_id);
        
        assert_eq!(hash1, hash2); // Consistent hashing
        assert_ne!(hash1, user_id); // Actually anonymized
        assert_eq!(hash1.len(), 64); // Blake3 hash length
    }

    #[test]
    fn test_user_id_anonymization_different_inputs() {
        let user_id1 = "1132151165410455552";
        let user_id2 = "9876543210123456789";
        
        let hash1 = hash_user_id(user_id1);
        let hash2 = hash_user_id(user_id2);
        
        // Different inputs should produce different hashes
        assert_ne!(hash1, hash2);
        
        // Both should be properly formatted hashes
        assert_eq!(hash1.len(), 64);
        assert_eq!(hash2.len(), 64);
        
        // Both should be hex strings (only contain 0-9, a-f)
        assert!(hash1.chars().all(|c| c.is_ascii_hexdigit()));
        assert!(hash2.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_user_id_anonymization_edge_cases() {
        // Test empty string
        let empty_hash = hash_user_id("");
        assert_eq!(empty_hash.len(), 64);
        
        // Test very long string
        let long_id = "a".repeat(1000);
        let long_hash = hash_user_id(&long_id);
        assert_eq!(long_hash.len(), 64);
        
        // Test special characters
        let special_id = "user@123!#$%";
        let special_hash = hash_user_id(special_id);
        assert_eq!(special_hash.len(), 64);
        assert_ne!(special_hash, special_id);
    }

    // Helper function to create sample DM data for testing
    fn create_sample_dm_data() -> Vec<DmWrapper> {
        vec![
            DmWrapper {
                dm_conversation: DmConversation {
                    conversation_id: "3382-1132151165410455552".to_string(),
                    messages: vec![
                        DmMessage {
                            message_create: Some(DmMessageCreate {
                                id: Some("msg1".to_string()),
                                text: Some("Hello there!".to_string()),
                                created_at: Some("2023-01-01T10:00:00.000Z".to_string()),
                            }),
                        },
                    ],
                },
            },
            DmWrapper {
                dm_conversation: DmConversation {
                    conversation_id: "1132151165410455552-9876543210".to_string(),
                    messages: vec![
                        DmMessage {
                            message_create: Some(DmMessageCreate {
                                id: Some("msg2".to_string()),
                                text: Some("How are you?".to_string()),
                                created_at: Some("2023-01-01T10:05:00.000Z".to_string()),
                            }),
                        },
                    ],
                },
            },
        ]
    }

    // Helper function to create sample tweet data for testing
    fn create_sample_tweet_data() -> Vec<Tweet> {
        vec![
            Tweet {
                id_str: "tweet1".to_string(),
                favorite_count: "5".to_string(),
                full_text: "Hello world!".to_string(),
                in_reply_to_status_id: None,
                retweeted: false,
                in_reply_to_screen_name: Some("alice".to_string()),
                retweet_count: "2".to_string(),
                created_at: "Mon Jan 01 10:00:00 +0000 2023".to_string(),
            },
            Tweet {
                id_str: "tweet2".to_string(),
                favorite_count: "3".to_string(),
                full_text: "Another tweet".to_string(),
                in_reply_to_status_id: None,
                retweeted: false,
                in_reply_to_screen_name: Some("bob".to_string()),
                retweet_count: "1".to_string(),
                created_at: "Mon Jan 01 11:00:00 +0000 2023".to_string(),
            },
        ]
    }

    #[test]
    fn test_extract_unique_users_from_dms() {
        let sample_dm_data = create_sample_dm_data();
        let analyzer = RelationshipAnalyzer::new();
        
        let users = analyzer.extract_users_from_dms(&sample_dm_data);
        
        // Should extract 3 unique users: "3382", "1132151165410455552", "9876543210"
        assert_eq!(users.len(), 3);
        assert!(users.contains(&hash_user_id("3382")));
        assert!(users.contains(&hash_user_id("1132151165410455552")));
        assert!(users.contains(&hash_user_id("9876543210")));
    }

    #[test]
    fn test_extract_users_from_tweets() {
        let sample_tweet_data = create_sample_tweet_data();
        let analyzer = RelationshipAnalyzer::new();
        
        let users = analyzer.extract_users_from_tweets(&sample_tweet_data);
        
        // Should extract 2 unique users from in_reply_to_screen_name: "alice", "bob"
        assert_eq!(users.len(), 2);
        assert!(users.contains(&hash_user_id("alice")));
        assert!(users.contains(&hash_user_id("bob")));
    }

    #[test]
    fn test_handle_empty_data_gracefully() {
        let analyzer = RelationshipAnalyzer::new();
        
        // Test with empty DM data
        let empty_dm_data: Vec<DmWrapper> = vec![];
        let dm_users = analyzer.extract_users_from_dms(&empty_dm_data);
        assert_eq!(dm_users.len(), 0);
        
        // Test with empty tweet data
        let empty_tweet_data: Vec<Tweet> = vec![];
        let tweet_users = analyzer.extract_users_from_tweets(&empty_tweet_data);
        assert_eq!(tweet_users.len(), 0);
    }

    #[test]
    fn test_extract_users_from_malformed_conversation_ids() {
        let malformed_dm_data = vec![
            DmWrapper {
                dm_conversation: DmConversation {
                    conversation_id: "no_dash_here".to_string(), // No dash separator
                    messages: vec![],
                },
            },
            DmWrapper {
                dm_conversation: DmConversation {
                    conversation_id: "user1-user2".to_string(), // Valid format
                    messages: vec![],
                },
            },
        ];
        
        let analyzer = RelationshipAnalyzer::new();
        let users = analyzer.extract_users_from_dms(&malformed_dm_data);
        
        // Should only extract from the valid conversation ID
        assert_eq!(users.len(), 2);
        assert!(users.contains(&hash_user_id("user1")));
        assert!(users.contains(&hash_user_id("user2")));
    }

    // Helper function to create sample conversation data for testing
    fn create_sample_conversation_data() -> Vec<DmWrapper> {
        vec![
            DmWrapper {
                dm_conversation: DmConversation {
                    conversation_id: "user123-target_user".to_string(),
                    messages: vec![
                        DmMessage {
                            message_create: Some(DmMessageCreate {
                                id: Some("msg1".to_string()),
                                text: Some("First message".to_string()),
                                created_at: Some("2023-01-01T10:00:00.000Z".to_string()),
                            }),
                        },
                        DmMessage {
                            message_create: Some(DmMessageCreate {
                                id: Some("msg2".to_string()),
                                text: Some("Second message".to_string()),
                                created_at: Some("2023-01-01T11:00:00.000Z".to_string()),
                            }),
                        },
                        DmMessage {
                            message_create: Some(DmMessageCreate {
                                id: Some("msg3".to_string()),
                                text: Some("Third message".to_string()),
                                created_at: Some("2023-01-01T12:00:00.000Z".to_string()),
                            }),
                        },
                    ],
                },
            },
        ]
    }

    #[test]
    fn test_create_basic_user_profile() {
        let sample_data = create_sample_conversation_data();
        let analyzer = RelationshipAnalyzer::new();
        let user_hash = hash_user_id("target_user");
        
        let profile = analyzer.create_user_profile(&user_hash, &sample_data);
        
        assert_eq!(profile.user_hash, user_hash);
        assert!(profile.dm_stats.total_messages > 0);
        assert!(profile.first_interaction.is_some());
        assert!(profile.last_interaction.is_some());
        assert!(profile.first_interaction <= profile.last_interaction);
    }

    #[test]
    fn test_dm_statistics_calculation() {
        let sample_data = create_sample_conversation_data();
        let analyzer = RelationshipAnalyzer::new();
        let user_hash = hash_user_id("target_user");
        
        let stats = analyzer.calculate_dm_statistics(&user_hash, &sample_data);
        
        assert_eq!(stats.total_messages, 3);
        assert_eq!(stats.messages_sent, 3); // All counted as sent for now
        assert_eq!(stats.messages_received, 0); // Not implemented yet
    }

    #[test]
    fn test_profile_with_no_interactions() {
        let empty_data: Vec<DmWrapper> = vec![];
        let analyzer = RelationshipAnalyzer::new();
        let user_hash = hash_user_id("nonexistent_user");
        
        let profile = analyzer.create_user_profile(&user_hash, &empty_data);
        
        assert_eq!(profile.user_hash, user_hash);
        assert_eq!(profile.dm_stats.total_messages, 0);
        assert!(profile.first_interaction.is_none());
        assert!(profile.last_interaction.is_none());
    }

    #[test]
    fn test_interaction_timespan_calculation() {
        let sample_data = create_sample_conversation_data();
        let analyzer = RelationshipAnalyzer::new();
        let user_hash = hash_user_id("target_user");
        
        let (first, last) = analyzer.find_interaction_timespan(&user_hash, &sample_data);
        
        assert!(first.is_some());
        assert!(last.is_some());
        
        let first_time = first.unwrap();
        let last_time = last.unwrap();
        
        // Should be chronologically ordered
        assert!(first_time <= last_time);
        
        // Should match our test data timestamps
        assert_eq!(first_time.format("%Y-%m-%d").to_string(), "2023-01-01");
        assert_eq!(last_time.format("%Y-%m-%d").to_string(), "2023-01-01");
    }

    // Helper function to check if timeline is chronologically sorted (newest first)
    fn is_chronologically_sorted(timeline: &[InteractionEvent]) -> bool {
        timeline.windows(2).all(|w| w[0].timestamp >= w[1].timestamp)
    }

    #[test]
    fn test_build_interaction_timeline() {
        let dm_data = create_sample_dm_data();
        let tweet_data = create_sample_tweet_data();
        let analyzer = RelationshipAnalyzer::new();
        
        let timeline = analyzer.build_timeline(&dm_data, &tweet_data);
        
        assert!(!timeline.is_empty());
        assert!(is_chronologically_sorted(&timeline));
        assert!(timeline.iter().any(|e| matches!(e.event_type, InteractionType::DmSent)));
        assert!(timeline.iter().any(|e| matches!(e.event_type, InteractionType::TweetReply)));
    }

    #[test]
    fn test_timeline_event_creation() {
        let dm_message = DmMessage {
            message_create: Some(DmMessageCreate {
                id: Some("test_msg_123".to_string()),
                text: Some("Test message content".to_string()),
                created_at: Some("2023-01-01T10:00:00.000Z".to_string()),
            }),
        };
        let conversation_id = "user1-user2";
        
        let event = InteractionEvent::from_dm_message(&dm_message, conversation_id);
        
        assert!(event.is_some());
        let event = event.unwrap();
        
        assert_eq!(event.event_type, InteractionType::DmSent);
        assert_eq!(event.participants.len(), 2);
        assert!(event.metadata.contains_key("message_id"));
        assert!(event.metadata.contains_key("conversation_id"));
        assert!(event.metadata.contains_key("text_length"));
        assert_eq!(event.metadata.get("message_id").unwrap(), "test_msg_123");
    }

    #[test]
    fn test_timeline_sorting() {
        let dm_data = vec![
            DmWrapper {
                dm_conversation: DmConversation {
                    conversation_id: "user1-user2".to_string(),
                    messages: vec![
                        DmMessage {
                            message_create: Some(DmMessageCreate {
                                id: Some("msg1".to_string()),
                                text: Some("First message".to_string()),
                                created_at: Some("2023-01-01T10:00:00.000Z".to_string()),
                            }),
                        },
                        DmMessage {
                            message_create: Some(DmMessageCreate {
                                id: Some("msg2".to_string()),
                                text: Some("Second message".to_string()),
                                created_at: Some("2023-01-01T12:00:00.000Z".to_string()),
                            }),
                        },
                    ],
                },
            },
        ];
        
        let tweet_data = vec![
            Tweet {
                id_str: "tweet1".to_string(),
                favorite_count: "5".to_string(),
                full_text: "Tweet in between".to_string(),
                in_reply_to_status_id: None,
                retweeted: false,
                in_reply_to_screen_name: Some("alice".to_string()),
                retweet_count: "2".to_string(),
                created_at: "Mon Jan 01 11:00:00 +0000 2023".to_string(),
            },
        ];
        
        let analyzer = RelationshipAnalyzer::new();
        let timeline = analyzer.build_timeline(&dm_data, &tweet_data);
        
        // Should be sorted newest first
        assert!(is_chronologically_sorted(&timeline));
        assert_eq!(timeline.len(), 3);
        
        // The order should be: msg2 (12:00), tweet1 (11:00), msg1 (10:00)
        assert!(timeline[0].timestamp > timeline[1].timestamp);
        assert!(timeline[1].timestamp > timeline[2].timestamp);
    }
}