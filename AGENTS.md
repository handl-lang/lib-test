# lib-test

Test framework library — assert/property/fuzz/snapshot — Handl stdlib
port. Pure `.hnl` package (the Rust const-shim crate was dropped in
`2b70515`).

## Layout

| path | role |
| --- | --- |
| `src/test.hnl` | test runner surface |
| `src/assert.hnl`, `property.hnl`, `fuzz.hnl`, `snapshot.hnl`, `benchmark.hnl` | the framework modules |
| `src/docs/*.yml` | per-module package docs |
| `pkg.hnl`, `scripts/hnl-gate.sh`, `workflows/gate.yaml` | manifest / hnl gate / bupha entry |

## Contract

- Pure `.hnl` package; no Rust crate (ADR-0160 package shape).
- `lib-mock`'s `assert_*` helpers return this package's ok/err Result
  shape.

## Verification

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
```

## License

MIT.

<!-- myco:agent-awareness -->

## Agent awareness

- Session start / any work discussion: run `myco status --prompt`
  before proposing next steps — know the board first.
- User describes new work → offer to capture it (`myco todo add`,
  `issue new`, `proposal new`). Don't wait to be asked.
- Picking up a task: `myco resume <ID>` is the briefing.
- Wrapping up: `myco handoff <ID>` anything resumable; keep statuses
  honest (claimed → in-progress → blocked/done).

<!-- /myco:agent-awareness -->
