use source_list_map::*;

pub struct TestMappingFunction;
impl MappingFunction for TestMappingFunction {
    fn map(&mut self, line: String) -> String {
        line.replace(";", "\n")
            .replace("\\\n", " ")
            .replace("$\n", "")
    }
}

pub struct IdenticalFunction;
impl MappingFunction for IdenticalFunction {
    fn map(&mut self, line: String) -> String {
        line
    }
}
