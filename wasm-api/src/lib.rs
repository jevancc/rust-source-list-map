#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;
extern crate source_list_map;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use wasm_bindgen::prelude::*;
use source_list_map::*;

#[wasm_bindgen]
pub struct _CodeNode {
    value: CodeNode,
}

#[wasm_bindgen]
impl _CodeNode {
    pub fn _new_String(generated_code: String) -> _CodeNode {
        _CodeNode {
            value: CodeNode::new(generated_code),
        }
    }

    pub fn _clone(&self) -> _CodeNode {
		_CodeNode {
            value: self.value.clone(),
        }
	}
}

#[wasm_bindgen]
pub struct _SourceNode {
    value: SourceNode,
}

#[wasm_bindgen]
impl _SourceNode {
    pub fn _new_String_Null_Null_Number(generated_code: String, starting_line: u32) -> _SourceNode {
        _SourceNode {
            value: SourceNode::new(generated_code, None, None, starting_line as usize),
        }
    }

    pub fn _new_String_String_String_Number(generated_code: String,
                                            source: String,
                                            original_source: String,
                                            starting_line: u32)
                                            -> _SourceNode {
        _SourceNode {
            value: SourceNode::new(generated_code,
                                   Some(source),
                                   Some(original_source),
                                   starting_line as usize),
        }
    }

    pub fn _clone(&self) -> _SourceNode {
        _SourceNode {
            value: self.value.clone()
        }
    }
}

#[wasm_bindgen]
pub struct _SingleLineNode {
    value: SingleLineNode,
}

#[wasm_bindgen]
impl _SingleLineNode {
    pub fn _new_String_Null_Null_Number(generated_code: String, starting_line: u32) -> _SingleLineNode {
        _SingleLineNode {
            value: SingleLineNode::new(generated_code, None, None, starting_line as usize),
        }
    }

    pub fn _new_String_String_String_Number(generated_code: String,
                                           source: String,
                                           original_source: String,
                                           starting_line: u32)
                                           -> _SingleLineNode {
        _SingleLineNode {
            value: SingleLineNode::new(generated_code,
                                       Some(source),
                                       Some(original_source),
                                       starting_line as usize),
        }
    }

    pub fn _clone(&self) -> _SingleLineNode {
        _SingleLineNode {
            value: self.value.clone()
        }
    }
}

#[wasm_bindgen]
pub struct _SourceListMap {
    value: SourceListMap,
}

#[wasm_bindgen]
impl _SourceListMap {
    pub fn _new() -> _SourceListMap {
        _SourceListMap {
            value: SourceListMap::new(None, None, None),
        }
    }

    // TODO: reduce clone
    pub fn _new_Nodes(nv: NodeVec) -> _SourceListMap {
        _SourceListMap {
            value: SourceListMap::new(Some(GenCode::CodeVec(nv.value.clone())), None, None),
        }
    }

    pub fn _add_Node(&mut self, nv: NodeVec) {
        self.value.add(nv.value[0].clone(), None, None);
    }

    pub fn _add_Node_String_String(&mut self, nv: NodeVec, source: String, original_source: String) {
        self.value.add(nv.value[0].clone(),
                       Some(source),
                       Some(original_source));
    }

    pub fn _prepend_Node(&mut self, nv: NodeVec) {
        self.value.prepend(nv.value[0].clone(), None, None);
    }

    pub fn _prepend_Node_String_String(&mut self, nv: NodeVec, source: String, original_source: String) {
        self.value.prepend(nv.value[0].clone(),
                       Some(source),
                       Some(original_source));
    }

    pub fn _to_string(&self) -> String {
        self.value.to_string()
    }

    pub fn _to_string_with_source_map(&mut self) -> String {
        let obj = self.value.to_string_with_source_map(None);
        serde_json::to_string(&JsStringWithSrcMap {
            source: obj.source,
            map: JsSrcMap::from_srcmap(&obj.map),
        }).unwrap()
    }

    pub fn _to_string_with_source_map_String(&mut self, options_file: String) -> String {
        let obj = self.value.to_string_with_source_map(Some(options_file));
        serde_json::to_string(&JsStringWithSrcMap {
            source: obj.source,
            map: JsSrcMap::from_srcmap(&obj.map),
        }).unwrap()
    }

    pub fn _map_generated_code(&self, fn_name: String) -> _SourceListMap {
        _SourceListMap {
            value: self.value.map_generated_code(&fn_name),
        }
    }
}

#[wasm_bindgen]
pub fn _from_string_with_source_map(code: &str,
                                    sources: StringVec,
                                    sources_content: StringVec,
                                    mappings: &str)
                                    -> _SourceListMap {
    let sources = sources.value;
    let sources_content = sources_content.value;

    _SourceListMap {
        value: from_string_with_source_map(
            code,
            sources.iter().map(|s| s.as_str()).collect(),
            sources_content.iter().map(|s| s.as_str()).collect(),
            mappings)
    }
}

#[wasm_bindgen]
pub struct StringVec {
    value: Vec<String>
}

#[wasm_bindgen]
impl StringVec {
    pub fn new() -> StringVec {
        StringVec {
            value: Vec::new()
        }
    }

    pub fn push_String(&mut self, s: String) {
        self.value.push(s);
    }
}

#[wasm_bindgen]
pub struct NodeVec {
    value: Vec<Node>,
}

#[wasm_bindgen]
impl NodeVec {
    pub fn new() -> NodeVec {
        NodeVec {
            value: Vec::new(),
        }
    }

    pub fn push_String(&mut self, s: String) {
        self.value.push(Node::NString(s));
    }

    pub fn push_SourceNode(&mut self, sn: &_SourceNode) {
        self.value.push(Node::NSourceNode(sn.value.clone()));
    }

    pub fn push_CodeNode(&mut self, cn: &_CodeNode) {
        self.value.push(Node::NCodeNode(cn.value.clone()));
    }

    pub fn push_SingleLineNode(&mut self, sln: &_SingleLineNode) {
        self.value.push(Node::NSingleLineNode(sln.value.clone()));
    }

    pub fn push_SourceListMap(&mut self, slp: &_SourceListMap) {
        self.value.push(Node::NSourceListMap(slp.value.clone()));
    }
}

#[derive(Serialize)]
struct JsStringWithSrcMap {
    pub source: String,
    pub map: JsSrcMap,
}

#[derive(Serialize)]
struct JsSrcMap {
    pub version: i32,
    pub file: String,
    pub sources: Option<Vec<Option<String>>>,
    pub sourcesContent: Option<Vec<String>>,
    pub mappings: String,
}

impl JsSrcMap {
    // TODO: Reduce clones
    pub fn from_srcmap(srcmap: &SrcMap) -> JsSrcMap {
        JsSrcMap {
            version: srcmap.version,
            file: srcmap.file.clone(),
            sources: if srcmap.sources.is_empty() {
                Some(vec![None])
            } else {
                Some(srcmap.sources.clone().into_iter().map(|s| Some(s)).collect())
            },
            sourcesContent: if srcmap.sources_content.is_empty() {
                None
            } else {
                Some(srcmap.sources_content.clone())
            },
            mappings: srcmap.mappings.clone(),
        }
    }
}
