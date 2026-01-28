---
id: WI-2026-01-28-background-changer-zombies
title: "Background Changer Zombie Processes"
status: refining
priority: P1
risk: medium
owner: "Joseph Dalrymple"
created: 2026-01-28
updated: 2026-01-28
related: []
requires: []
supersedes: ""
target_release: ""
---

# Background Changer Zombie Processes

## Goal
- Outcome: Eliminate zombie processes from background changes and reduce bursty background updates.
- Why: `background_cmd` is currently spawned without reaping, and update events can fire rapidly.

## Success Metrics
- SM1: No zombie processes are left behind after background changes.
- SM2: Background updates coalesce during bursty workspace events (single update per burst).

## Scope
### In
- Replace per-call debounce thread with a single coalescing background-update worker.
- Ensure spawned background commands are reaped safely.
- Preserve existing background configuration behavior.

### Out (Non-goals)
- Adding new background configuration features.
- Changing how groups are resolved per output.
- Reworking polybar update semantics beyond what’s required for background updates.

## Constraints
- Must follow AGENTS.md workitem process.
- Docs and coverage requirements from AGENTS.md must be met or explicitly waived with a Decision.

## Assumptions
- A1: Background commands are generally short-lived (e.g., `nitrogen`, `feh`), but may occasionally be long-running.
- A2: Current debounce window (50ms) is acceptable unless a Decision changes it.

## System Context (exhaustive src/ review)
### Architecture snapshot
- CLI entry in `src/main.rs` routes to command modules.
- Polybar watcher in `src/commands/polybar/watch.rs` subscribes to i3 events and triggers updates.
- Background changes are driven by group assignments and outputs via `src/groups/` and `src/i3/`.

### Key modules / boundaries
- `src/commands/polybar/watch.rs`: background update logic + current per-event thread debounce + background command spawning.
- `src/commands/cli/group.rs`: triggers `update_and_bg()` for group assignment changes outside watch mode.
- `src/config/global/groups.rs`: background config (`background_cmd`, `background_screen_arg`, `backgrounds`).
- `src/i3/mod.rs`, `src/i3/outputs.rs`, `src/i3/workspace.rs`: output/workspace resolution used by `update_bg()`.
- `src/polybar/mod.rs`: uses `Command::output()` (already reaps processes).

### Conventions to follow
- Keep changes localized to polybar/watch unless needed elsewhere.
- Maintain existing CLI and config semantics.
- Prefer minimal dependencies; use stdlib concurrency primitives.

### Surfaces to avoid breaking
- `i3-wsman polybar watch` event handling.
- `group assign` and other CLI calls that expect backgrounds to update.
- Background command arguments derived from config.

## Requirements
### Functional
- FR1: Background updates are triggered via a single worker thread that coalesces events in a short window.
- FR2: Background commands are reaped to prevent zombies.

### Non-functional
- NFR1: No regression in responsiveness for workspace changes.
- NFR2: Worker thread should not unboundedly spawn or leak resources.

## Acceptance Criteria
- AC1 (Given/When/Then): Given repeated workspace events in quick succession, when updates occur, then only one background update is executed per burst.
- AC2 (Given/When/Then): Given background changes are applied, when the background command exits, then no zombie process remains.

## Design
### Proposed approach
- Introduce a single background-update worker thread (channel-based) in `src/commands/polybar/watch.rs`.
- `update_and_bg()` sends a request to the worker after calling `polybar::update()`.
- Worker blocks on `recv()` and coalesces additional requests with `recv_timeout(50ms)`.
- After coalescing, worker calls `update_bg()` exactly once.
- In `update_bg()`, spawn each background command and hand off the `Child` to a short-lived reaper thread that waits, preventing zombies without blocking the worker.

### Alternatives considered
- Option A: Keep current debounce thread and just add `wait()` (fixes zombies, but still spawns per event).
- Option B: Use a global atomic timer without a worker thread (more complex, less clear shutdown semantics).

### Interfaces and contracts (as applicable)
- APIs: no changes.
- CLI: no changes.
- Config: no changes.
- Data model: no changes.
- UI: no changes.

## Documentation Deliverables (repo policy applies; enumerate here)
### End-user docs
- Deliverables:
	- Describe background update coalescing behavior and any user-visible impact.
- Location(s):
	- `README.md` (exact section TBD).
- Validation (command/CI job):
	- Unknown (see Open Question Q1).

### Contributor docs
- Deliverables:
	- Note background worker design and rationale for reaping in developer docs.
- Location(s):
	- `README.md` or a developer doc (TBD).
- Validation (command/CI job):
	- Unknown (see Open Question Q1).

### Rust docs (public API rustdoc)
- Targets (modules/APIs):
	- None expected (no public API changes).
- Examples/doctests:
	- N/A.
- Validation (command/CI job):
	- Unknown (see Open Question Q1).

## Coverage Plan (repo policy applies; specify measurement + scope)
- Measurement command(s) / CI job:
	- Unknown (see Open Question Q1).
- Enforcement scope:
	- no-regression on touched code (pending confirmation).
- Expected hard-to-cover areas (if any):
	- Background worker concurrency paths may be hard to unit test.
- If exclusions are needed, record as a Decision.

## Observability
- Logs:
	- None currently; keep existing behavior unless a Decision adds logging.
- Metrics:
	- None.
- Traces:
	- None.

## Test Plan
- Unit:
	- If feasible, add tests for coalescing logic (may require new testable module).
- Integration:
	- Not currently present; likely manual validation.
- E2E (if applicable):
	- Manual: run `i3-wsman polybar watch` and trigger rapid workspace changes.
- Manual validation:
	- Confirm background command execution and verify no zombies (e.g., `ps`/`top`).

## Risks & Mitigations
- Risk: Long-running background commands could keep reaper threads alive.
	Mitigation: Acceptable tradeoff; reaper threads are minimal and bounded by command count.
- Risk: Coalescing could delay updates too long.
	Mitigation: Keep 50ms window; revisit if user feedback indicates delay.

## Rollout Plan
- Flags:
	- None.
- Migration:
	- None.
- Backward compatibility:
	- Config and CLI unchanged.
- Rollback:
	- Revert to prior debounce + spawn implementation.

## Open Questions
- Q1: What are the canonical coverage/docs/test commands and CI jobs for this repo?
	Owner: Joseph
	Plan to resolve: Provide commands or point to CI configuration.
	Status: open
	Resolution:
- Q2: Keep debounce at 50ms or make it configurable?
	Owner: Joseph
	Plan to resolve: Confirm desired behavior.
	Status: open
	Resolution:
- Q3: Should background updates from short-lived CLI commands (e.g., `group assign`) be guaranteed before process exit, or is best-effort acceptable?
	Owner: Joseph
	Plan to resolve: Confirm desired behavior.
	Status: open
	Resolution:

## Decisions
- D1 (2026-01-28): Use a single coalescing worker thread for background updates to reduce event-driven churn.
	Context: Current per-event thread spawning is noisy and contributes to excessive process creation.
	Options:
	- Keep debounce thread and add reaping.
	- Single worker thread with coalescing.
	Decision: Single worker thread with coalescing.
	Rationale: Reduces thread churn and keeps behavior predictable.
	Consequences: Additional shared state (channel/worker) in `watch.rs`.

## Execution Plan (atomic steps)

### Step 1: Design worker threading + channel
- Change summary: Introduce a background-update worker and channel in `src/commands/polybar/watch.rs`.
- Files likely touched: `src/commands/polybar/watch.rs`.
- Implementation notes:
	- Use stdlib `mpsc` channel or `crossbeam` only if already in deps (prefer std).
	- Worker loops on `recv()` and drains with `recv_timeout(50ms)`.
- Verification:
	- Build (command TBD, see Q1).
- Docs update in this step:
	- None.
- Coverage impact + how validated:
	- None yet (command TBD).
- Rollback notes:
	- Revert to current debounce implementation.

### Step 2: Wire `update_and_bg()` to worker
- Change summary: Replace per-call thread spawn with send to worker.
- Files likely touched: `src/commands/polybar/watch.rs`.
- Implementation notes:
	- Keep `polybar::update()` call immediate.
	- Ensure worker initialized lazily/once.
- Verification:
	- Build (command TBD).
- Docs update in this step:
	- None.
- Coverage impact + how validated:
	- None yet (command TBD).
- Rollback notes:
	- Revert `update_and_bg()` to prior flow.

### Step 3: Reap background commands
- Change summary: Add reaper for spawned background commands to prevent zombies.
- Files likely touched: `src/commands/polybar/watch.rs`.
- Implementation notes:
	- Spawn command, then move `Child` into a short-lived thread that calls `wait()`.
- Verification:
	- Manual validation: run background change and check for zombies.
- Docs update in this step:
	- None.
- Coverage impact + how validated:
	- None yet (command TBD).
- Rollback notes:
	- Revert to previous spawn behavior.

### Step 4: Documentation + validation updates
- Change summary: Update docs to reflect new background update behavior and add verification notes.
- Files likely touched: `README.md` (exact location TBD).
- Implementation notes:
	- Add section describing coalescing and background command behavior.
- Verification:
	- Docs build command (TBD).
- Docs update in this step:
	- End-user + contributor notes.
- Coverage impact + how validated:
	- None.
- Rollback notes:
	- Revert documentation changes if needed.

## Work Log
- 2026-01-28:
	Summary: Created initial workitem and captured system context.
	Links (commits/PRs):
	Notes:

## Change Log (workitem edits)
- 2026-01-28:
	What changed: Rewrote workitem to new template, added frontmatter, system context, requirements, design, and plan.
	Why: Align with updated workitem process and folder conventions.
