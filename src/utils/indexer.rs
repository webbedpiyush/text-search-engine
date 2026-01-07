use super::engine::{SearchEngine, DocMetadata};
use super::analyzer::Analyzer;

use bzip2::read::MultiBzDecoder; 
use quick_xml::reader::Reader;
use quick_xml::events::Event;

use std::fs::File;
use std::io::BufReader;

fn clean_wikitext(raw: &str) -> String {
    if raw.to_lowercase().starts_with("#redirect") { return String::new(); }
    
    let mut clean = String::new();
    let mut in_braces = 0;
    
    for c in raw.chars() {
        match c {
            '{' => in_braces += 1,
            '}' => if in_braces > 0 { in_braces -= 1 },
            '[' | ']' | '\'' | '*' | '|' => continue,
            _ => {
                if in_braces == 0 {
                    clean.push(c);
                }
            }
        }
        if clean.len() > 500 { break; }
    }
    clean.trim().to_string()
}

pub fn run(path: &str, analyzer: &Analyzer) -> SearchEngine {
    println!("Opening file: {}", path);
    let file = File::open(path).expect("Failed to open file");
    
    let decoder = MultiBzDecoder::new(file);
    let buf_reader = BufReader::new(decoder);
    let mut reader = Reader::from_reader(buf_reader);
    
    reader.config_mut().trim_text(false);

    let mut engine = SearchEngine::default();
    let mut buffer = Vec::new();
    let mut current_id = 0;

    let mut in_title = false;
    let mut in_text = false;
    
    let mut title = String::new();
    let mut text_content = String::new();

    println!("Starting indexing... parsing raw Wikipedia dump.");

    loop {
        match reader.read_event_into(&mut buffer) {
            Ok(Event::Start(e)) => {
                match e.name().as_ref() {
                    b"title" => in_title = true,
                    b"text" => in_text = true,
                    _ => (),
                }
            },
            
            Ok(Event::Text(e)) => {
                if in_title {
                    title.push_str(&e.unescape().unwrap_or_default());
                }
                if in_text {
                    text_content.push_str(&e.unescape().unwrap_or_default());
                }
            },

            Ok(Event::End(e)) => {
                match e.name().as_ref() {
                    b"title" => in_title = false,
                    b"text" => in_text = false,
                    b"page" => {
                      
                        let abstract_summary = clean_wikitext(&text_content);

                        if !abstract_summary.is_empty() && !title.contains(":") {
                            let id = current_id;

                            engine.docs.insert(id, DocMetadata {
                                title: title.clone(),
                                url: format!("https://en.wikipedia.org/wiki/{}", title.replace(" ", "_")),
                                abstract_text: abstract_summary.clone(),
                            });

                            let content = format!("{} {}", title, abstract_summary);
                            let tokens = analyzer.analyze(&content);
                            
                            for token in tokens {
                                engine.index.entry(token).or_default().push(id);
                            }
                            
                            current_id += 1;
                            if current_id % 1000 == 0 {
                                println!("Indexed {} articles...", current_id);
                            }
                        }
                        
                        title.clear();
                        text_content.clear();
                    },
                    _ => (),
                }
            },
            
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buffer.clear();
    }
    
    println!("Indexing complete. Total articles: {}", current_id);
    engine
}