#[macro_use]
extern crate serde_derive;

extern crate linked_hash_map;
extern crate vlq;

mod code_node;
mod from_string_with_source_map;
mod helpers;
mod mapping_functions;
mod mappings_context;
mod single_line_node;
mod source_list_map;
mod source_node;

pub use code_node::CodeNode;
pub use from_string_with_source_map::from_string_with_source_map;
pub use mappings_context::MappingsContext;
pub use single_line_node::SingleLineNode;
pub use source_list_map::GenCode;
pub use source_list_map::SourceListMap;
pub use source_list_map::SrcMap;
pub use source_list_map::StringWithSrcMap;
pub use source_node::SourceNode;

#[derive(Clone, Debug)]
pub enum Node {
    NString(String),
    NCodeNode(CodeNode),
    NSourceNode(SourceNode),
    NSingleLineNode(SingleLineNode),
    NSourceListMap(SourceListMap),
}
