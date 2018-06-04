extern crate source_list_map;

mod utils;

#[cfg(test)]
mod mapping_generation {
    use source_list_map::*;
    use utils::*;

    #[test]
    fn should_generate_mappings() {
        let mut map = SourceListMap::new(None, None, None);
        map.add(g_n_str("Gen\nCode "), None, None);
        map.add(
            g_n_str("Source\nCode\n"),
            Some(g_str("file.txt")),
            Some(g_str("Source\nCode\n")),
        );
        map.add(g_n_str("Gen "), None, None);
        map.add(g_n_str("Code "), None, None);
        map.add(
            g_n_str("Source\nCode"),
            Some(g_str("file.txt")),
            Some(g_str("Source\nCode\n")),
        );

        let result = map.to_string_with_source_map(Some(String::from("test.txt")));
        assert_eq!(
            result.source,
            "Gen\nCode Source\nCode\nGen Code Source\nCode"
        );
        assert_eq!(
            if let Some(map) = result.map {
                map.sources_content.get(0).unwrap().clone()
            } else {
                String::new()
            },
            "Source\nCode\n"
        );
    }

    #[test]
    fn should_the_same_mappings_for_single_line_and_normal_node() {
        let mut map1 = SourceListMap::new(None, None, None);
        let mut map2 = SourceListMap::new(None, None, None);

        map1.add(
            Node::NSingleLineNode(SingleLineNode::new(
                g_str("abc"),
                Some(g_str("abc")),
                Some(g_str("source")),
                10,
            )),
            None,
            None,
        ).add(g_n_str("\n\n"), None, None)
            .add(
                g_n_str("Source Code\n"),
                Some(g_str("file.txt")),
                Some(g_str("Source\nCode\n")),
            );

        map2.add(
            Node::NSourceNode(SourceNode::new(
                g_str("abc"),
                Some(g_str("abc")),
                Some(g_str("source")),
                10,
            )),
            None,
            None,
        ).add(g_n_str("\n\n"), None, None)
            .add(
                g_n_str("Source Code\n"),
                Some(g_str("file.txt")),
                Some(g_str("Source\nCode\n")),
            );

        let result1 = map1.to_string_with_source_map(Some(g_str("test.txt")));
        let result2 = map2.to_string_with_source_map(Some(g_str("test.txt")));
        assert_eq!(result1, result2);
    }
}
