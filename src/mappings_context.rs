use std::collections::HashMap;
use Node;

#[derive(Clone, Debug)]
pub struct MappingsContext {
    pub sources_indices: HashMap<String, usize>,
    pub sources_content: HashMap<String, Node>,
    pub has_source_content: bool,
    pub current_original_line: usize,
    pub current_source: usize,
    pub unfinished_generated_line: usize,
}

impl MappingsContext {
    pub fn new() -> Self {
        MappingsContext {
            sources_indices: HashMap::new(),
            sources_content: HashMap::new(),
            has_source_content: false,
            current_original_line: 1,
            current_source: 0,
            unfinished_generated_line: 0,
        }
    }

    pub fn ensure_source(&mut self, src: Option<String>, original_source: Option<Node>) -> usize {
        let src = match src {
            Some(s) => s,
            None => String::new(),
        };
        let original_source = match original_source {
            Some(s) => s,
            None => Node::NString(String::new()),
        };
        if self.sources_indices.contains_key(&src) {
            *self.sources_indices.get(&src).unwrap()
        } else {
            let sources_indices_len = self.sources_indices.len();
            if let Node::NString(_) = original_source {
                self.has_source_content = true;
            }
            self.sources_content.insert(src.clone(), original_source);
            self.sources_indices.insert(src, sources_indices_len);
            sources_indices_len
        }
    }

    pub fn get_arrays(&self) -> Srcs {
        let mut sources: Vec<String> = Vec::new();
        let mut sources_content: Vec<Node> = Vec::new();
        for (key, val) in self.sources_content.clone() {
            if !key.is_empty() {
                sources.push(key);
                sources_content.push(val);
            }
        }
        Srcs {
            sources,
            sources_content,
        }
    }
}

pub struct Srcs {
    pub sources: Vec<String>,
    pub sources_content: Vec<Node>,
}
