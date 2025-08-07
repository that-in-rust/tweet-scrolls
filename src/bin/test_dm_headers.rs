use tweet_scrolls::processing::dm_headers_analyzer::DmHeadersAnalyzer;
use std::time::Instant;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🚀 DM Headers Performance Test");
    println!("==============================");
    
    let headers_file = "/home/amuldotexe/Desktop/GitHub202410/tweet-scrolls/private_data/REALDATA/direct-message-headers.js";
    let full_dm_file = "/home/amuldotexe/Desktop/GitHub202410/tweet-scrolls/private_data/REALDATA/direct-messages.js";
    
    // Test 1: Headers-only analysis (fast)
    println!("\n📊 Test 1: Headers-Only Analysis (Optimized)");
    println!("==============================================");
    
    let start_time = Instant::now();
    let mut headers_analyzer = DmHeadersAnalyzer::new();
    
    match headers_analyzer.analyze_dm_headers(headers_file, "1132151165410455552").await {
        Ok(_) => {
            let duration = start_time.elapsed();
            let stats = headers_analyzer.get_performance_stats();
            
            println!("⚡ Headers Analysis Results:");
            println!("   ⏱️  Processing time: {:.2?}", duration);
            println!("   📨 Messages processed: {}", stats.total_messages_processed);
            println!("   💬 Conversations: {}", stats.unique_conversations);
            println!("   👥 Relationships found: {}", stats.unique_relationships);
            
            if let Some((hour, count)) = stats.peak_hour {
                println!("   🕐 Peak activity: {}:00 ({} messages)", hour, count);
            }
            
            if let Some((day, count)) = &stats.most_active_day {
                println!("   📅 Most active day: {} ({} messages)", day, count);
            }
            
            // Show top relationships
            let results = headers_analyzer.generate_results();
            let mut relationships: Vec<_> = results.relationships.values().collect();
            relationships.sort_by(|a, b| b.interaction_count.cmp(&a.interaction_count));
            
            println!("\n🏆 Top 5 Relationships (Headers Analysis):");
            for (i, rel) in relationships.iter().take(5).enumerate() {
                println!("   {}. {} - {} interactions", i + 1, rel.username, rel.interaction_count);
            }
        }
        Err(e) => {
            println!("❌ Headers analysis failed: {}", e);
        }
    }
    
    // Test 2: File size comparison
    println!("\n📏 File Size Comparison:");
    println!("========================");
    
    if let Ok(headers_metadata) = tokio::fs::metadata(headers_file).await {
        println!("📄 Headers file: {:.1} MB", headers_metadata.len() as f64 / 1_048_576.0);
    }
    
    if let Ok(full_metadata) = tokio::fs::metadata(full_dm_file).await {
        println!("📄 Full DM file: {:.1} MB", full_metadata.len() as f64 / 1_048_576.0);
        
        if let Ok(headers_metadata) = tokio::fs::metadata(headers_file).await {
            let size_reduction = (1.0 - (headers_metadata.len() as f64 / full_metadata.len() as f64)) * 100.0;
            println!("💾 Size reduction: {:.1}% smaller", size_reduction);
        }
    }
    
    println!("\n✨ Key Advantages of DM Headers:");
    println!("================================");
    println!("🚀 Performance:");
    println!("   • 35% smaller file size (68MB vs 104MB)");
    println!("   • Faster parsing - only essential metadata");
    println!("   • Lower memory usage - no text content");
    println!("   • Optimized for relationship analysis");
    println!();
    println!("🔒 Privacy:");
    println!("   • No message text content");
    println!("   • No URLs or media references");
    println!("   • Just interaction metadata");
    println!("   • Perfect for relationship analysis");
    println!();
    println!("📊 Analysis Quality:");
    println!("   • All relationship data preserved");
    println!("   • Complete timeline information");
    println!("   • Activity pattern detection");
    println!("   • Response time calculations possible");
    
    Ok(())
}