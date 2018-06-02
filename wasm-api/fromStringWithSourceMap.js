"use strict";

var wasm = require("./build/source_list_map_wasm_api");
var SourceListMap = require("./SourceListMap");
var StringVec = require("./utils").StringVec;

module.exports = function fromStringWithSourceMap(code, map) {
    var sources = StringVec(map.sources || []);
    var sourcesContent = StringVec(map.sourcesContent || []);
    var mappings = map.mappings;

    var slp = new SourceListMap(-1);
    slp.ptr = wasm._from_string_with_source_map(
        code,
        sources,
        sourcesContent,
        mappings
    ).ptr;

    return slp;
};
