use tweet_scrolls::utils::tweet_classifier::{classify_tweet_type, generate_twitter_url, create_reply_context};
use tweet_scrolls::models::tweet_classification::TweetType;
use tweet_scrolls::processing::data_structures::{Tweet, TweetEntities};

#[cfg(test)]
mod working_enhanced_features_tests {
    use super::*;

    fn create_test_tweet(id: &str, text: &str, reply_to_id: Option<&str>, reply_to_user: Option<&str>) -> Tweet {
        Tweet {
            id_str: id.to_string(),
            id: id.to_string(),
            full_text: text.to_string(),
            created_at: "Mon Jan 01 12:00:00 +0000 2023".to_string(),
            favorite_count: "5".to_string(),
            retweet_count: "2".to_string(),
            retweeted: false,
            favorited: false,
            truncated: false,
            lang: "en".to_string(),
            source: "Twitter Web App".to_string(),
            display_text_range: vec!["0".to_string(), text.len().to_string()],
            in_reply_to_status_id: reply_to_id.map(|s| s.to_string()),
            in_reply_to_status_id_str: reply_to_id.map(|s| s.to_string()),
            in_reply_to_user_id: None,
            in_reply_to_user_id_str: None,
            in_reply_to_screen_name: reply_to_user.map(|s| s.to_string()),
            edit_info: None,
            entities: TweetEntities {
                hashtags: vec![],
                symbols: vec![],
                user_mentions: vec![],
                urls: vec![],
            },
            possibly_sensitive: None,
        }
    }

    #[test]
    fn test_enhanced_tweet_processing_pipeline() {
        // Test the complete enhanced tweet processing pipeline
        let screen_name = "testuser";
        
        // Create test tweets representing a thread
        let original_tweet = create_test_tweet("1", "This is my original thought", None, None);
        let reply_tweet = create_test_tweet("2", "Adding more context to my thought", Some("1"), Some("testuser"));
        let reply_to_other = create_test_tweet("3", "Replying to someone else", Some("999"), Some("otheruser"));
        
        // Test classification
        assert_eq!(classify_tweet_type(&original_tweet, screen_name), TweetType::Original);
        assert_eq!(classify_tweet_type(&reply_tweet, screen_name), TweetType::ReplyToUser);
        assert_eq!(classify_tweet_type(&reply_to_other, screen_name), TweetType::ReplyToOthers);
        
        // Test URL generation
        assert_eq!(
            generate_twitter_url(&original_tweet, screen_name),
            "https://twitter.com/testuser/status/1"
        );
        assert_eq!(
            generate_twitter_url(&reply_tweet, screen_name),
            "https://twitter.com/testuser/status/2"
        );
        
        // Test reply context
        assert_eq!(create_reply_context(&original_tweet), None);
        assert_eq!(
            create_reply_context(&reply_tweet),
            Some("Reply to @testuser (1)".to_string())
        );
        assert_eq!(
            create_reply_context(&reply_to_other),
            Some("Reply to @otheruser (999)".to_string())
        );
    }

    #[test]
    fn test_enhanced_csv_data_preparation() {
        // Test that we can prepare data for enhanced CSV output
        let screen_name = "testuser";
        let tweet = create_test_tweet("123", "Hello world", None, None);
        
        // Simulate CSV record creation
        let tweet_type = classify_tweet_type(&tweet, screen_name);
        let twitter_url = generate_twitter_url(&tweet, screen_name);
        let reply_context = create_reply_context(&tweet).unwrap_or_default();
        
        // Verify the data is correctly prepared
        assert_eq!(format!("{:?}", tweet_type), "Original");
        assert_eq!(twitter_url, "https://twitter.com/testuser/status/123");
        assert_eq!(reply_context, "");
        
        // Verify tweet data is accessible
        assert_eq!(tweet.id_str, "123");
        assert_eq!(tweet.full_text, "Hello world");
        assert_eq!(tweet.favorite_count, "5");
        assert_eq!(tweet.retweet_count, "2");
        assert_eq!(tweet.lang, "en");
        assert_eq!(tweet.source, "Twitter Web App");
    }

    #[test]
    fn test_thread_context_simulation() {
        // Test simulating thread context for enhanced processing
        let screen_name = "testuser";
        
        // Create a multi-tweet thread
        let tweets = vec![
            create_test_tweet("1", "First tweet in thread", None, None),
            create_test_tweet("2", "Second tweet in thread", Some("1"), Some("testuser")),
            create_test_tweet("3", "Third tweet in thread", Some("2"), Some("testuser")),
        ];
        
        // Simulate thread processing
        let mut thread_data = Vec::new();
        for (position, tweet) in tweets.iter().enumerate() {
            let tweet_type = classify_tweet_type(tweet, screen_name);
            let twitter_url = generate_twitter_url(tweet, screen_name);
            let reply_context = create_reply_context(tweet).unwrap_or_default();
            
            thread_data.push((
                tweet.id_str.clone(),
                tweet.full_text.clone(),
                format!("{:?}", tweet_type),
                position + 1,
                twitter_url,
                reply_context,
            ));
        }
        
        // Verify thread structure
        assert_eq!(thread_data.len(), 3);
        
        // First tweet should be original
        assert_eq!(thread_data[0].2, "Original");
        assert_eq!(thread_data[0].3, 1); // position
        assert_eq!(thread_data[0].5, ""); // no reply context
        
        // Second tweet should be reply to user
        assert_eq!(thread_data[1].2, "ReplyToUser");
        assert_eq!(thread_data[1].3, 2); // position
        assert_eq!(thread_data[1].5, "Reply to @testuser (1)");
        
        // Third tweet should be reply to user
        assert_eq!(thread_data[2].2, "ReplyToUser");
        assert_eq!(thread_data[2].3, 3); // position
        assert_eq!(thread_data[2].5, "Reply to @testuser (2)");
    }
}