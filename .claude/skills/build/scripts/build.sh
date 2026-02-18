#!/bin/bash
set -e

echo "[build] Running anchor build..." >&2
anchor build

echo "[build] Running cargo build..." >&2
cargo build

echo "[build] Build complete." >&2
