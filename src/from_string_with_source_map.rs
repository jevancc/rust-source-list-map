use code_node::CodeNode;
use source_list_map::GenCode;
use source_list_map::SourceListMap;
use source_node::SourceNode;
use std::rc::Rc;
use vlq;
use Node;
use StringPtr;

pub fn from_string_with_source_map(
    code: &str,
    sources: &Vec<&str>,
    sources_content: &Vec<&str>,
    mappings: &str,
) -> SourceListMap {
    let mappings = mappings.split(';').enumerate();
    let mut lines = code.split('\n').enumerate();
    let lines_count = lines.clone().count();
    let mut nodes: Vec<Node> = vec![];

    let mut current_line: i64 = 1;
    let mut current_source_index: usize = 0;
    let mut current_source_node_line: usize = 0;

    for (i, mapping) in mappings {
        if let Some((_, line)) = lines.next() {
            let line = if i != lines_count - 1 {
                String::from(line) + "\n"
            } else {
                String::from(line)
            };
            if !mapping.is_empty() {
                let mut line_added: bool = false;
                let mut rest = mapping.as_bytes().iter().cloned().peekable();

                while let Some(_) = rest.peek() {
                    line_added = {
                        if let Some(c) = rest.clone().peek() {
                            if *c != b',' {
                                vlq::decode(&mut rest).unwrap();
                            }
                        }

                        match rest.clone().peek() {
                            None => false,
                            Some(c) => {
                                if *c == b',' {
                                    rest.next();
                                    false
                                } else {
                                    let value = vlq::decode(&mut rest).unwrap();
                                    let source_index = value as usize + current_source_index;
                                    current_source_index = source_index;

                                    let mut line_position: i64;
                                    if let Some(c) = rest.clone().peek() {
                                        if *c != b',' {
                                            let value = vlq::decode(&mut rest).unwrap();
                                            line_position = value + current_line as i64;
                                            current_line = line_position;
                                        } else {
                                            line_position = current_line;
                                        }
                                    } else {
                                        line_position = current_line;
                                    }

                                    while let Some(c) = rest.clone().peek() {
                                        if *c != b',' {
                                            rest.next();
                                        } else {
                                            break;
                                        }
                                    }

                                    if !line_added {
                                        add_source(
                                            &mut nodes,
                                            &mut current_source_node_line,
                                            line.clone(),
                                            sources.get(source_index),
                                            sources_content.get(source_index),
                                            line_position as usize,
                                        );
                                        true
                                    } else {
                                        false
                                    }
                                }
                            }
                        }
                    } || line_added;
                }
                if !line_added {
                    add_code(&mut nodes, &mut current_source_node_line, line);
                }
            } else {
                add_code(&mut nodes, &mut current_source_node_line, line);
            }
        }
    }

    let mut last = String::new();
    while let Some((i, line)) = lines.next() {
        if i < lines_count - 1 && line.trim().is_empty() {
            let line = String::from(line) + "\n";
            add_code(&mut nodes, &mut current_source_node_line, line);
        } else {
            last += line;
            while let Some((_, line)) = lines.next() {
                last += "\n";
                last += line;
            }
            add_code(&mut nodes, &mut current_source_node_line, last);
            break;
        }
    }
    SourceListMap::new(Some(GenCode::CodeVec(nodes)), None, None)
}

fn add_code(nodes: &mut Vec<Node>, current_source_node_line: &mut usize, generated_code: String) {
    match nodes.last_mut() {
        Some(Node::NCodeNode(ref mut n)) => {
            n.add_generated_code(&generated_code);
            return;
        }
        Some(Node::NSourceNode(ref mut n)) => {
            if generated_code.trim().is_empty() {
                n.add_generated_code(&generated_code);
                *current_source_node_line += 1;
                return;
            }
        }
        _ => {}
    }
    nodes.push(Node::NCodeNode(CodeNode::new(generated_code)));
}

fn add_source(
    nodes: &mut Vec<Node>,
    current_source_node_line: &mut usize,
    generated_code: String,
    source: Option<&&str>,
    original_source: Option<&&str>,
    line: usize,
) {
    let source = source.map(|s| String::from(*s));
    let original_source = original_source.map(|s| String::from(*s));

    if let Some(Node::NSourceNode(ref mut n)) = nodes.last_mut() {
        if ((n.source.is_none() && source.is_none()) || {
            let ns = Rc::into_raw(n.source.clone().unwrap());
            let s = source.clone().unwrap();
            unsafe { *ns == s }
        }) && *current_source_node_line == line
        {
            n.add_generated_code(&generated_code);
            *current_source_node_line += 1;
            return;
        }
    }
    nodes.push(Node::NSourceNode(SourceNode::new(
        generated_code,
        source.map(|s| StringPtr::Str(s)),
        original_source.map(|s| StringPtr::Str(s)),
        line,
    )));
    *current_source_node_line = line + 1;
}
