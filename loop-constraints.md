# Loop Constraints — mao_agent

Binding rules for autonomous agents. Read with `gate.yaml` and `loop-budget.md`.

## Testing redlines

- Never delete, skip, or `#[ignore]` tests to make CI green.
- Never weaken assertions (Recall gates, citation reject suite, hook/gate self-checks).
- Prefer `cargo test --no-default-features`. Tests use `DeterministicEmbedder`; do not require live keys.

## Artifact invariants (indexes are read-only to the loop)

- Never hand-edit `data/*.bin`, `data/tantivy_index/**`, `*.embedcache`, or `.fastembed_cache/**`.
- Never write `corpus/**/raw/**` (993 source PDFs/images). Ingest already skips `raw/` directories.
- Never commit `config.toml`, `.env*`, or API keys. Placeholders live in `config.example.toml`.
- Regenerate indexes only with `cargo run -- ingest` (human-approved), not by patching snapshots.

## Attempt budget

- Max **3** automated fix attempts per item; then escalate to a human (gate checker exit 2).
- L1 is **report-only**: it may atomically update `STATE.md` and must not rewrite application source.
- If `MAO_LOOP_PAUSE=1`, stop immediately.

## Paths

Mechanical enforcement: `python scripts/gate_check.py check --action tool --paths <files>`.
PreToolUse: `python scripts/hook_guard.py pre-tool` loads the same `gate.yaml`.
