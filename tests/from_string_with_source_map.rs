extern crate serde_json;
extern crate source_list_map;

#[macro_use]
extern crate serde_derive;

mod utils;

#[cfg(test)]
mod from_string_with_source_map {
    use serde_json;
    use source_list_map::*;
    use std::fs;
    use std::io;
    use std::io::prelude::*;
    use std::path::Path;

    const TESTS_PATH: &str = "./tests/fixtures/from-to-tests";

    #[test]
    fn should_parse_and_generate() {
        let items = get_test_items(Path::new(TESTS_PATH)).unwrap();
        for item in items.iter().cloned() {
            let map: JsSrcMap = serde_json::from_str(
                &read_file(Path::new(&(item.clone() + ".input.map"))).unwrap(),
            ).unwrap();
            let map = map.to_srcmap();
            let generated_code =
                read_file(Path::new(TESTS_PATH).join(map.file.clone()).as_path()).unwrap();
            let expected_map: JsSrcMap = serde_json::from_str(
                &read_file(Path::new(&(item.clone() + ".expected.map"))).unwrap(),
            ).unwrap();
            let expected_map = expected_map.to_srcmap();

            let mut slm = from_string_with_source_map(
                &generated_code,
                &map.sources.iter().map(|s| s.as_ref()).collect(),
                &map.sources_content.iter().map(|s| s.as_ref()).collect(),
                &map.mappings,
            );
            let result = slm.to_string_with_source_map(Some(map.file.clone()));
            assert_eq!(result.map, expected_map, "Failed on: {}", item);
            assert_eq!(result.source, generated_code, "Failed on: {}", item);

            let mut slm = from_string_with_source_map(
                &generated_code,
                &expected_map.sources.iter().map(|s| s.as_ref()).collect(),
                &expected_map
                    .sources_content
                    .iter()
                    .map(|s| s.as_ref())
                    .collect(),
                &expected_map.mappings,
            );
            let result = slm.to_string_with_source_map(Some(map.file.clone()));
            assert_eq!(result.map, expected_map, "Failed on: {}", item);
            assert_eq!(result.source, generated_code, "Failed on: {}", item);
        }
    }

    #[derive(Clone, Deserialize, Debug)]
    struct JsSrcMap {
        pub version: i32,
        pub file: String,
        pub sources: Option<Vec<Option<String>>>,
        pub sourcesContent: Option<Vec<String>>,
        pub mappings: String,
    }

    impl JsSrcMap {
        pub fn to_srcmap(self) -> SrcMap {
            SrcMap {
                version: self.version,
                file: self.file,
                sources: match self.sources {
                    Some(v) => v.into_iter().filter_map(|i| i).collect(),
                    None => vec![],
                },
                sources_content: match self.sourcesContent {
                    Some(v) => v,
                    None => vec![],
                },
                mappings: self.mappings,
            }
        }
    }

    fn get_test_items(path: &Path) -> io::Result<Vec<String>> {
        let mut test_items: Vec<String> = vec![];
        for entry in fs::read_dir(path)? {
            let file_path = entry?.path();
            let file_path = file_path.to_str().unwrap();
            if file_path.ends_with(".input.map") {
                test_items.push(String::from(file_path.trim_right_matches(".input.map")));
            }
        }
        Ok(test_items)
    }

    fn read_file(path: &Path) -> io::Result<String> {
        let file = fs::File::open(path)?;
        let mut buf_reader = io::BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;
        Ok(contents)
    }
}
