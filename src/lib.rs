#[macro_use]
extern crate serde_derive;

extern crate linked_hash_map;
extern crate vlq;

mod code_node;
mod from_string_with_source_map;
mod helpers;
mod mapping_function;
mod mappings_context;
mod single_line_node;
mod source_list_map;
mod source_node;

pub use code_node::CodeNode;
pub use from_string_with_source_map::from_string_with_source_map;
pub use mapping_function::MappingFunction;
pub use mappings_context::MappingsContext;
pub use single_line_node::SingleLineNode;
pub use source_list_map::GenCode;
pub use source_list_map::SourceListMap;
pub use source_list_map::SrcMap;
pub use source_list_map::StringWithSrcMap;
pub use source_node::SourceNode;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum Node {
    NRcString(Rc<String>),
    NString(String),
    NCodeNode(CodeNode),
    NSourceNode(SourceNode),
    NSingleLineNode(SingleLineNode),
    NSourceListMap(SourceListMap),
}

pub enum StringPtr {
    Str(String),
    Ptr(Rc<String>),
}

impl StringPtr {
    pub fn to_ptr(self) -> Rc<String> {
        match self {
            StringPtr::Str(s) => Rc::new(s),
            StringPtr::Ptr(p) => p,
        }
    }
}
