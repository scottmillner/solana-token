---
description: Run integration tests against a local test validator. Use when asked to "run integration tests" or "test end-to-end".
---

Run the integration test script:

```bash
bash .claude/skills/integration-tests/scripts/run-integration.sh
```

Requires `anchor build` to have been run first (needs IDL and .so file). If missing, run the `/build` skill first. Report pass/fail results to the user.
