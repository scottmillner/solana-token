#!/bin/bash
set -e

echo "[unit-tests] Running library tests..." >&2
cargo test --lib

echo "[unit-tests] Running generated code tests..." >&2
cargo test --test generated_code_tests

echo "[unit-tests] All unit tests passed." >&2
