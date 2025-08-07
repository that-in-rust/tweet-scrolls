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

    // Write CSV file
    let csv_path = output_dir.join(format!("dm_conversations_{}_{}.csv", _screen_name, _timestamp));
    let csv_file = File::create(&csv_path)?;
    let mut csv_writer = CsvWriterLib::from_writer(BufWriter::new(csv_file));
    
    csv_writer.write_record(&[
        "Conversation ID",
        "Message Count", 
        "First Message Date",
        "Last Message Date",
    ])?;
    
    for conv in &conversations {
        csv_writer.write_record(&[
            &conv.id,
            &conv.message_count.to_string(),
            conv.first_message_date.as_deref().unwrap_or("N/A"),
            conv.last_message_date.as_deref().unwrap_or("N/A"),
        ])?;
    }
    csv_writer.flush()?;
    
    // Write TXT file
    let txt_path = output_dir.join(format!("dm_conversations_{}_{}.txt", _screen_name, _timestamp));
    let txt_file = File::create(&txt_path)?;
    let mut txt_writer = BufWriter::new(txt_file);
    
    for (i, conv) in conversations.iter().enumerate() {
        writeln!(txt_writer, "--- Conversation {} ---", i + 1)?;
        writeln!(txt_writer, "ID: {}", conv.id)?;
        writeln!(txt_writer, "Messages: {}", conv.message_count)?;
        if let Some(first) = &conv.first_message_date {
            writeln!(txt_writer, "First Message: {}", first)?;
        }
        if let Some(last) = &conv.last_message_date {
            writeln!(txt_writer, "Last Message: {}", last)?;
        }
        writeln!(txt_writer)?;
    }
    txt_writer.flush()?;
    
    // Write summary
    let duration = start_time.elapsed();
    let total_messages: usize = conversations.iter().map(|c| c.message_count).sum();
    
    let summary = format!(
        "DM Processing Summary\n\
         ====================\n\
         Total Conversations: {}\n\
         Total Messages: {}\n\
         Processing Time: {:.2} seconds\n\
         ====================\n\
         Status: Complete",
        conversations.len(),
        total_messages,
        duration.as_secs_f64()
    );
    
    let summary_path = output_dir.join(format!("dm_results_{}_{}.txt", _screen_name, _timestamp));
    async_fs::write(&summary_path, summary).await.context("Failed to write DM summary")?;
    
    println!("✅ DM processing complete! Generated {} files", 3);
    Ok(())
}

/// Calculate response times between consecutive messages in a conversation
pub fn calculate_response_times(messages: &[DmMessage]) -> Vec<std::time::Duration> {
    let mut response_times = Vec::new();
    let mut timestamps = Vec::new();
    
    // Collect valid timestamps
    for message in messages {
        if let Some(message_create) = &message.message_create {
            if let Some(created_at) = &message_create.created_at {
                if let Ok(timestamp) = DateTime::parse_from_rfc3339(created_at) {
                    timestamps.push(timestamp.with_timezone(&Utc));
                }
            }
        }
    }
    
    // Calculate response times between consecutive messages
    for i in 1..timestamps.len() {
        let duration = timestamps[i].signed_duration_since(timestamps[i-1]);
        if let Ok(std_duration) = duration.to_std() {
            response_times.push(std_duration);
        }
    }
    
    response_times
}

/// Calculate average response time for a conversation
pub fn calculate_average_response_time(messages: &[DmMessage]) -> std::time::Duration {
    let response_times = calculate_response_times(messages);
    
    if response_times.is_empty() {
        return std::time::Duration::from_secs(0);
    }
    
    let total_nanos: u128 = response_times.iter().map(|d| d.as_nanos()).sum();
    let avg_nanos = total_nanos / response_times.len() as u128;
    
    std::time::Duration::from_nanos(avg_nanos as u64)
}

/// Analyze hourly activity patterns from interaction events
pub fn analyze_hourly_activity(events: &[InteractionEvent]) -> Vec<usize> {
    let mut hourly_counts = vec![0; 24];
    
    for event in events {
        let hour = event.timestamp.hour() as usize;
        hourly_counts[hour] += 1;
    }
    
    hourly_counts
}

/// Find the most active day of the week from interaction events
pub fn find_most_active_day(events: &[InteractionEvent]) -> Option<chrono::Weekday> {
    use chrono::Weekday;
    use std::collections::HashMap;
    
    let mut day_counts = HashMap::new();
    
    for event in events {
        let weekday = event.timestamp.weekday();
        *day_counts.entry(weekday).or_insert(0) += 1;
    }
    
    day_counts.into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(day, _)| day)
}
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
                                created_at: Some("2023-01-15T10:30:00.000Z".to_string()),
                            }),
                        },
                        DmMessage {
                            message_create: Some(DmMessageCreate {
                                id: Some("msg2".to_string()),
                                text: Some("How are you?".to_string()),
                                created_at: Some("2023-01-15T11:00:00.000Z".to_string()),
                            }),
                        },
                    ],
                },
            },
            DmWrapper {
                dm_conversation: DmConversation {
                    conversation_id: "9876543210-1132151165410455552".to_string(),
                    messages: vec![
                        DmMessage {
                            message_create: Some(DmMessageCreate {
                                id: Some("msg3".to_string()),
                                text: Some("Another conversation".to_string()),
                                created_at: Some("2023-02-01T14:20:00.000Z".to_string()),
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
                full_text: "Hello @testuser!".to_string(),
                in_reply_to_status_id: None,
                retweeted: false,
                in_reply_to_screen_name: Some("testuser".to_string()),
                retweet_count: "2".to_string(),
                created_at: "Sun Jan 15 10:30:00 +0000 2023".to_string(),
            },
            Tweet {
                id_str: "tweet2".to_string(),
                favorite_count: "10".to_string(),
                full_text: "Just a regular tweet".to_string(),
                in_reply_to_status_id: None,
                retweeted: false,
                in_reply_to_screen_name: None,
                retweet_count: "3".to_string(),
                created_at: "Mon Jan 16 15:45:00 +0000 2023".to_string(),
            },
        ]
    }

    // Phase 1 Tests - User Extraction & Basic Profiling

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
        let user_id2 = "9876543210";
        
        let hash1 = hash_user_id(user_id1);
        let hash2 = hash_user_id(user_id2);
        
        assert_ne!(hash1, hash2); // Different inputs produce different hashes
        assert_eq!(hash1.len(), 64);
        assert_eq!(hash2.len(), 64);
    }

    #[test]
    fn test_user_id_anonymization_edge_cases() {
        // Test empty string
        let empty_hash = hash_user_id("");
        assert_eq!(empty_hash.len(), 64);
        
        // Test very long string
        let long_string = "a".repeat(1000);
        let long_hash = hash_user_id(&long_string);
        assert_eq!(long_hash.len(), 64);
        
        // Test special characters
        let special_hash = hash_user_id("user@#$%^&*()");
        assert_eq!(special_hash.len(), 64);
    }

    #[test]
    fn test_extract_unique_users_from_dms() {
        let sample_dm_data = create_sample_dm_data();
        let analyzer = RelationshipAnalyzer::new();
        
        let users = analyzer.extract_users_from_dms(&sample_dm_data);
        
        assert_eq!(users.len(), 3); // "3382", "1132151165410455552", "9876543210"
        assert!(users.contains(&hash_user_id("3382")));
        assert!(users.contains(&hash_user_id("1132151165410455552")));
        assert!(users.contains(&hash_user_id("9876543210")));
    }

    #[test]
    fn test_extract_users_from_tweets() {
        let sample_tweet_data = create_sample_tweet_data();
        let analyzer = RelationshipAnalyzer::new();
        
        let users = analyzer.extract_users_from_tweets(&sample_tweet_data);
        
        assert_eq!(users.len(), 1); // Only "testuser" from reply
        assert!(users.contains(&hash_user_id("testuser")));
    }

    #[test]
    fn test_handle_empty_data_gracefully() {
        let analyzer = RelationshipAnalyzer::new();
        
        let empty_dm_data: Vec<DmWrapper> = vec![];
        let empty_tweet_data: Vec<Tweet> = vec![];
        
        let dm_users = analyzer.extract_users_from_dms(&empty_dm_data);
        let tweet_users = analyzer.extract_users_from_tweets(&empty_tweet_data);
        
        assert!(dm_users.is_empty());
        assert!(tweet_users.is_empty());
    }

    #[test]
    fn test_extract_users_from_malformed_conversation_ids() {
        let malformed_dm_data = vec![
            DmWrapper {
                dm_conversation: DmConversation {
                    conversation_id: "invalid_format".to_string(), // No dash
                    messages: vec![],
                },
            },
            DmWrapper {
                dm_conversation: DmConversation {
                    conversation_id: "user1-user2-user3".to_string(), // Multiple dashes
                    messages: vec![],
                },
            },
        ];
        
        let analyzer = RelationshipAnalyzer::new();
        let users = analyzer.extract_users_from_dms(&malformed_dm_data);
        
        // Should handle malformed IDs gracefully
        // First case: no dash, no users extracted
        // Second case: takes first dash, extracts "user1" and "user2-user3"
        assert_eq!(users.len(), 2);
        assert!(users.contains(&hash_user_id("user1")));
        assert!(users.contains(&hash_user_id("user2-user3")));
    }

    #[test]
    fn test_create_basic_user_profile() {
        let sample_data = create_sample_dm_data();
        let analyzer = RelationshipAnalyzer::new();
        let user_hash = hash_user_id("3382");
        
        let profile = analyzer.create_user_profile(&user_hash, &sample_data);
        
        assert_eq!(profile.user_hash, user_hash);
        assert!(profile.dm_stats.total_messages > 0);
        assert!(profile.first_interaction.is_some());
        assert!(profile.last_interaction.is_some());
        assert!(profile.first_interaction <= profile.last_interaction);
    }

    #[test]
    fn test_dm_statistics_calculation() {
        let sample_data = create_sample_dm_data();
        let analyzer = RelationshipAnalyzer::new();
        let user_hash = hash_user_id("3382");
        
        let stats = analyzer.calculate_dm_statistics(&user_hash, &sample_data);
        
        assert_eq!(stats.total_messages, 2); // Two messages in first conversation
        assert_eq!(stats.messages_sent, 2); // Currently counting all as sent
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
        let sample_data = create_sample_dm_data();
        let analyzer = RelationshipAnalyzer::new();
        let user_hash = hash_user_id("1132151165410455552");
        
        let profile = analyzer.create_user_profile(&user_hash, &sample_data);
        
        assert!(profile.first_interaction.is_some());
        assert!(profile.last_interaction.is_some());
        
        // Should span from first message (2023-01-15T10:30:00.000Z) to last (2023-02-01T14:20:00.000Z)
        let first = profile.first_interaction.unwrap();
        let last = profile.last_interaction.unwrap();
        assert!(first < last);
    }

    // Phase 2 Tests - Timeline Construction

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
                id: Some("test_msg".to_string()),
                text: Some("Test message".to_string()),
                created_at: Some("2023-01-15T10:30:00.000Z".to_string()),
            }),
        };
        
        let event = InteractionEvent::from_dm_message(&dm_message, "user1-user2");
        
        assert!(event.is_some());
        let event = event.unwrap();
        assert_eq!(event.event_type, InteractionType::DmSent);
        assert_eq!(event.participants.len(), 2);
        assert!(event.metadata.contains_key("message_id"));
        assert_eq!(event.metadata.get("message_id").unwrap(), "test_msg");
    }

    #[test]
    fn test_timeline_sorting() {
        let dm_data = create_sample_dm_data();
        let tweet_data = create_sample_tweet_data();
        let analyzer = RelationshipAnalyzer::new();
        
        let timeline = analyzer.build_timeline(&dm_data, &tweet_data);
        
        // Timeline should be sorted newest first
        for i in 1..timeline.len() {
            assert!(timeline[i-1].timestamp >= timeline[i].timestamp);
        }
    }

    // Helper function to check if timeline is chronologically sorted (newest first)
    fn is_chronologically_sorted(timeline: &[InteractionEvent]) -> bool {
        for i in 1..timeline.len() {
            if timeline[i-1].timestamp < timeline[i].timestamp {
                return false;
            }
        }
        true
    }

    // Phase 2 Tests - Response Time Analysis

    #[test]
    fn test_response_time_calculation() {
        let conversation = create_sample_conversation_with_timestamps();
        let response_times = calculate_response_times(&conversation);
        
        assert_eq!(response_times.len(), 2); // 3 messages = 2 response times
        assert!(response_times.iter().all(|&rt| rt > std::time::Duration::from_secs(0)));
    }

    #[test]
    fn test_average_response_time() {
        let conversation = create_sample_conversation_with_timestamps();
        let avg_response_time = calculate_average_response_time(&conversation);
        
        assert!(avg_response_time > std::time::Duration::from_secs(0));
        assert!(avg_response_time < std::time::Duration::from_hours(24)); // Reasonable upper bound
    }

    #[test]
    fn test_response_time_with_single_message() {
        let single_message_conversation = vec![
            DmMessage {
                message_create: Some(DmMessageCreate {
                    id: Some("msg1".to_string()),
                    text: Some("Single message".to_string()),
                    created_at: Some("2023-01-15T10:30:00.000Z".to_string()),
                }),
            },
        ];
        
        let response_times = calculate_response_times(&single_message_conversation);
        assert!(response_times.is_empty()); // No response times for single message
    }

    // Helper function to create conversation with specific timestamps for response time testing
    fn create_sample_conversation_with_timestamps() -> Vec<DmMessage> {
        vec![
            DmMessage {
                message_create: Some(DmMessageCreate {
                    id: Some("msg1".to_string()),
                    text: Some("First message".to_string()),
                    created_at: Some("2023-01-15T10:00:00.000Z".to_string()),
                }),
            },
            DmMessage {
                message_create: Some(DmMessageCreate {
                    id: Some("msg2".to_string()),
                    text: Some("Response after 30 minutes".to_string()),
                    created_at: Some("2023-01-15T10:30:00.000Z".to_string()),
                }),
            },
            DmMessage {
                message_create: Some(DmMessageCreate {
                    id: Some("msg3".to_string()),
                    text: Some("Another response after 1 hour".to_string()),
                    created_at: Some("2023-01-15T11:30:00.000Z".to_string()),
                }),
            },
        ]
    }

    // Phase 3 Tests - Communication Pattern Analysis

    #[test]
    fn test_activity_by_hour_analysis() {
        let events = create_events_across_different_hours();
        let hourly_activity = analyze_hourly_activity(&events);
        
        assert_eq!(hourly_activity.len(), 24);
        assert!(hourly_activity.iter().any(|&count| count > 0));
        
        // Check that the specific hours we created events for have activity
        assert!(hourly_activity[10] > 0); // 10 AM
        assert!(hourly_activity[14] > 0); // 2 PM
        assert!(hourly_activity[20] > 0); // 8 PM
    }

    #[test]
    fn test_most_active_day_detection() {
        let events = create_events_across_week();
        let most_active_day = find_most_active_day(&events);
        
        // Should return a valid weekday
        assert!(matches!(most_active_day, Some(_)));
    }

    #[test]
    fn test_communication_frequency_analysis() {
        let dm_data = create_sample_dm_data();
        let analyzer = RelationshipAnalyzer::new();
        let user_hash = hash_user_id("3382");
        
        let frequency = analyzer.calculate_communication_frequency(&user_hash, &dm_data);
        
        assert!(frequency.messages_per_day >= 0.0);
        assert!(frequency.active_days > 0);
    }

    // Helper functions for pattern analysis tests
    fn create_events_across_different_hours() -> Vec<InteractionEvent> {
        vec![
            InteractionEvent {
                timestamp: DateTime::parse_from_rfc3339("2023-01-15T10:30:00.000Z").unwrap().with_timezone(&Utc),
                event_type: InteractionType::DmSent,
                participants: vec![hash_user_id("user1"), hash_user_id("user2")],
                metadata: HashMap::new(),
            },
            InteractionEvent {
                timestamp: DateTime::parse_from_rfc3339("2023-01-15T14:15:00.000Z").unwrap().with_timezone(&Utc),
                event_type: InteractionType::DmSent,
                participants: vec![hash_user_id("user1"), hash_user_id("user2")],
                metadata: HashMap::new(),
            },
            InteractionEvent {
                timestamp: DateTime::parse_from_rfc3339("2023-01-15T20:45:00.000Z").unwrap().with_timezone(&Utc),
                event_type: InteractionType::TweetReply,
                participants: vec![hash_user_id("user1"), hash_user_id("user3")],
                metadata: HashMap::new(),
            },
        ]
    }

    fn create_events_across_week() -> Vec<InteractionEvent> {
        vec![
            // Monday events
            InteractionEvent {
                timestamp: DateTime::parse_from_rfc3339("2023-01-16T10:00:00.000Z").unwrap().with_timezone(&Utc), // Monday
                event_type: InteractionType::DmSent,
                participants: vec![hash_user_id("user1")],
                metadata: HashMap::new(),
            },
            InteractionEvent {
                timestamp: DateTime::parse_from_rfc3339("2023-01-16T15:00:00.000Z").unwrap().with_timezone(&Utc), // Monday
                event_type: InteractionType::DmSent,
                participants: vec![hash_user_id("user1")],
                metadata: HashMap::new(),
            },
            // Tuesday events
            InteractionEvent {
                timestamp: DateTime::parse_from_rfc3339("2023-01-17T11:00:00.000Z").unwrap().with_timezone(&Utc), // Tuesday
                event_type: InteractionType::TweetReply,
                participants: vec![hash_user_id("user2")],
                metadata: HashMap::new(),
            },
            // Wednesday events (most active)
            InteractionEvent {
                timestamp: DateTime::parse_from_rfc3339("2023-01-18T09:00:00.000Z").unwrap().with_timezone(&Utc), // Wednesday
                event_type: InteractionType::DmSent,
                participants: vec![hash_user_id("user1")],
                metadata: HashMap::new(),
            },
            InteractionEvent {
                timestamp: DateTime::parse_from_rfc3339("2023-01-18T12:00:00.000Z").unwrap().with_timezone(&Utc), // Wednesday
                event_type: InteractionType::DmSent,
                participants: vec![hash_user_id("user1")],
                metadata: HashMap::new(),
            },
            InteractionEvent {
                timestamp: DateTime::parse_from_rfc3339("2023-01-18T18:00:00.000Z").unwrap().with_timezone(&Utc), // Wednesday
                event_type: InteractionType::TweetReply,
                participants: vec![hash_user_id("user2")],
                metadata: HashMap::new(),
            },
        ]
    }
}

