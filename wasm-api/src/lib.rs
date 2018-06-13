#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate serde;
extern crate source_list_map;
extern crate wasm_bindgen;

#[macro_use]
extern crate serde_json;

mod mapping_functions;
// mod utils;

use mapping_functions::*;
use source_list_map::*;
// use utils::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct _CodeNode {
    val: CodeNode,
}

#[wasm_bindgen]
impl _CodeNode {
    pub fn _new_string(generated_code: String) -> _CodeNode {
        _CodeNode {
            val: CodeNode::new(generated_code),
        }
    }

    pub fn _clone(&self) -> _CodeNode {
        _CodeNode {
            val: self.val.clone(),
        }
    }
}

#[wasm_bindgen]
pub struct _SourceNode {
    val: SourceNode,
}

#[wasm_bindgen]
impl _SourceNode {
    pub fn _new_string_null_null_number(generated_code: String, starting_line: u32) -> _SourceNode {
        _SourceNode {
            val: SourceNode::new(generated_code, None, None, starting_line as usize),
        }
    }

    pub fn _new_string_string_string_number(
        generated_code: String,
        source: String,
        original_source: String,
        starting_line: u32,
    ) -> _SourceNode {
        _SourceNode {
            val: SourceNode::new(
                generated_code,
                Some(StringPtr::Str(source)),
                Some(StringPtr::Str(original_source)),
                starting_line as usize,
            ),
        }
    }

    pub fn _clone(&self) -> _SourceNode {
        _SourceNode {
            val: self.val.clone(),
        }
    }
}

#[wasm_bindgen]
pub struct _SingleLineNode {
    val: SingleLineNode,
}

#[wasm_bindgen]
impl _SingleLineNode {
    pub fn _new_string_null_null_number(
        generated_code: String,
        starting_line: u32,
    ) -> _SingleLineNode {
        _SingleLineNode {
            val: SingleLineNode::new(generated_code, None, None, starting_line as usize),
        }
    }

    pub fn _new_string_string_string_number(
        generated_code: String,
        source: String,
        original_source: String,
        starting_line: u32,
    ) -> _SingleLineNode {
        _SingleLineNode {
            val: SingleLineNode::new(
                generated_code,
                Some(StringPtr::Str(source)),
                Some(StringPtr::Str(original_source)),
                starting_line as usize,
            ),
        }
    }

    pub fn _clone(&self) -> _SingleLineNode {
        _SingleLineNode {
            val: self.val.clone(),
        }
    }
}

#[wasm_bindgen]
pub struct _SourceListMap {
    val: SourceListMap,
}

#[wasm_bindgen]
impl _SourceListMap {
    pub fn _new() -> _SourceListMap {
        _SourceListMap {
            val: SourceListMap::new(None, None, None),
        }
    }

    // TODO: reduce clone
    pub fn _new_nodes(nv: NodeVec) -> _SourceListMap {
        _SourceListMap {
            val: SourceListMap::new(Some(GenCode::CodeVec(nv.val.clone())), None, None),
        }
    }

    pub fn _add_node(&mut self, nv: NodeVec) {
        self.val.add(nv.val[0].clone(), None, None);
    }

    pub fn _add_node_string_string(
        &mut self,
        nv: NodeVec,
        source: String,
        original_source: String,
    ) {
        self.val
            .add(nv.val[0].clone(), Some(StringPtr::Str(source)), Some(StringPtr::Str(original_source)));
    }

    pub fn _prepend_node(&mut self, nv: NodeVec) {
        self.val.prepend(nv.val[0].clone(), None, None);
    }

    pub fn _prepend_node_string_string(
        &mut self,
        nv: NodeVec,
        source: String,
        original_source: String,
    ) {
        self.val
            .prepend(nv.val[0].clone(), Some(StringPtr::Str(source)), Some(StringPtr::Str(original_source)));
    }

    pub fn _to_string(&self) -> String {
        self.val.to_string()
    }

    pub fn _to_string_with_source_map(&mut self) -> JsSrcMap {
        let srcmap = self.val.to_string_with_source_map(None);
        JsSrcMap {
            source: srcmap.source,
            map_sources_content: srcmap.map.sources_content,
            map_sources: srcmap.map.sources,
            mappings: srcmap.map.mappings,
        }
    }

    pub fn _to_string_with_source_map_json(&mut self) -> String {
        let srcmap = self.val.to_string_with_source_map(None);
        json!({
            "source": &srcmap.source,
            "mapSourcesContent": &srcmap.map.sources_content,
            "mapSources": &srcmap.map.sources,
            "mappings": &srcmap.map.mappings,
        }).to_string()
    }
}

#[wasm_bindgen]
pub fn _sourcelistmap_map_generated_code(slp: _SourceListMap, fn_name: &str) -> _SourceListMap {
    let mut test_mapping_function = TestMappingFunction {};
    let mut identical_function = IdenticalFunction {};

    let map = match fn_name {
        "map_generated_code_test" => slp.val.map_generated_code(&mut test_mapping_function),
        _ => slp.val.map_generated_code(&mut identical_function),
    };

    _SourceListMap { val: map }
}

#[wasm_bindgen]
pub fn _from_string_with_source_map(
    code: &str,
    sources: StringVec,
    sources_content: StringVec,
    mappings: &str,
) -> _SourceListMap {
    let sources = sources.val;
    let sources_content = sources_content.val;

    _SourceListMap {
        val: from_string_with_source_map(
            code,
            sources.iter().map(|s| s.as_str()).collect(),
            sources_content.iter().map(|s| s.as_str()).collect(),
            mappings,
        ),
    }
}

#[wasm_bindgen]
pub struct StringVec {
    val: Vec<String>,
}

#[wasm_bindgen]
impl StringVec {
    pub fn new() -> StringVec {
        StringVec { val: Vec::new() }
    }

    pub fn push_string(&mut self, s: String) {
        self.val.push(s);
    }
}

#[wasm_bindgen]
pub struct NodeVec {
    val: Vec<Node>,
}

#[wasm_bindgen]
impl NodeVec {
    pub fn new() -> NodeVec {
        NodeVec { val: Vec::new() }
    }

    pub fn push_string(&mut self, s: String) {
        self.val.push(Node::NString(s));
    }

    pub fn push_sourcenode(&mut self, sn: &_SourceNode) {
        self.val.push(Node::NSourceNode(sn.val.clone()));
    }

    pub fn push_codenode(&mut self, cn: &_CodeNode) {
        self.val.push(Node::NCodeNode(cn.val.clone()));
    }

    pub fn push_singlelinenode(&mut self, sln: &_SingleLineNode) {
        self.val.push(Node::NSingleLineNode(sln.val.clone()));
    }

    pub fn push_sourcelistmap(&mut self, slp: &_SourceListMap) {
        self.val.push(Node::NSourceListMap(slp.val.clone()));
    }
}

#[wasm_bindgen]
pub struct JsSrcMap {
    source: String,
    map_sources: Vec<String>,
    map_sources_content: Vec<String>,
    mappings: String,
}

#[wasm_bindgen]
impl JsSrcMap {
    pub fn get_source(&self) -> String {
        self.source.clone()
    }

    pub fn get_map_contents_len(&self) -> i32 {
        self.map_sources_content.len() as i32
    }

    pub fn get_map_sources_len(&self) -> i32 {
        self.map_sources.len() as i32
    }

    pub fn get_map_contents_nth(&self, idx: i32) -> String {
        self.map_sources_content.get(idx as usize).unwrap().clone()
    }

    pub fn get_map_sources_nth(&self, idx: i32) -> String {
        self.map_sources.get(idx as usize).unwrap().clone()
    }

    pub fn get_mappings(&self) -> String {
        self.mappings.clone()
    }
}
