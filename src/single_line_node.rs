extern crate vlq;

use std::str;
use helpers;
use mappings_context::MappingsContext;
use source_node::SourceNode;
use ::Node;

#[derive(Clone, Debug)]
pub struct SingleLineNode {
    pub generated_code: String,
	pub original_source: String,
	pub source: String,
	pub line: usize,
	pub _number_of_lines: usize,
	pub _ends_with_new_line: bool,
}

impl SingleLineNode {
    pub fn new(generated_code: String,
               source: String,
               original_source: String,
               line: usize) -> Self {
        SingleLineNode {
            original_source: original_source,
            source: source,
            line: line,
            _number_of_lines: helpers::number_of_lines(&generated_code),
            _ends_with_new_line: generated_code.ends_with("\n"),
            generated_code: generated_code,
        }
    }

    pub fn map_generated_code(&self, f: &Fn(String) -> String) -> SingleLineNode {
		let generated_code = f(self.clone().generated_code);
        SingleLineNode::new(generated_code,
                            self.source.clone(),
                            self.original_source.clone(),
                            self.line)
	}

    pub fn merge(self, other_node: Node) -> Option<Node> {
        match other_node {
            Node::NSingleLineNode(n) => {
                self.merge_single_line_node(n)
            }
            _ => None,
        }
    }

    fn merge_single_line_node(mut self, other_node: SingleLineNode) -> Option<Node> {
        if self.source == other_node.source &&
           self.original_source == other_node.original_source {
            if self.line == other_node.line {
                self.generated_code += &other_node.generated_code;
                self._number_of_lines += other_node._number_of_lines;
                self._ends_with_new_line = other_node._ends_with_new_line;
                Some(Node::NSingleLineNode(self))
            } else if self.line + 1 == other_node.line && self._ends_with_new_line &&
                      self._number_of_lines == 1 &&
                      other_node._number_of_lines <= 1 {
                Some(Node::NSourceNode(SourceNode::new(self.generated_code + &other_node.generated_code,
                                                       self.source,
                                                       self.original_source,
                                                       self.line)))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn add_single_line_node(&mut self, other_node: SingleLineNode) {
        // self.generated_code += &other_node.generated_code;
        // self.number_of_lines += other_node.number_of_lines;
        // self.ends_with_new_line = other_node.ends_with_new_line;
        // self
    }

    pub fn get_generated_code(&self) -> &str {
        &self.generated_code
    }

    pub fn get_mappings(&mut self, mappings_context: &mut MappingsContext) -> String {
        if self.generated_code.is_empty() {
            String::from("")
        } else {
            let line_mapping = ";AAAA";
            let lines = self._number_of_lines;
            let source_index = mappings_context
                               .ensure_source(self.source.clone(),
                                              Node::NString(self.original_source.clone()));

            let mut mappings = String::from("A");
            if mappings_context.unfinished_generated_line != 0 {
                let mut buf = Vec::<u8>::new();
                vlq::encode(mappings_context.unfinished_generated_line as i64, &mut buf).unwrap();
                mappings = String::from(",");
                mappings += str::from_utf8(&buf).unwrap();
            }
            let mut buf = Vec::<u8>::new();
            vlq::encode(
                source_index as i64 - mappings_context.current_source as i64,
                &mut buf).unwrap();
            vlq::encode(
                self.line as i64 - mappings_context.current_original_line as i64,
                &mut buf).unwrap();
            buf.push('A' as u8); // original column 0
            mappings += str::from_utf8(&buf).unwrap();

            mappings_context.current_source = source_index;
		    mappings_context.current_original_line = self.line;

            let unfinished_generated_line = helpers::get_unfinished_lines(&self.generated_code);
            mappings_context.unfinished_generated_line = unfinished_generated_line;
            if lines > 0 {
                mappings += &line_mapping.repeat(lines.wrapping_sub(1));
            }

            if mappings_context.unfinished_generated_line == 0 {
    			mappings += ";";
    		} else if lines != 0 {
    			mappings += line_mapping;
    		}
    		mappings
        }
    }

    pub fn get_normalized_nodes(&self) -> Vec<SingleLineNode> {
		vec![self.clone()]
	}
}
