---
id: WI-2026-01-28-clippy-warnings-and-tests
title: "Clippy Cleanup and Test Coverage"
status: draft
priority: P2
risk: medium
owner: "Joseph Dalrymple"
created: 2026-01-28
updated: 2026-01-28
related: []
requires: []
supersedes: ""
target_release: ""
---

# Clippy Cleanup and Test Coverage

## Goal
- Outcome: Resolve existing clippy warnings and add tests to meet coverage thresholds.
- Why: Current `cargo clippy -- -D warnings` fails with many warnings and `cargo llvm-cov` reports 0% coverage due to missing tests.

## Success Metrics
- SM1: `cargo clippy --all-targets --all-features -- -D warnings` passes.
- SM2: `cargo llvm-cov --workspace --all-features --fail-under-lines 80 --fail-under-regions 70` passes.

## Scope
### In
- Fix clippy warnings across `src/` identified by the current clippy run.
- Add unit/integration tests to raise coverage above thresholds.
- Document testing strategy and any coverage exclusions.

### Out (Non-goals)
- Feature work unrelated to lint/test quality.
- Large refactors that change behavior beyond lint/test needs.

## Constraints
- Follow AGENTS.md workitem process and readiness gate.
- Documentation and coverage requirements apply unless explicitly waived via Decision.

## Assumptions
- A1: Clippy warnings are pre-existing and can be resolved without behavior changes.
- A2: Test coverage can be improved without requiring full i3 IPC integration.

## System Context (exhaustive src/ review)
### Architecture snapshot
- CLI entrypoint in `src/main.rs` routes to action, polybar, and CLI commands.
- i3 IPC integrations and workspace/output modeling in `src/i3/` and `src/groups/`.
- Polybar integration and watch loop in `src/commands/polybar/`.
- Config is loaded from TOML via `src/config/` and state persisted under temp dirs via `src/state.rs`.

### Key modules / boundaries
- `src/commands/*`: user-facing commands and formatting logic.
- `src/i3/*`: IPC accessors and workspace/output logic.
- `src/state.rs`: file locks and persisted state.
- `src/config/*`: config loading/parsing.

### Conventions to follow
- Prefer minimal changes that do not alter runtime behavior.
- Use stdlib patterns consistent with the rest of the repo.
- Keep tests deterministic; avoid requiring a running i3 unless explicitly accepted.

### Surfaces to avoid breaking
- CLI command parsing and behavior.
- Config compatibility with existing examples.
- Polybar module output formatting.

## Requirements
### Functional
- FR1: Eliminate clippy warnings flagged by `-D warnings`.
- FR2: Add tests that execute key code paths to meet coverage thresholds.

### Non-functional
- NFR1: Preserve existing behavior and outputs.
- NFR2: Keep test runtime reasonable and deterministic.

## Acceptance Criteria
- AC1 (Given/When/Then): Given the repo, when running clippy with `-D warnings`, then it succeeds.
- AC2 (Given/When/Then): Given the repo, when running llvm-cov with thresholds, then coverage passes.

## Design
### Proposed approach
- Address clippy warnings by applying idiomatic Rust fixes (e.g., `is_empty`, `+=`, `cloned`, `Display` impls).
- Add focused unit tests around pure logic modules (`common`, `groups`, `config` helpers) and lightweight integration tests if feasible.
- If needed, introduce feature-gated stubs or test helpers to avoid i3 IPC dependency.

### Alternatives considered
- Option A: Allow clippy warnings and reduce lint strictness.
- Option B: Add CI-only lint ignores.

### Interfaces and contracts (as applicable)
- APIs: no planned changes initially.
- CLI: no planned changes initially.
- Config: no planned changes initially.
- Data model: no planned changes initially.
- UI: no planned changes initially.

## Documentation Deliverables (repo policy applies; enumerate here)
### End-user docs
- Deliverables:
	- None expected (lint/test-only change).
- Location(s):
	- N/A.
- Validation (command/CI job):
	- Manual review (no build step).

### Contributor docs
- Deliverables:
	- Document testing strategy and any coverage exclusions.
- Location(s):
	- `README.md` or a developer doc (TBD).
- Validation (command/CI job):
	- Manual review.

### Rust docs (public API rustdoc)
- Targets (modules/APIs):
	- None expected unless public APIs change.
- Examples/doctests:
	- N/A.
- Validation (command/CI job):
	- `cargo doc --no-deps`

## Coverage Plan (repo policy applies; specify measurement + scope)
- Measurement command(s) / CI job:
	- `cargo llvm-cov --workspace --all-features --fail-under-lines 80 --fail-under-regions 70`
	- Branch coverage (nightly): `cargo +nightly llvm-cov --workspace --all-features --branch`
- Enforcement scope:
	- repo-wide (targeting existing thresholds)
- Expected hard-to-cover areas (if any):
	- i3 IPC interactions; may need mocks or exclusion decisions.
- If exclusions are needed, record as a Decision.

## Observability
- Logs:
	- None planned.
- Metrics:
	- None.
- Traces:
	- None.

## Test Plan
- Unit:
	- `cargo test`
- Integration:
	- `cargo test --tests`
- E2E (if applicable):
	- None expected.
- Manual validation:
	- Spot-check key command paths.
- Lint/format:
	- `cargo fmt --check`
	- `cargo clippy --all-targets --all-features -- -D warnings`

## Risks & Mitigations
- Risk: Fixing clippy warnings changes behavior.
	Mitigation: Keep changes mechanical and add tests for touched code paths.
- Risk: Coverage thresholds are unattainable without large refactors.
	Mitigation: Add exclusions with Decisions and revisit thresholds if needed.

## Rollout Plan
- Flags:
	- None.
- Migration:
	- None.
- Backward compatibility:
	- No user-facing changes.
- Rollback:
	- Revert lint/test changes if regressions appear.

## Open Questions
- Q1: Which modules should be prioritized for test coverage to reach thresholds fastest?
	Owner: Joseph
	Plan to resolve: Confirm target modules.
	Status: open
	Resolution:
- Q2: Are we allowed to add test-only mocks for i3 IPC to avoid runtime dependencies?
	Owner: Joseph
	Plan to resolve: Confirm acceptable testing approach.
	Status: open
	Resolution:

## Decisions
- None yet.

## Execution Plan (atomic steps)

### Step 1: Inventory clippy warnings
- Change summary: Capture and categorize clippy warnings from the latest run.
- Files likely touched: None.
- Implementation notes:
	- Group by module/type to sequence fixes.
- Verification:
	- `cargo clippy --all-targets --all-features -- -D warnings` (expected fail).
- Docs update in this step:
	- None.
- Coverage impact + how validated:
	- None.
- Rollback notes:
	- N/A.

### Step 2: Mechanical clippy fixes
- Change summary: Apply safe, mechanical changes (e.g., `is_empty`, `+=`, `cloned`, `Display` impls).
- Files likely touched: `src/**` as needed.
- Implementation notes:
	- Avoid behavior changes.
- Verification:
	- `cargo clippy --all-targets --all-features -- -D warnings`.
- Docs update in this step:
	- None.
- Coverage impact + how validated:
	- None.
- Rollback notes:
	- Revert touched files.

### Step 3: Add tests for coverage
- Change summary: Add unit/integration tests to meet thresholds.
- Files likely touched: `src/**`, `tests/**`.
- Implementation notes:
	- Focus on deterministic logic.
- Verification:
	- `cargo test`, `cargo llvm-cov --workspace --all-features --fail-under-lines 80 --fail-under-regions 70`.
- Docs update in this step:
	- Document test approach and coverage notes.
- Coverage impact + how validated:
	- llvm-cov report.
- Rollback notes:
	- Revert test additions.

### Step 4: Documentation + validation
- Change summary: Update contributor docs with testing/coverage guidance.
- Files likely touched: `README.md` or docs.
- Implementation notes:
	- Ensure docs match actual commands.
- Verification:
	- `cargo doc --no-deps`.
- Docs update in this step:
	- Contributor notes.
- Coverage impact + how validated:
	- None.
- Rollback notes:
	- Revert doc changes.

## Work Log
- 2026-01-28:
	Summary: Created workitem after exhaustive `src/` review.
	Links (commits/PRs):
	Notes:

## Change Log (workitem edits)
- 2026-01-28:
	What changed: Created initial workitem for clippy warnings and test coverage.
	Why: Track follow-up quality work separate from the instability fix.
