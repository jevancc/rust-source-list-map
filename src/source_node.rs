use helpers;
use mappings_context::MappingsContext;
use single_line_node::SingleLineNode;
use std::rc::Rc;
use std::str;
use vlq;
use Node;
use StringPtr;

#[derive(Clone, Debug)]
pub struct SourceNode {
    pub generated_code: String,
    pub original_source: Option<Rc<String>>,
    pub source: Option<Rc<String>>,
    pub starting_line: usize,
    pub _number_of_lines: usize,
    pub _ends_with_new_line: bool,
}

impl SourceNode {
    pub fn new(
        generated_code: String,
        source: Option<StringPtr>,
        original_source: Option<StringPtr>,
        starting_line: usize,
    ) -> Self {
        let source = source.map(|sp| sp.to_ptr());
        let original_source = original_source.map(|sp| sp.to_ptr());

        SourceNode {
            _ends_with_new_line: generated_code.ends_with('\n'),
            _number_of_lines: helpers::number_of_lines(&generated_code),
            generated_code,
            original_source,
            source,
            starting_line,
        }
    }

    pub fn add_generated_code(&mut self, code: &str) {
        self.generated_code += code;
        self._number_of_lines += helpers::number_of_lines(code);
        self._ends_with_new_line = code.ends_with('\n');
    }

    // pub fn map_generated_code(&self, fn_name: &str) -> SourceNode {
    // }

    pub fn merge(self, other_node: &Node) -> Result<Node, Node> {
        match other_node {
            Node::NSourceNode(n) => self.merge_source_node(n),
            Node::NSingleLineNode(n) => self.merge_single_line_node(n),
            _ => Err(Node::NSourceNode(self)),
        }
    }

    fn merge_source_node(mut self, other_node: &SourceNode) -> Result<Node, Node> {
        if self.source == other_node.source
            && self._ends_with_new_line
            && self.starting_line + self._number_of_lines == other_node.starting_line
        {
            self.generated_code += &other_node.generated_code;
            self._number_of_lines += other_node._number_of_lines;
            self._ends_with_new_line = other_node._ends_with_new_line;
            Ok(Node::NSourceNode(self))
        } else {
            Err(Node::NSourceNode(self))
        }
    }

    fn merge_single_line_node(mut self, other_node: &SingleLineNode) -> Result<Node, Node> {
        if self.source == other_node.source
            && self._ends_with_new_line
            && self.starting_line + self._number_of_lines == other_node.line
            && other_node._number_of_lines <= 1
        {
            self.add_single_line_node(other_node);
            Ok(Node::NSourceNode(self))
        } else {
            Err(Node::NSourceNode(self))
        }
    }

    fn add_single_line_node(&mut self, other_node: &SingleLineNode) -> &SourceNode {
        self.generated_code += &other_node.generated_code;
        self._number_of_lines += other_node._number_of_lines;
        self._ends_with_new_line = other_node._ends_with_new_line;
        self
    }

    pub fn get_generated_code(&self) -> &str {
        &self.generated_code
    }

    pub fn get_mappings(&self, mappings_context: &mut MappingsContext) -> String {
        if self.generated_code.is_empty() {
            String::new()
        } else {
            let line_mapping = ";AACA";
            let lines = self._number_of_lines;
            let source_index = mappings_context.ensure_source(
                self.source.clone(),
                self.original_source.clone().map(|p| Node::NRcString(p)),
            );
            let mut mappings = String::from("A");
            if mappings_context.unfinished_generated_line != 0 {
                mappings = String::from(",");
                let mut buf = Vec::<u8>::new();
                vlq::encode(mappings_context.unfinished_generated_line as i64, &mut buf).unwrap();
                mappings += str::from_utf8(&buf).unwrap();
            }
            let mut buf = Vec::<u8>::new();
            vlq::encode(
                source_index as i64 - mappings_context.current_source as i64,
                &mut buf,
            ).unwrap();
            vlq::encode(
                self.starting_line as i64 - mappings_context.current_original_line as i64,
                &mut buf,
            ).unwrap();
            buf.push(b'A');
            mappings += str::from_utf8(&buf).unwrap();

            mappings_context.current_source = source_index;
            mappings_context.current_original_line = self.starting_line + lines;
            mappings_context.current_original_line -= 1;

            let unfinished_generated_line = helpers::get_unfinished_lines(&self.generated_code);
            mappings_context.unfinished_generated_line = unfinished_generated_line;
            if lines > 0 {
                mappings += &line_mapping.repeat(lines.saturating_sub(1));
            }

            if unfinished_generated_line == 0 {
                mappings += ";";
            } else {
                if lines != 0 {
                    mappings += line_mapping;
                }
                mappings_context.current_original_line += 1;
            }
            mappings
        }
    }

    pub fn get_normalized_nodes(&self) -> Vec<SingleLineNode> {
        let mut results = Vec::<SingleLineNode>::new();
        let mut current_line = self.starting_line;
        let mut lines = self.generated_code.lines().peekable();

        while let Some(line) = lines.next() {
            let line_code = if lines.peek().is_some() || self._ends_with_new_line {
                String::from(line) + "\n"
            } else {
                String::from(line)
            };

            results.push(SingleLineNode::new(
                line_code,
                self.source.clone().map(|p| StringPtr::Ptr(p)),
                self.original_source.clone().map(|p| StringPtr::Ptr(p)),
                current_line,
            ));
            current_line += 1;
        }
        results
    }
}
