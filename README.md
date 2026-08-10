# Leviathan

Leviathan is a local-first, goal-driven, LLM-guided modular trading platform for Linux CLI workflows.

The core design taken from the project documents is:

- LLMs generate constrained signals, not final numeric portfolio weights.
- Deterministic code owns portfolio construction, risk limits, execution, ledger state, and reconciliation.
- The system progresses from paper trading to broker sandbox to controlled live trading.
- Rust owns the operational core.
- Python owns stateless intelligence and research pipelines.

The current repository state is an initial scaffold aligned to the frozen blueprint and implementation plan.

## Monorepo Layout

```text
core/           Rust workspace for deterministic trading core
intelligence/   Python package boundary for stateless intelligence
schemas/        Shared generated schemas owned by Rust contracts
docs/           Extracted project summary and implementation notes
xtask/          Developer automation placeholder
```

## Current Scope

This scaffold intentionally focuses on Phase 1 foundations:

- domain boundaries
- goal and risk data structures
- broker abstraction
- execution proposal flow
- storage and schema placeholders
- CLI entry point

It does not yet implement:

- SQLite persistence
- real model providers
- market data ingestion
- broker adapters
- portfolio math
- grounding or eval harness

## Quick Start

```bash
cargo run --manifest-path core/Cargo.toml -p leviathan-cli
python3 intelligence/src/leviathan_ai/service.py
```

## Verification
```bash
cargo run --manifest-path core/Cargo.toml -p leviathan-cli
cargo test --manifest-path core/Cargo.toml
python3 intelligence/src/leviathan_ai/service.py
```


## Source Documents

- `leviathan_final_blueprint_v1.html`
- `leviathan_implementation_plan.html`
- `docs/project-summary.md`
