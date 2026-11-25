# lib-test

Test framework library — assert/property/fuzz/snapshot — Handl stdlib
port. Pure `.hnl` plus a const-only Rust crate re-exporting the
sources.

## Layout

| path | role |
| --- | --- |
| `src/test.hnl` | test runner surface |
| `src/assert.hnl`, `property.hnl`, `fuzz.hnl`, `snapshot.hnl`, `benchmark.hnl` | the framework modules |
| `src/docs/*.yml` | per-module package docs |
| `src/lib.rs` | const-only crate re-exporting the .hnl sources |
| `Cargo.toml`, `xtask/` | const-only crate manifest + `cargo xtask` gate |
| `pkg.hnl`, `scripts/hnl-gate.sh`, `workflows/gate.yaml` | manifest / hnl gate / bupha entry |

## Contract

- Pure `.hnl`; the Rust crate only re-exports sources as consts.
- `lib-mock`'s `assert_*` helpers return this package's ok/err Result
  shape.

## Verification

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
cargo xtask gate        # the const-only crate side
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
