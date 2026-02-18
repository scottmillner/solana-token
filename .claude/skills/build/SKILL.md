---
description: Build the Solana token program and CLI. Use when asked to "build", "compile", or "rebuild".
---

Run the build script to compile the Anchor program and CLI:

```bash
bash .claude/skills/build/scripts/build.sh
```

The script runs `anchor build` first (generates IDL), then `cargo build` (generates CLI types from IDL). Both steps must succeed. Report the outcome to the user.
