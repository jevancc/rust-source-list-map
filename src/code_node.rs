extern crate vlq;

use std::str;
use helpers;
use mappings_context::MappingsContext;
use ::Node;

#[derive(Clone, Debug)]
pub struct CodeNode {
    generated_code: String,
}

impl CodeNode {
    pub fn new(generated_code: String) -> Self {
        CodeNode {
            generated_code: generated_code
        }
    }

    pub fn add_generated_code(&mut self, generated_code: &str) {
		self.generated_code += generated_code;
	}

    pub fn map_generated_code(&self, f: &Fn(String) -> String) -> CodeNode {
        let generated_code = f(self.clone().generated_code);
        CodeNode::new(generated_code)
	}

    pub fn merge(mut self, other_node: Node) -> Option<Node> {
        match other_node {
            Node::NCodeNode(n) => {
                self.generated_code += &n.generated_code;
                Some(Node::NCodeNode(self))
            }
            _ => None,
        }
    }

    pub fn get_generated_code(&self) -> &str {
        &self.generated_code
    }

    pub fn get_mappings(&mut self, mappings_context: &mut MappingsContext) -> String {
        let lines = helpers::number_of_lines(&self.generated_code);
        let mut mappings: String = ";".repeat(lines);

        if lines > 0 {
            mappings_context.unfinished_generated_line = helpers::get_unfinished_lines(&self.generated_code);
            if mappings_context.unfinished_generated_line > 0 {
                mappings += "A";
            }
        } else {
            let prev_unfinished = mappings_context.unfinished_generated_line;
            mappings_context.unfinished_generated_line += helpers::get_unfinished_lines(&self.generated_code);
            if prev_unfinished == 0 && mappings_context.unfinished_generated_line > 0 {
                mappings = String::from("A");
            } else {
                mappings = String::from("");
            }
        }
        mappings
    }

    pub fn get_normalized_nodes(&self) -> Vec<CodeNode> {
        vec![self.clone()]
    }
}
