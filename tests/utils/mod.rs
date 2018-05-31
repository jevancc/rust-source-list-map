extern crate source_list_map;

use source_list_map::*;

pub fn g_str(s: &str) -> String {
    String::from(s)
}

pub fn g_n_str(s: &str) -> Node {
    Node::NString(String::from(s))
}
