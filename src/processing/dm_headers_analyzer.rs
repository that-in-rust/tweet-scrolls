use crate::models::dm_headers::{DmHeaderWrapper, DmHeaderMessage};
use crate::processing::mvp_analyzer::SimpleRelationship;
use anyhow::{Result, Context};
use chrono::{DateTime, Utc, Timelike, Weekday, Datelike};
use std::collections::HashMap;
use tokio::fs;

/// Fast DM relationship analyzer using headers-only data
pub struct DmHeadersAnalyzer {
    pub relationships: HashMap<String, SimpleRelationship>,
    pub hourly_activity: HashMap<u32, u32>,
    pub daily_activity: HashMap<String, u32>,
    pub total_messages: u32,
    pub unique_conversations: u32,
}

impl DmHeadersAnalyzer {
    pub fn new() -> Self {
        Self {
            relationships: HashMap::new(),
            hourly_activity: HashMap::new(),
            daily_activity: HashMap::new(),
            total_messages: 0,
            unique_conversations: 0,
        }
    }

    /// Process DM headers file for fast relationship analysis
    pub async fn analyze_dm_headers(&mut self, file_path: &str, user_id: &str) -> Result<()> {
        println!("🚀 Fast DM Analysis: Using headers-only data for optimal performance");
        
        // Read and parse the headers file
        let content = fs::read_to_string(file_path).await
            .context("Failed to read DM headers file")?;
        
        let json_content = self.extract_json_content(&content)?;
        
        // Try parsing with better error handling
        let dm_data: Vec<DmHeaderWrapper> = serde_json::from_str(&json_content)
            .with_context(|| {
                let sample = &json_content[..json_content.len().min(200)];
                format!("Failed to parse DM headers JSON. Sample: {}", sample)
            })?;

        println!("📊 Processing {} conversations (headers only)...", dm_data.len());
        
        self.unique_conversations = dm_data.len() as u32;
        
        // Process each conversation
        for conversation in dm_data {
            self.process_conversation_headers(&conversation, user_id)?;
        }

        println!("✅ Fast analysis complete!");
        println!("   📨 Total messages: {}", self.total_messages);
        println!("   💬 Conversations: {}", self.unique_conversations);
        println!("   👥 Unique relationships: {}", self.relationships.len());
        
        Ok(())
    }

    fn extract_json_content(&self, content: &str) -> Result<String> {
        // Remove JavaScript prefix for DM headers
        let start_marker = "window.YTD.direct_message_headers.part0 = ";
        
        let start_pos = content.find(start_marker)
            .context("Could not find DM headers start marker")?;
        let json_start = start_pos + start_marker.len();
        
        // Find the last closing bracket
        let end_pos = content.rfind(']')
            .context("Could not find DM headers end marker")?;
        let json_end = end_pos + 1;
        
        Ok(content[json_start..json_end].to_string())
    }

    fn process_conversation_headers(&mut self, conversation: &DmHeaderWrapper, user_id: &str) -> Result<()> {
        let conversation_id = &conversation.dm_conversation.conversation_id;
        
        // Extract participant IDs from conversation ID (format: "user1-user2")
        let participants: Vec<&str> = conversation_id.split('-').collect();
        let other_participant = participants.iter()
            .find(|&&p| p != user_id)
            .unwrap_or(&"unknown");

        // Process each message header
        for message in &conversation.dm_conversation.messages {
            self.process_message_header(message, user_id, other_participant)?;
        }

        Ok(())
    }

    fn process_message_header(&mut self, message: &DmHeaderMessage, user_id: &str, other_participant: &str) -> Result<()> {
        let msg_create = &message.message_create;
        self.total_messages += 1;

        // Parse timestamp for activity analysis
        if let Ok(timestamp) = DateTime::parse_from_rfc3339(&msg_create.created_at) {
            let utc_time: DateTime<Utc> = timestamp.into();
            
            // Track hourly activity
            let hour = utc_time.hour();
            *self.hourly_activity.entry(hour).or_insert(0) += 1;
            
            // Track daily activity
            let weekday = match utc_time.weekday() {
                Weekday::Mon => "Monday",
                Weekday::Tue => "Tuesday", 
                Weekday::Wed => "Wednesday",
                Weekday::Thu => "Thursday",
                Weekday::Fri => "Friday",
                Weekday::Sat => "Saturday",
                Weekday::Sun => "Sunday",
            };
            *self.daily_activity.entry(weekday.to_string()).or_insert(0) += 1;
        }

        // Update relationship data
        let username = format!("@user_{}", other_participant);
        let relationship = self.relationships.entry(username.clone()).or_insert(SimpleRelationship {
            username: username.clone(),
            interaction_count: 0,
            last_interaction: msg_create.created_at.clone(),
            interaction_type: "dms".to_string(),
        });

        relationship.interaction_count += 1;
        // Keep the most recent interaction timestamp
        if msg_create.created_at > relationship.last_interaction {
            relationship.last_interaction = msg_create.created_at.clone();
        }

        Ok(())
    }

    /// Generate analysis results compatible with the main system
    pub fn generate_results(&self) -> DmHeadersAnalysisResult {
        DmHeadersAnalysisResult {
            relationships: self.relationships.clone(),
            hourly_activity: self.hourly_activity.clone(),
            daily_activity: self.daily_activity.clone(),
        }
    }

    /// Get performance statistics
    pub fn get_performance_stats(&self) -> DmHeadersPerformanceStats {
        DmHeadersPerformanceStats {
            total_messages_processed: self.total_messages,
            unique_conversations: self.unique_conversations,
            unique_relationships: self.relationships.len() as u32,
            peak_hour: self.hourly_activity.iter()
                .max_by_key(|(_, &count)| count)
                .map(|(&hour, &count)| (hour, count)),
            most_active_day: self.daily_activity.iter()
                .max_by_key(|(_, &count)| count)
                .map(|(day, &count)| (day.clone(), count)),
        }
    }
}

/// Analysis results from DM headers processing
#[derive(Debug, Clone)]
pub struct DmHeadersAnalysisResult {
    pub relationships: HashMap<String, SimpleRelationship>,
    pub hourly_activity: HashMap<u32, u32>,
    pub daily_activity: HashMap<String, u32>,
}

/// Performance statistics for DM headers analysis
#[derive(Debug)]
pub struct DmHeadersPerformanceStats {
    pub total_messages_processed: u32,
    pub unique_conversations: u32,
    pub unique_relationships: u32,
    pub peak_hour: Option<(u32, u32)>,
    pub most_active_day: Option<(String, u32)>,
}

impl Default for DmHeadersAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}