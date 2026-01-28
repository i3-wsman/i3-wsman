# AGENTS.md

## Purpose
This repository uses a structured multi-agent workflow to plan and implement changes safely and repeatably. A **workitem** is the single source of truth for intent, scope, decisions, and step-by-step execution.

This document is the collaboration contract. Repo-wide operational details (folders, frontmatter schema, commands) live in `.workitems/README.md`.

## Definitions
- Workitem: A Markdown document that defines a goal, context, requirements, acceptance criteria, design, open questions, decisions, and an atomic execution plan.
- Open Question: A tracked unknown that must be resolved (or explicitly accepted) before execution proceeds.
- Decision: A recorded choice with rationale and consequences that keeps the workitem consistent with implementation reality.

## Collaboration Modes
### PO/BA/PM Mode (Planning/Refinement)
Responsibilities:
- Define goal, scope, constraints, and success metrics.
- Exhaustively review `src/` (and critical adjacent dirs as applicable) and record System Context.
- Produce objective acceptance criteria.
- Identify risks, dependencies, rollout, and test strategy.
- Break work into atomic, verifiable steps.
- Track Open Questions and record Decisions.

Exit criteria for `status: ready`:
- Scope and non-goals are explicit.
- Acceptance criteria are objective and testable.
- Steps are atomic and each includes verification.
- Documentation deliverables are enumerated (end-user + contributor + rustdoc expectations).
- Coverage plan exists and is feasible under repo policy.
- Open Questions are resolved, or explicitly marked as blocking execution.

### Engineer Mode (Execution)
Responsibilities:
- Execute exactly one planned step at a time.
- Run the step verification.
- Update the workitem with decisions, discoveries, and plan adjustments.
- Keep the workitem consistent with the implementation reality.
- Do not continue through ambiguity; ambiguity becomes an Open Question or a Decision.

## Mandatory Context Refresh
- At the start of planning/refinement for any workitem: exhaustively review `src/` (and critical adjacent dirs as applicable) and record System Context.
- Immediately before executing Step 1, or when resuming after an absence: re-review `src/` and the workitem to restore context.

## Implementation Rules
- No code changes without a workitem approved by the user and in `status: ready` (or `status: in_progress` if already started).
- Execute one step at a time. Do not start the next step until:
	- current step verification passes, and
	- the workitem has been updated to reflect reality, and
	- internal consistency has been confirmed.

## Decision and Consistency Rules
- Any acknowledged question/concern/decision point must be recorded in the workitem.
- After recording a Decision, update all impacted sections (Design/Steps/Acceptance Criteria/Scope) and confirm the workitem is consistent before continuing.
- Decisions must include:
	- context,
	- options considered,
	- decision made,
	- rationale,
	- consequences / follow-ups.

## Hard Requirements (repo-wide)
### Documentation
Workitems must deliver, at minimum:
- Extensive end-user documentation (as applicable to the change)
- Extensive contributor documentation (as applicable to the change)
- Rust documentation for public APIs (rustdoc), including examples where applicable

Any exception requires a Decision in the workitem with rationale and compensating guidance.

### Coverage
Minimum thresholds:
- Line coverage: >= 80%
- Branch coverage: >= 70%

Workitems must specify:
- how coverage is measured (canonical command and/or CI job),
- what scope is enforced (repo-wide, per-crate, or no-regression on touched code).

Exclusions/deviations:
- Allowed only with an explicit Decision in the workitem.
- The Decision must list exact excluded paths/modules, rationale, compensating validation, and revisit criteria.

## Stop Conditions (return to Refinement)
Execution must pause if:
- requirements are ambiguous or contradictory,
- new dependencies materially change scope,
- proposed changes conflict with established conventions,
- verification fails outside the boundary of the current step,
- documentation or coverage requirements cannot be met without changing scope.

## Status Vocabulary
Workitem statuses:
- draft
- refining
- ready
- in_progress
- blocked
- done
- archived

Allowed transitions:
- draft → refining → ready → in_progress → done
- any → blocked (when blocked); blocked returns to prior status when unblocked
- any → archived (cancelled/superseded)

Operational folder mappings and conventions are defined in `.workitems/README.md`.

