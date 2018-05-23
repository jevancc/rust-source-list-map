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

    pub fn ensure_source(&mut self, src: String, original_source: Node) -> usize {
        // HACK: use borrow instead of clone
        let cloned_sources_indices = self.sources_indices.clone();
        match cloned_sources_indices.get(&src) {
            Some(si) => *si,
            None => {
                let idx = self.sources_indices.len();
                if let Node::NString(_) = original_source {
                    self.has_source_content = true;
                }
                self.sources_indices.insert(src.clone(), idx);
                self.sources_content.insert(src, original_source);
                idx
            }
        }
    }

    pub fn get_arrays(&self) -> Srcs {
        let mut sources: Vec<String> = Vec::new();
        let mut sources_content: Vec<Node> = Vec::new();
        for (key, val) in self.sources_content.clone() {
            sources.push(key);
            sources_content.push(val);
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
