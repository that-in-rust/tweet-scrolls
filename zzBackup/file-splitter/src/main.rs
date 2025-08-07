//! Command-line interface for the file-splitter utility

use clap::Parser;
use file_splitter::{SplitConfig, split_file, SplitError};
use std::process;
use tracing::{info, error, Level};
use tracing_subscriber::FmtSubscriber;

/// Split a file into smaller chunks
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file to split
    #[arg(short, long, value_name = "FILE")]
    input: String,

    /// Output directory for chunks (default: same as input file)
    #[arg(short, long, value_name = "DIR")]
    output_dir: Option<String>,

    /// Size of each chunk in bytes (e.g., 1K, 2M, 1G)
    #[arg(short, long, default_value = "1M")]
    chunk_size: String,

    /// Prefix for output filenames (default: input filename)
    #[arg(short, long)]
    prefix: Option<String>,

    /// Number of digits in chunk numbers (default: 3)
    #[arg(short = 'n', long, default_value_t = 3)]
    digits: u8,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn parse_size(size_str: &str) -> Result<u64, String> {
    let size_str = size_str.trim().to_uppercase();
    
    if size_str.is_empty() {
        return Err("Empty size string".to_string());
    }
    
    // Find the split between number and unit
    let split_pos = size_str.find(|c: char| !c.is_ascii_digit())
        .unwrap_or_else(|| size_str.len());
    
    let (num_str, unit) = size_str.split_at(split_pos);
    let num: u64 = num_str.parse().map_err(|e| format!("Invalid number: {}", e))?;
    
    let multiplier = match unit {
        "" | "B" => 1,
        "K" | "KB" => 1024,
        "M" | "MB" => 1024 * 1024,
        "G" | "GB" => 1024 * 1024 * 1024,
        _ => return Err(format!("Invalid unit: {}", unit)),
    };
    
    Ok(num * multiplier)
}

fn setup_logging(verbose: bool) {
    let level = if verbose {
        Level::DEBUG
    } else {
        Level::INFO
    };
    
    let subscriber = FmtSubscriber::builder()
        .with_max_level(level)
        .with_writer(std::io::stderr)
        .finish();
    
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set up logging");
}

fn main() {
    let args = Args::parse();
    setup_logging(args.verbose);
    
    if let Err(e) = run(args) {
        error!("Error: {}", e);
        process::exit(1);
    }
}

fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    // Parse chunk size
    let chunk_size = parse_size(&args.chunk_size)
        .map_err(|e| format!("Invalid chunk size: {}", e))?;
    
    // Build config
    let config = SplitConfig {
        input_path: args.input,
        output_dir: args.output_dir,
        chunk_size,
        prefix: args.prefix,
        digits: args.digits,
    };
    
    info!("Splitting file: {}", config.input_path);
    info!("Chunk size: {} bytes", config.chunk_size);
    if let Some(ref dir) = config.output_dir {
        info!("Output directory: {}", dir);
    }
    
    // Perform the split
    let result = match split_file(&config) {
        Ok(r) => r,
        Err(SplitError::Io(e)) => {
            return Err(format!("I/O error: {}", e).into());
        }
        Err(SplitError::InvalidChunkSize(msg)) => {
            return Err(format!("Invalid chunk size: {}", msg).into());
        }
        Err(SplitError::InvalidInputPath(msg)) => {
            return Err(format!("Invalid input path: {}", msg).into());
        }
        Err(SplitError::InvalidOutputDir(msg)) => {
            return Err(format!("Invalid output directory: {}", msg).into());
        }
    };
    
    // Print the result
    println!("\n{}", result);
    
    Ok(())
}
