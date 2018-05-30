"use strict";

var wasm = require("./build/source_list_map_wasm_api");
var SourceNode = require("./SourceNode");
var CodeNode = require("./CodeNode");
var SingleLineNode = require("./SingleLineNode");
var SourceListMap = require("./SourceListMap");

exports.StringVec = function StringVec(strs) {
    var stringVec = wasm.StringVec.new();
    for (var i = 0; i < strs.length; i++) {
        if (typeof strs[i] === "string")
            stringVec.push_String(strs[i]);
    }
    return stringVec;
}

exports.NodeVec = function NodeVec(nodes) {
    var nodeVec = wasm.NodeVec.new();
    for (var i = 0; i < nodes.length; i++) {
        if (typeof nodes[i] === "string")
            nodeVec.push_String(nodes[i]);
        else if (nodes[i] instanceof CodeNode)
            nodeVec.push_CodeNode(nodes[i]);
        else if (nodes[i] instanceof SourceNode)
            nodeVec.push_SourceNode(nodes[i]);
        else if (nodes[i] instanceof SingleLineNode)
            nodeVec.push_SingleLineNode(nodes[i]);
        else if (nodes[i] instanceof SourceListMap)
            nodeVec.push_SourceListMap(nodes[i]);
        else
            throw("Invalid node type");
    }
    return nodeVec;
}
