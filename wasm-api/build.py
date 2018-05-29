#!/usr/bin/env python3
import argparse
import os
import re
import subprocess
import sys

CRATE_NAME = "source_list_map_wasm_api"
BUILD_DIR = "./build"

def run(cmd, **kwargs):
    sys.stderr.write("> " + " ".join(cmd) + "\n")

    # if "stdout" not in kwargs:
    #     kwargs["stdout"] = subprocess.PIPE
    child = subprocess.run(cmd, **kwargs)
    if child.returncode != 0:
        raise Exception("{} did not exit OK".format(str(cmd)))
    # return child.stdout.decode(encoding="utf-8", errors="ignore")

def main():
    run(["cargo", "build", "--target", "wasm32-unknown-unknown"])
    run(["wasm-bindgen",
        "target/wasm32-unknown-unknown/debug/{}.wasm".format(CRATE_NAME),
        "--out-dir", BUILD_DIR,
        "--nodejs"
        ])

if __name__ == "__main__":
    if os.path.dirname(__file__) != "":
        os.chdir(os.path.dirname(__file__))
    main()
