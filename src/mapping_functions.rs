pub fn mapping_function(fn_name: &str) -> &Fn(String) -> String {
    match fn_name {
        "map_generated_code_test" => &map_generated_code_test,
        _ => &identical,
    }
}

fn map_generated_code_test(line: String) -> String {
    line.replace(";", "\n")
        .replace("\\\n", " ")
        .replace("$\n", "")
}

fn identical(line: String) -> String {
    line
}
