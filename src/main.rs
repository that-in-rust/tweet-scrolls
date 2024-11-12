use anyhow::{Context, Result};
use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::Path;
use std::time::Instant;
use tokio::fs as async_fs;
use tokio::sync::mpsc;
use tokio::task;
use csv::Writer as CsvWriterLib;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[tokio::main]
async fn main() -> Result<()> {
    let input_file = get_input_file()?;
    let screen_name = get_screen_name()?;
    let timestamp = Utc::now().timestamp();

    println!("🕶️ Current working directory: {}", std::env::current_dir()?.display());

    if !async_fs::metadata(&input_file).await.is_ok() {
        anyhow::bail!("❌ File does not exist: {}", input_file);
    }
    //  |   |          |          |       |
    //  |   |          |          |       Error message with file path
    //  |   |          |          Check if file exists
    //  |   |          Await async operation
    //  |   Get metadata of input file
    //  Negate the result to check if file does not exist

    // Create output directory
    let input_path = Path::new(&input_file);
    //  |           |      |
    //  |           |      Convert input file path to Path
    //  |           Path::new() creates a new Path instance
    //  Variable to hold the Path instance
    let output_dir = input_path.parent().unwrap().join(format!("output_{}_{}", screen_name, timestamp));
    //  |           |          |      |     |      |
    //  |           |          |      |     |      Timestamp value
    //  |           |          |      |     Screen name value
    //  |           |          |      String formatting with {}
    //  |           |          join() adds path component
    //  |           Gets parent directory, unwraps Option
    //  Output directory PathBuf
    
    // Memory layout:
    // input_path: "~/data/tweets.json"
    //      |
    //      v
    // parent(): "~/data"
    //      |
    //      v 
    // join(): "~/data/output_alice_1234567890"

    async_fs::create_dir_all(&output_dir).await.context("Failed to create output directory")?;

    // Create a channel for CsvWriter
    let (tx, rx) = mpsc::channel::<Vec<String>>(100);

    // Initialize CsvWriter and spawn its run task
    let csv_writer = CsvWriter::new(output_dir.join(format!("threads_{}_{}.csv", screen_name, timestamp)).to_str().unwrap().to_string(), rx, 100);
    tokio::spawn(csv_writer.run());

    println!("🌟 Avengers, assemble! Initiating Operation: Tweet Processing...");
    if let Err(e) = process_tweets(&input_file, &screen_name, tx, &output_dir, timestamp).await {
        eprintln!("🚨 Mission Failed: {}", e);
    } else {
        println!("🎉 Victory! Tweets have been successfully processed and organized.");
    }

    Ok(())
}

fn get_input_file() -> Result<String> {
    prompt_input("🗂️ Please enter the absolute path to the input JSON file: ")
}

fn get_screen_name() -> Result<String> {
    prompt_input("🕵️‍♂️ Please enter the Twitter handle: ")
}

fn prompt_input(prompt: &str) -> Result<String> {
    print!("{}", prompt);
    io::stdout().flush().context("Failed to flush stdout")?;
    let mut input = String::new();
    io::stdin().read_line(&mut input).context("Failed to read input")?;
    Ok(input.trim().to_string())
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Tweet {
    pub id_str: String,
    pub favorite_count: String,
    pub full_text: String,
    pub in_reply_to_status_id: Option<String>,
    pub retweeted: bool,
    pub in_reply_to_screen_name: Option<String>,
    pub retweet_count: String,
    pub created_at: String,
}

#[derive(Deserialize, Debug)]
pub struct TweetWrapper {
    pub tweet: Tweet,
}

#[derive(Debug)]
pub struct Thread {
    pub id: String,
    pub tweets: Vec<Tweet>,
}

pub struct CsvWriter {
    output_path: String,
    receiver: mpsc::Receiver<Vec<String>>,
    buffer_size: usize,
}

impl CsvWriter {
    pub fn new(output_path: String, receiver: mpsc::Receiver<Vec<String>>, buffer_size: usize) -> Self {
        Self {
            output_path,
            receiver,
            buffer_size,
        }
    }

    pub async fn run(mut self) -> Result<()> {
        let file = File::create(&self.output_path)
            .with_context(|| format!("Failed to create file: {}", self.output_path))?;
        let mut writer = CsvWriterLib::from_writer(BufWriter::new(file));

        // Write headers
        writer.write_record(&[
            "Thread ID",
            "Date time of first tweet",
            "Number of Tweets in Thread",
            "Likes in first tweet",
            "Retweets in first tweet",
            "Total likes for all tweets",
            "Total retweets for all tweets",
            "Thread Text",
        ])?;

        let mut buffer = Vec::with_capacity(self.buffer_size);

        while let Some(record) = self.receiver.recv().await {
            buffer.push(record);
            if buffer.len() >= self.buffer_size {
                self.flush_buffer(&mut writer, &mut buffer)?;
            }
        }

        if !buffer.is_empty() {
            self.flush_buffer(&mut writer, &mut buffer)?;
        }

        writer.flush()?;
        Ok(())
    }

    fn flush_buffer(&self, writer: &mut CsvWriterLib<BufWriter<File>>, buffer: &mut Vec<Vec<String>>) -> Result<()> {
        for record in buffer.drain(..) {
            writer.write_record(&record)?;
        }
        Ok(())
    }
}

pub async fn process_tweets(input_file: &str, screen_name: &str, csv_tx: mpsc::Sender<Vec<String>>, output_dir: &Path, _timestamp: i64) -> Result<()> {
    let screen_name = screen_name.to_string(); // Clone to own the String

    let start_datetime = Local::now();
    let timestamp = Utc::now().timestamp();

    println!("🕰️ Avengers, assemble! Mission start time: {}", start_datetime.format("%Y-%m-%d %H:%M:%S"));
    let start_time = Instant::now();

    println!("🕵️‍♀️ Black Widow is infiltrating the enemy base (reading the file)...");
    let script_content = async_fs::read_to_string(input_file).await.context("Failed to read input file")?;
    println!("📂 Intelligence gathered. File size: {} bytes", script_content.len());

    println!("🧠 Tony and Bruce are decoding the alien artifact (parsing JSON)...");
    let json_start = script_content.find('[').context("Invalid JSON format: missing opening bracket")?;
    let json_end = script_content.rfind(']').context("Invalid JSON format: missing closing bracket")?;
    let json_content = &script_content[json_start..=json_end];
    let tweets: Vec<TweetWrapper> = from_str(json_content).context("Failed to parse JSON")?;
    let total_tweets = tweets.len();
    println!("🎉 Decoding complete! We've identified {} potential threats (tweets).", total_tweets);

    println!("🇺🇸 Captain America is assembling the strike team (filtering tweets)...");
    let mut tweets: Vec<Tweet> = tweets.into_iter().map(|tw| tw.tweet).collect();
    let initial_tweet_count = tweets.len();
    tweets.retain(|tweet| !tweet.retweeted && (tweet.in_reply_to_screen_name.as_deref() == Some(&screen_name) || tweet.in_reply_to_screen_name.is_none()));
    let filtered_tweet_count = initial_tweet_count - tweets.len();
    println!("👥 Strike team assembled. {} members are on standby, {} are joining the mission.", filtered_tweet_count, tweets.len());

    println!("📡 Shuri is establishing secure comms (organizing tweets)...");
    let tweets_map: HashMap<String, Tweet> = tweets.into_iter().map(|t| (t.id_str.clone(), t)).collect();
    println!("🔐 Secure network established. We can now track {} individual operatives.", tweets_map.len());

    println!("🕴️ Nick Fury is forming tactical units (grouping tweets into conversations)...");
    let screen_name_clone = screen_name.clone();
    let threads = task::spawn_blocking(move || {
        let mut threads: Vec<Vec<Tweet>> = Vec::new();
        for tweet in tweets_map.values() {
            if tweet.in_reply_to_status_id.is_none() || tweet.in_reply_to_screen_name.as_deref() != Some(&screen_name_clone) {
                let mut thread = vec![tweet.clone()];
                let mut current_id = tweet.id_str.clone();
                while let Some(reply) = tweets_map.values().find(|t| t.in_reply_to_status_id.as_deref() == Some(&current_id)) {
                    thread.push(reply.clone());
                    current_id = reply.id_str.clone();
                }
                threads.push(thread);
            }
        }
        threads
    }).await?;

    println!("👥 Tactical units formed. We have {} specialized teams ready for action.", threads.len());

    println!("🔮 Dr. Strange is using the Time Stone to prioritize our missions (sorting threads)...");
    let mut threads = threads;
    threads.sort_by(|a, b| {
        let date_a = DateTime::parse_from_str(&a[0].created_at, "%a %b %d %H:%M:%S %z %Y").unwrap();
        let date_b = DateTime::parse_from_str(&b[0].created_at, "%a %b %d %H:%M:%S %z %Y").unwrap();
        date_b.cmp(&date_a)
    });
    println!("⏳ Timelines analyzed. Most critical missions identified.");

    println!("📝 Agent Coulson is documenting our missions (writing threads to files)...");
    let threads: Vec<Thread> = threads.into_iter().map(|thread| {
        let id = thread[0].id_str.clone();
        Thread { id, tweets: thread }
    }).collect();

    // Handle writing to files
    write_threads_to_file(&threads, &screen_name, timestamp, output_dir).await?;
    write_csv(&threads, &screen_name, timestamp, csv_tx).await?;

    let end_datetime = Local::now();
    let end_time = Instant::now();
    let duration = end_time.duration_since(start_time);

    println!("🌍 Director Fury is compiling the final mission report...");
    let results_content = format!(
        "Avengers Operation Summary\n\
         ===========================\n\
         Mission Start: {}\n\
         Total Threats Identified: {}\n\
         Threats Neutralized (Filtered): {}\n\
         Successful Interventions (Final Thread Count): {}\n\
         Mission End: {}\n\
         Operation Duration: {:.2} seconds\n\
         ===========================\n\
         Status: Mission Accomplished",
        start_datetime.format("%Y-%m-%d %H:%M:%S"),
        total_tweets,
        filtered_tweet_count,
        threads.len(),
        end_datetime.format("%Y-%m-%d %H:%M:%S"),
        duration.as_secs_f64()
    );

    let results_file_path = output_dir.join(format!("results_{}_{}.txt", screen_name, timestamp));
    async_fs::write(&results_file_path, results_content).await.context("Failed to write results file")?;
    println!("📊 Final mission report filed. Operation summary complete!");

    Ok(())
}

async fn write_threads_to_file(threads: &[Thread], screen_name: &str, timestamp: i64, output_dir: &Path) -> Result<()> {
    let file_path = output_dir.join(format!("threads_{}_{}.txt", screen_name, timestamp));
    let file = File::create(&file_path)?;
    let mut writer = BufWriter::new(file);

    for thread in threads {
        writeln!(writer, "--- Start of Thread ---")?;
        writeln!(writer, "Thread ID: {}", thread.id)?;
        writeln!(writer, "Timestamp: {}", thread.tweets[0].created_at)?;
        writeln!(writer, "Public Support: {} retweets, {} likes",
                 thread.tweets[0].retweet_count, thread.tweets[0].favorite_count)?;
        writeln!(writer, "Thread text:")?;

        for (i, tweet) in thread.tweets.iter().enumerate() {
            writeln!(writer, "- Tweet {}:", i + 1)?;
            writeln!(writer, "{}", tweet.full_text)?;
            writeln!(writer)?;
        }

        writeln!(writer, "--- End of Thread ---\n")?;
    }

    writer.flush()?;
    Ok(())
}

async fn write_csv(
    threads: &[Thread],
    screen_name: &str,
    timestamp: i64,
    csv_tx: mpsc::Sender<Vec<String>>,
) -> Result<()> {
    for thread in threads {
        let first_tweet = &thread.tweets[0];
        let total_likes: u32 = thread.tweets.iter().filter_map(|t| t.favorite_count.parse::<u32>().ok()).sum();
        let total_retweets: u32 = thread.tweets.iter().filter_map(|t| t.retweet_count.parse::<u32>().ok()).sum();
        let thread_text: String = thread.tweets.iter().map(|t| t.full_text.replace('\n', " ")).collect::<Vec<_>>().join(" ");

        let record = vec![
            thread.id.clone(),
            first_tweet.created_at.clone(),
            thread.tweets.len().to_string(),
            first_tweet.favorite_count.clone(),
            first_tweet.retweet_count.clone(),
            total_likes.to_string(),
            total_retweets.to_string(),
            thread_text,
        ];

        csv_tx.send(record).await?;
    }

    Ok(())
}


