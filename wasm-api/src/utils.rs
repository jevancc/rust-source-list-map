use serde_json::value::Value;
use source_list_map::*;

pub fn string_with_srcmap_to_json(obj: &StringWithSrcMap) -> Value {
    let mut map = json!({
        "version": obj.map.version,
        "file": obj.map.file,
        "sources": if obj.map.sources.is_empty() {
            json!([null])
        } else {
            json!(obj.map.sources)
        },
        "mappings": obj.map.mappings
    });

    if !obj.map.sources_content.is_empty() {
        if let Value::Object(ref mut m) = map {
            m.insert(
                String::from("sourcesContent"),
                json!(obj.map.sources_content),
            );
        }
    }

    json!({
        "source": obj.source,
        "map": map,
    })
}
