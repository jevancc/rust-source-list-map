extern crate vlq;

use std::collections::HashMap;
use code_node::CodeNode;
use source_node::SourceNode;
use mappings_context::MappingsContext;
use Node;

#[derive(Clone, Debug)]
pub struct SourceListMap {
    pub children: Vec<Node>,
}

impl SourceListMap {
    pub fn new(generated_code: Option<GenCode>, source: &str, original_source: &str) -> Self {
        match generated_code {
            Some(GenCode::Code(c)) => {
                let mut slm = SourceListMap {
                    children: Vec::new()
                };
                slm.add(c, String::from(source), String::from(original_source));
                slm
            }
            Some(GenCode::CodeVec(cv)) => SourceListMap {
                children: cv
            },
            None => SourceListMap {
                children: Vec::new()
            },
        }
    }

    pub fn add(&mut self, generated_code: Node, source: String, original_source: String)
              -> &mut SourceListMap {
        match generated_code {
            Node::NString(s) => {
                if !source.is_empty() {
                    self.children
                        .push(Node::NSourceNode(SourceNode::new(s,
                                                                source,
                                                                original_source,
                                                                1)));
                } else {
                    // HACK: use borrow instead of clone
                    let cloned_children = self.children.clone();
                    match cloned_children.last() {
                        Some(Node::NCodeNode(_)) => {
                            let len = self.children.len();
                            let mut ln = self.children.get_mut(len - 1).unwrap();
                            if let Node::NCodeNode(ln) = ln {
                                ln.add_generated_code(&s);
                            }
                        }
                        _ => {
                            self.children.push(Node::NCodeNode(CodeNode::new(s)));
                        }
                    }
                }
            }
            Node::NCodeNode(cn) => {
                self.children.push(Node::NCodeNode(cn));
            }
            Node::NSourceNode(sn) => {
                self.children.push(Node::NSourceNode(sn));
            }
            Node::NSingleLineNode(sln) => {
                self.children.push(Node::NSingleLineNode(sln));
            }
            Node::NSourceListMap(slm) => {
                for child in slm.children {
                    self.children.push(child);
                }
            }
        }
        self
    }

    pub fn prepend(&mut self, generated_code: Node, source: String, original_source: String)
                  -> &mut SourceListMap {
        match generated_code {
            Node::NString(s) => {
                if source.len() == 0 {
                    self.children
                        .insert(0, Node::NSourceNode(SourceNode::new(s,
                                                                     original_source,
                                                                     source,
                                                                     1)));
                }
                // TODO: branch for last child node with preprendGeneratedCode
                // else if !self.children.is_empty() {}
                else {
                    self.children
                        .insert(0, Node::NCodeNode(CodeNode::new(s)));
                }
            }
            Node::NCodeNode(cn) => {
                self.children.insert(0, Node::NCodeNode(cn))
            }
            Node::NSourceNode(sn) => {
                self.children.insert(0, Node::NSourceNode(sn))
            }
            Node::NSingleLineNode(sln) => {
                self.children.insert(0, Node::NSingleLineNode(sln))
            }
            Node::NSourceListMap(slm) => {
                let mut new_childern = Vec::<Node>::new();
                for child in slm.children {
                    new_childern.push(child);
                }
                // HACK: use borrow instead of clone
                for child in self.children.clone() {
                    new_childern.push(child);
                }
                self.children = new_childern;
            }
        }
        self
    }

    pub fn map_generated_code(&mut self, f: &Fn(String) -> String) -> SourceListMap {
        let mut normalized_nodes: Vec<Node> = Vec::new();
        let children = self.children.clone();

        for child in children {
            match child {
                Node::NCodeNode(cn) => {
                    for n in cn.get_normalized_nodes() {
                        normalized_nodes.push(Node::NCodeNode(n));
                    }
                }
                Node::NSourceNode(sn) => {
                    for n in sn.get_normalized_nodes() {
                        normalized_nodes.push(Node::NSingleLineNode(n));
                    }
                }
                Node::NSingleLineNode(sln) => {
                    for n in sln.get_normalized_nodes() {
                        normalized_nodes.push(Node::NSingleLineNode(n));
                    }
                }
                _ => {}
            }
        }

        let mut optimized_nodes: Vec<Node> = Vec::new();
        for nodes in normalized_nodes {
            let sln = match nodes {
                Node::NCodeNode(n) => Some(Node::NCodeNode(n.map_generated_code(f))),
                Node::NSourceNode(n) => Some(Node::NSourceNode(n.map_generated_code(f).unwrap())),
                Node::NSingleLineNode(n) => Some(Node::NSingleLineNode(n.map_generated_code(f))),
                _ => None,
            };

            if optimized_nodes.is_empty() {
                if let Some(n) = sln {
                    optimized_nodes.push(n);
                }
            } else {
                // TODO: reduce clones
                let last = optimized_nodes.last().unwrap().clone();
                let merge_node: Option<Node> = match last {
                    Node::NCodeNode(ln) => {
                        match sln.clone() {
                            Some(n) => ln.merge(n),
                            _ => None,
                        }
                    }
                    Node::NSourceNode(ln) => {
                        match sln.clone() {
                            Some(n) => ln.merge(n),
                            _ => None,
                        }
                    }
                    Node::NSingleLineNode(ln) => {
                        match sln.clone() {
                            Some(n) => ln.merge(n),
                            _ => None,
                        }
                    }
                    _ => None
                };

                match merge_node {
                    Some(n) => {
                        optimized_nodes.pop();
                        optimized_nodes.push(n);
                    }
                    _ => {
                        if let Some(n) = sln {
                            optimized_nodes.push(n);
                        }
                    }
                }
            }
        }
        SourceListMap::new(Some(GenCode::CodeVec(optimized_nodes)), "", "")
    }

    pub fn to_string(&self) -> String {
        let mut output = String::from("");
        let children = self.children.clone();
        for child in children {
            match child {
                Node::NSingleLineNode(sln) => output += sln.get_generated_code(),
                _ => {}
            };
        }
        output
    }

    pub fn to_string_with_source_map(&mut self, options_file: Option<String>) -> StringWithSrcMap {
        let mut mc: MappingsContext = MappingsContext::new();

        let mut src: String = String::from("");
        for child in &self.children {
            match child {
                &Node::NCodeNode(ref sln) => src += sln.get_generated_code(),
                &Node::NSourceNode(ref sln) => src += sln.get_generated_code(),
                &Node::NSingleLineNode(ref sln) => src += sln.get_generated_code(),
                &Node::NString(ref sln) => src += &sln,
                _ => {}
            }
        }

        let mut mappings: String = String::from("");
        // HACK: use borrow instead of clone
        for mut child in self.children.clone() {
            match child {
                Node::NSourceNode(ref mut sln) => mappings += &sln.get_mappings(&mut mc),
                Node::NCodeNode(ref mut sln) => mappings += &sln.get_mappings(&mut mc),
                Node::NSingleLineNode(ref mut sln) => mappings += &sln.get_mappings(&mut mc),
                _ => {}
            };
        }

        let file = match options_file {
            Some(s) => s,
            None => String::from(""),
        };
        let arrays = mc.get_arrays();
        StringWithSrcMap {
            source: src,
            map: SrcMap {
                version: String::from("3"),
                file: file,
                sources: arrays.sources,
                sources_content: if mc.has_source_content {
                    let mut vec = Vec::<String>::new();
                    for sc in arrays.sources_content {
                        if let Node::NString(s) = sc {
                            vec.push(s);
                        }
                    }
                    vec
                } else {
                    vec![]
                },
                mappings: mappings,
            }
        }
    }
}

pub enum GenCode {
    Code(Node),
    CodeVec(Vec<Node>),
}

#[derive(Debug)]
pub struct StringWithSrcMap {
    pub source: String,
    pub map: SrcMap,
}

#[derive(Debug)]
pub struct SrcMap {
    pub version: String,
    pub file: String,
    pub sources: Vec<String>,
    pub sources_content: Vec<String>,
    pub mappings: String,
}

#[cfg(debug_assertions)]
impl PartialEq for StringWithSrcMap {
    fn eq(&self, other: &StringWithSrcMap) -> bool {
        self.source == other.source &&
        self.map == other.map
    }
}

#[cfg(debug_assertions)]
impl PartialEq for SrcMap {
    fn eq(&self, other: &SrcMap) -> bool {
        self.version == other.version &&
        self.file == other.file &&
        self.mappings == other.mappings &&
        self.sources.len() == other.sources.len() &&
        {
            let mut hm1: HashMap<&String, &String> = HashMap::new();
            let mut hm2: HashMap<&String, &String> = HashMap::new();

            for i in 0..self.sources.len() {
                hm1.insert(&self.sources[i], &self.sources_content[i]);
                hm2.insert(&other.sources[i], &other.sources_content[i]);
            }
            hm1 == hm2
        }
    }
}
