var fs = require("fs");
var path = require("path");
var utils = require("./utils");

var ROOT = path.join(__dirname, "../");
var BUILD_DIR = path.join(ROOT, "wasm-api/build");
var CWD = path.join(ROOT, "wasm-api");
var CRATE_NAME = "source_list_map_wasm_api";

function main() {
    if (!fs.existsSync(BUILD_DIR)) {
        fs.mkdirSync(BUILD_DIR);
    }

    var options = {
        cwd: CWD,
        stdio: "inherit"
    };

    utils.run(
        ["cargo", "build", "--target", "wasm32-unknown-unknown", "--release"],
        options
    );
    utils.run(
        [
            "wasm-bindgen",
            "target/wasm32-unknown-unknown/release/" + CRATE_NAME + ".wasm",
            "--out-dir",
            BUILD_DIR,
            "--nodejs",
            "--no-typescript"
        ],
        options
    );
}

main();
