extern crate source_list_map;

mod utils;

#[cfg(test)]
mod mapping_generation {
    use source_list_map::*;
    use utils::*;

    #[test]
    fn should_generate_mappings() {
        let mut map = SourceListMap::new(None, None, None);
        map.add(g_NStr("Gen\nCode "), None, None);
		map.add(g_NStr("Source\nCode\n"), Some(g_Str("file.txt")), Some(g_Str("Source\nCode\n")));
		map.add(g_NStr("Gen "), None, None);
		map.add(g_NStr("Code "), None, None);
		map.add(g_NStr("Source\nCode"), Some(g_Str("file.txt")), Some(g_Str("Source\nCode\n")));

		let result = map.to_string_with_source_map(Some(String::from("test.txt")));
        assert_eq!(result.source, "Gen\nCode Source\nCode\nGen Code Source\nCode");
        assert_eq!(result.map.sources_content[0], "Source\nCode\n");
    }

    #[test]
    fn should_the_same_mappings_for_single_line_and_normal_node() {
        let mut map1 = SourceListMap::new(None, None, None);
        let mut map2 = SourceListMap::new(None, None, None);

		map1.add(
                Node::NSingleLineNode(SingleLineNode::new(g_Str("abc"),
                                                          Some(g_Str("abc")),
                                                          Some(g_Str("source")),
                                                          10)),
                None,
                None)
            .add(g_NStr("\n\n"), None, None)
            .add(g_NStr("Source Code\n"), Some(g_Str("file.txt")), Some(g_Str("Source\nCode\n")));

        map2.add(
                Node::NSourceNode(SourceNode::new(g_Str("abc"), Some(g_Str("abc")), Some(g_Str("source")), 10)),
                None,
                None)
            .add(g_NStr("\n\n"), None, None)
            .add(g_NStr("Source Code\n"), Some(g_Str("file.txt")), Some(g_Str("Source\nCode\n")));

		let result1 = map1.to_string_with_source_map(Some(g_Str("test.txt")));
		let result2 = map2.to_string_with_source_map(Some(g_Str("test.txt")));
        assert_eq!(result1, result2);
    }
}
