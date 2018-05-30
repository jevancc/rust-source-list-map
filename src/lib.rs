#[macro_use]
extern crate serde_derive;

mod source_node;
mod code_node;
mod single_line_node;
mod mappings_context;
mod source_list_map;
mod from_string_with_source_map;
mod mapping_functions;
mod helpers;

pub use code_node::CodeNode;
pub use source_node::SourceNode;
pub use single_line_node::SingleLineNode;
pub use mappings_context::MappingsContext;
pub use source_list_map::SourceListMap;
pub use source_list_map::SrcMap;
pub use source_list_map::GenCode;
pub use from_string_with_source_map::from_string_with_source_map;

#[derive(Clone, Debug)]
pub enum Node {
    NString(String),
    NCodeNode(CodeNode),
    NSourceNode(SourceNode),
    NSingleLineNode(SingleLineNode),
    NSourceListMap(SourceListMap),
}
