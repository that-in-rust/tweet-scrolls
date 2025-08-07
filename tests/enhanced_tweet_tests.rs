use tweet_scrolls::processing::data_structures::{Tweet, TweetWrapper, TweetEntities};
use tweet_scrolls::utils::tweet_classifier::{classify_tweet_type, generate_twitter_url};
use tweet_scrolls::models::tweet_classification::TweetType;

#[cfg(test)]
mod enhanced_tweet_tests {
    use super::*;

    fn create_test_tweet(id: &str, text: &str, reply_to_id: Option<&str>, reply_to_user: Option<&str>) -> Tweet {
        Tweet {
            id_str: id.to_string(),
            id: id.to_string(),
            full_text: text.to_string(),
            created_at: "Mon Jan 01 12:00:00 +0000 2023".to_string(),
            favorite_count: "0".to_string(),
            retweet_count: "0".to_string(),
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
    fn test_classify_original_tweet() {
        let tweet = create_test_tweet("1", "Hello world", None, None);
        assert_eq!(classify_tweet_type(&tweet, "testuser"), TweetType::Original);
    }

    #[test]
    fn test_classify_reply_to_user() {
        let tweet = create_test_tweet("2", "Reply to myself", Some("1"), Some("testuser"));
        assert_eq!(classify_tweet_type(&tweet, "testuser"), TweetType::ReplyToUser);
    }

    #[test]
    fn test_classify_reply_to_others() {
        let tweet = create_test_tweet("3", "Reply to someone else", Some("1"), Some("otheruser"));
        assert_eq!(classify_tweet_type(&tweet, "testuser"), TweetType::ReplyToOthers);
    }

    #[test]
    fn test_generate_twitter_url() {
        let tweet = create_test_tweet("123456789", "Test tweet", None, None);
        assert_eq!(generate_twitter_url(&tweet, "testuser"), "https://twitter.com/testuser/status/123456789");
    }
}