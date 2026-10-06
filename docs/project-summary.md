# Leviathan Project Summary

This summary is extracted from `leviathan_final_blueprint_v1.html` and `leviathan_implementation_plan.html`.

## Purpose

Leviathan is intended to be a personal Linux CLI trading platform that starts from a user goal:

- initial capital
- target amount
- target period
- risk tolerance

It then evaluates whether that goal is realistic, builds a risk budget for the current cycle, screens the market, asks the intelligence layer for constrained signals, and passes those signals through deterministic portfolio, risk, and execution code.

## Target Architecture

1. User goal
2. Goal feasibility engine
3. Progress-adaptive risk budget controller
4. Point-in-time research layer
5. Quant pre-filter
6. AI/LLM pipeline
7. Grounding checker
8. Portfolio constructor
9. Risk engine
10. Execution policy and router
11. Broker adapter
12. Ledger, reports, evaluation, and next cycle

## Non-Negotiable Design Rules

- LLMs do not own final numbers such as exact position weights.
- Deterministic code owns risk limits, target weights, order permission, execution state, and accounting.
- The intelligence service is stateless and must not write to the database or call brokers directly.
- The system fails closed. If data is stale, schema validation fails, or the model layer fails, the default outcome is `HOLD`.
- The core must remain broker-independent.

## Implementation Direction

- Rust: domain, goal engine, portfolio construction, risk, execution, storage, broker abstraction, CLI
- Python: model routing, research, prompt orchestration, grounding, reports
- SQLite: operational store
- Shared schemas: generated from Rust contracts and consumed read-only from Python

## Phase Priorities Reflected In This Scaffold

- Phase 1: core workspace boundaries
- Phase 2: placeholders for research and screening boundaries
- Phase 3: explicit contracts boundary between Rust and Python
- Phase 4: execution mode and risk budget placeholders

## Recommended Early Build Order

1. Core domain model and ledger invariants
2. Goal feasibility prototype
3. Conviction/weight-band contract
4. Deterministic portfolio constructor minimum version
5. Basic quant pre-filter
6. Paper execution path
