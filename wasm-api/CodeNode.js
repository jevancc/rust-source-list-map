"use strict";

var wasm = require("./build/source_list_map_wasm_api");

class CodeNode extends wasm._CodeNode {
    constructor(generatedCode) {
        super(0);
        if (generatedCode) {
            this.ptr = CodeNode._new_string(generatedCode).ptr;
        }
    }

    clone() {
        var ret = new CodeNode();
        ret.ptr = wasm._codenode__clone(this.ptr);
        return ret;
    }
}

module.exports = CodeNode;
