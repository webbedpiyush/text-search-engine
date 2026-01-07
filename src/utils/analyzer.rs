use rust_stemmers::{Algorithm , Stemmer};
use std::collections::HashSet;

pub struct Analyzer {
  stemmer : Stemmer,
  stopWords: HashSet<String>,
}

impl Analyzer {
  pub fn new() -> Self {
    let words = stop_words::get(stop_words::LANGUAGE::English);

    Self {
      stemmer: Stemmer::create(Algorithm::English),
      stopWords:words.into_iter().collect(),
    }
  }

  pub fn analyze(&self ,text: &str) -> Vec<String> {
    text.to_lowercase()
    .split(|c :char| !c.is_alphanumeric())
    .filter(|token| {
      !token.is_empty() &&
      token.len() > 2 &&
      !self.stopWords.contains(*token)
    })
    .map(|token| self.stemmer.stem(token).to_string()) 
    .collect()
  }
}
