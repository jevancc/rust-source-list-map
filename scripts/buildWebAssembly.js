var path = require("path");
var utils = require("./utils");

var ROOT = path.join(__dirname, "../");
var BUILD_DIR = path.join(ROOT, "wasm-release/build");
var CWD = path.join(ROOT, "wasm-api");
var CRATE_NAME = "source_list_map_wasm_api";

function main() {
    utils.run(["cargo", "build", "--target", "wasm32-unknown-unknown"],
        { cwd: CWD });
    utils.run(["wasm-bindgen",
        "target/wasm32-unknown-unknown/debug/" + CRATE_NAME + ".wasm",
        "--out-dir", BUILD_DIR,
        "--nodejs"],
        { cwd: CWD });
}

main();
