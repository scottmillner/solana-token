#!/bin/bash
set -e

IDL="target/idl/solana_token.json"
SO="target/deploy/solana_token.so"

if [ ! -f "$IDL" ] || [ ! -f "$SO" ]; then
  echo "[integration-tests] ERROR: IDL or .so file missing. Run /build first." >&2
  exit 1
fi

echo "[integration-tests] Running integration tests..." >&2
cargo test --package solana-token-cli --test integration -- --nocapture

echo "[integration-tests] All integration tests passed." >&2
