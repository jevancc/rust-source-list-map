extern crate source_list_map;

use source_list_map::*;

pub fn g_Str(s: &str) -> String {
    String::from(s)
}

pub fn g_NStr(s: &str) -> Node {
    Node::NString(String::from(s))
}
