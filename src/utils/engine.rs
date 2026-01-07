use serde::{Deserialize , Serialize};
use std::collections::HashMap;

#[derive(Serialize , Deserialize, Clone , Debug)]
pub struct DocMetadata {
    pub title: String,
    pub url: String,
    pub abstract_text: String,
}


#[derive(Serialize ,Deserialize, Default)]
pub struct SearchEngine {
  pub index : HashMap<String , Vec<u32>>,
  pub docs : HashMap<u32 , DocMetadata>
}

impl SearchEngine {
  pub fn search(&self,query : &str , analyzer : &super::analyzer::Analyzer) -> Vec<DocMetadata> {
    let tokens = analyzer.analyze(query);
    if tokens.is_empty() {
      return vec![];
    }

    let mut resultIds = match self.index.get(&tokens[0]) {
      Some(ids) => ids.clone(),
      None => return vec![],
    };

    for token in tokens.iter().skip(1) {
      if let Some(ids) = self.index.get(token) {
        resultIds.retain(|id| ids.contains(id));
      } else {
        return vec![];
      }
    }

   resultIds.into_iter()
   .filter_map(|id| self.docs.get(&id))
   .cloned()
   .collect()
  }
}