#!/bin/bash
set -euo pipefail

#curl -sSLo lib/antlr4.jar https://github.com/rrevenantt/antlr4rust/releases/download/antlr4-4.8-2-Rust-0.2/antlr4-4.8-2-SNAPSHOT-complete.jar
java -jar lib/antlr4.jar -Dlanguage=Rust src/grammar/bsize.g4
