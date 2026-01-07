mod utils;

use utils::analyzer::Analyzer;
use utils::engine::SearchEngine;
use utils::indexer;
use clap::{Parser, Subcommand};
use std::fs::File;

// it tells rust to write all cli code into struct
#[derive(Parser)]
#[command(name = "text-se-engine")]
#[command(about = "Rust Search Engine")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Index { path: String },
    Search { query: String },
}

fn main() {
    let cli = Cli::parse();
    let analyzer = Analyzer::new();

    match cli.command {
        Commands::Index {path} => {
            let engine = indexer::run(&path , &analyzer);
            let file = File::create("index.bin").expect("...");
            bincode::serialize_into(file, &engine).expect("...");
        }

        Commands::Search { query } => {
            let file = File::open("index.bin").expect("...");
            let engine :SearchEngine = bincode::deserialize_from(file).expect("...");
            let results = engine.search(&query , &analyzer);

            if results.is_empty() {
                println!("No results found for: {}",query);
            } else {
                for (i , doc) in results.iter().enumerate() {
                    println!("{}. {}",i+1,doc.title);
                    println!("URL: {}\n", doc.url)
                }
            }
        }
    }
}