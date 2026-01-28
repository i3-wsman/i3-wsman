---
id: WI-2026-01-28-background-changer-zombies
title: "Background Changer Zombie Processes"
status: done
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
- Current debounce uses a per-call thread with a global flag and joins the previous thread, which can block callers briefly.

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
- FR3: CLI-triggered background updates complete before process exit.

### Non-functional
- NFR1: No regression in responsiveness for workspace changes.
- NFR2: Worker thread should not unboundedly spawn or leak resources.

## Acceptance Criteria
- AC1 (Given/When/Then): Given repeated workspace events in quick succession, when updates occur, then only one background update is executed per burst.
- AC2 (Given/When/Then): Given background changes are applied, when the background command exits, then no zombie process remains.
- AC3 (Given/When/Then): Given a CLI-triggered background update, when the command returns to the shell, then the background command has completed and been reaped.

## Design
### Proposed approach
- Introduce a single background-update worker thread (channel-based) in `src/commands/polybar/watch.rs`.
- `update_and_bg()` sends a request to the worker after calling `polybar::update()`.
- Worker blocks on `recv()` and coalesces additional requests with `recv_timeout(50ms)`.
- After coalescing, worker calls `update_bg()` exactly once.
- In `update_bg()`, spawn each background command and hand off the `Child` to a short-lived reaper thread that waits, preventing zombies without blocking the worker.
- For CLI-triggered updates, provide a blocking path (e.g., `update_and_bg_blocking()`) that runs the background update and waits for completion before returning.

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
	- None (see Decision D4).
- Location(s):
	- N/A.
- Validation (command/CI job):
	- N/A.

### Contributor docs
- Deliverables:
	- None (see Decision D4).
- Location(s):
	- N/A.
- Validation (command/CI job):
	- N/A.

### Rust docs (public API rustdoc)
- Targets (modules/APIs):
	- None expected (no public API changes).
- Examples/doctests:
	- N/A.
- Validation (command/CI job):
	- `cargo doc --no-deps`

## Coverage Plan (repo policy applies; specify measurement + scope)
- Measurement command(s) / CI job:
	- `cargo llvm-cov --workspace --all-features --fail-under-lines 80 --fail-under-regions 70`
	- `cargo +nightly llvm-cov --workspace --all-features --branch`
	- CI: None (not yet defined).
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
	Status: resolved
	Resolution: Use `.workitems/README.md` canonical commands (llvm-cov line/branch, `cargo doc --no-deps`, manual doc review).
- Q2: Keep debounce at 50ms or make it configurable?
	Owner: Joseph
	Plan to resolve: Confirm desired behavior.
	Status: resolved
	Resolution: Keep 50ms debounce window.
- Q3: Should background updates from short-lived CLI commands (e.g., `group assign`) be guaranteed before process exit, or is best-effort acceptable?
	Owner: Joseph
	Plan to resolve: Confirm desired behavior.
	Status: resolved
	Resolution: CLI-triggered background updates must complete before process exit (blocking path).

## Decisions
- D1 (2026-01-28): Use a single coalescing worker thread for background updates to reduce event-driven churn.
	Context: Current per-event thread spawning is noisy and contributes to excessive process creation.
	Options:
	- Keep debounce thread and add reaping.
	- Single worker thread with coalescing.
	Decision: Single worker thread with coalescing.
	Rationale: Reduces thread churn and keeps behavior predictable.
	Consequences: Additional shared state (channel/worker) in `watch.rs`.
- D2 (2026-01-28): Keep debounce window at 50ms (not configurable).
	Context: Current debounce is 50ms; question was whether to add configurability.
	Options:
	- Keep 50ms fixed.
	- Make debounce configurable via config or flag.
	Decision: Keep 50ms fixed.
	Rationale: Simple behavior; avoids new config surface for now.
	Consequences: Users cannot tune burst coalescing without code change.
- D3 (2026-01-28): CLI-triggered background updates must complete before exit.
	Context: CLI commands often exit immediately; detached threads may not run or reap children.
	Options:
	- Best-effort enqueue (may drop on exit).
	- Blocking path for CLI updates.
	Decision: Provide a blocking path for CLI-triggered background updates.
	Rationale: Ensures background commands run and are reaped even for short-lived CLI invocations.
	Consequences: CLI commands may block while background commands run; document this behavior.
- D4 (2026-01-28): Skip end-user and contributor doc updates for background update behavior.
	Context: README updates were reverted; behavior is internal and not user-facing enough to document.
	Options:
	- Add README notes describing coalescing + blocking behavior.
	- Skip documentation changes.
	Decision: Skip documentation changes.
	Rationale: End users do not need this internal detail; contributor doc update not requested.
	Consequences: Behavior remains undocumented; rely on workitem record for historical context.

## Execution Plan (atomic steps)

### Step 1: Add worker + coalescing queue
- Change summary: Replace the per-call debounce thread with a background-update worker and channel in `src/commands/polybar/watch.rs`.
- Files likely touched: `src/commands/polybar/watch.rs`.
- Implementation notes:
	- Use stdlib `mpsc` channel or `crossbeam` only if already in deps (prefer std).
	- Worker loops on `recv()` and drains with `recv_timeout(50ms)`.
	- Update `update_and_bg()` to enqueue a request instead of spawning its own thread.
- Verification:
	- `cargo build`
- Docs update in this step:
	- None.
- Coverage impact + how validated:
	- None yet (command TBD).
- Rollback notes:
	- Revert to current debounce implementation.

### Step 2: CLI blocking path + reaping behavior
- Change summary: Add a blocking variant for CLI usage and ensure background commands are reaped.
- Files likely touched: `src/commands/polybar/watch.rs`.
- Implementation notes:
	- Keep `polybar::update()` call immediate.
	- Ensure worker initialized lazily/once.
	- Add a blocking variant for CLI usage (e.g., `update_and_bg_blocking()`), and update CLI call sites to use it.
	- Ensure background commands are reaped (async for worker path, blocking waits for CLI path).
- Verification:
	- `cargo build`
- Docs update in this step:
	- None.
- Coverage impact + how validated:
	- None yet (command TBD).
- Rollback notes:
	- Revert `update_and_bg()` to prior flow.

### Step 3: Manual validation + doc decision
- Change summary: Validate behavior in a real i3/polybar session and record documentation decision.
- Files likely touched: None.
- Implementation notes:
	- Confirm no zombies after background command exits.
	- Confirm CLI-triggered update blocks until background command completes.
- Verification:
	- Manual validation (i3/polybar session).
- Docs update in this step:
	- None (Decision D4).
- Coverage impact + how validated:
	- None.
- Rollback notes:
	- N/A.

## Work Log
- 2026-01-28:
	Summary: Created initial workitem and captured system context.
	Links (commits/PRs):
	Notes:
- 2026-01-28:
	Summary: Step 1 complete. Replaced per-call debounce with a worker + channel; `update_and_bg()` now enqueues coalesced updates.
	Links (commits/PRs):
	Notes: `cargo build` succeeded with existing warnings (deprecated `State::global_groups`, unused `get_filtered_workspaces`, unused `Workspace::get_neighbor`).
- 2026-01-28:
	Summary: Step 2 complete. Added blocking CLI path and reaped background commands; `group assign` now blocks until background update completes.
	Links (commits/PRs):
	Notes: `cargo build` succeeded with the same existing warnings.
- 2026-01-28:
	Summary: Step 3 attempted. README updates were reverted; rustdoc was built.
	Links (commits/PRs):
	Notes: `cargo doc --no-deps` succeeded. Manual i3/polybar validation not run in this environment.
- 2026-01-28:
	Summary: Manual validation completed by Joseph; README documentation changes reverted.
	Links (commits/PRs):
	Notes: Confirmed no zombies and CLI blocking behavior in real session (per user).

## Change Log (workitem edits)
- 2026-01-28:
	What changed: Rewrote workitem to new template, added frontmatter, system context, requirements, design, and plan.
	Why: Align with updated workitem process and folder conventions.
- 2026-01-28:
	What changed: Moved to in-progress and consolidated steps to cover worker wiring plus CLI blocking + reaping.
	Why: Avoid unused worker code and keep execution steps cohesive.
