# Workitems

Workitems are Markdown documents used to plan, refine, execute, and record changes. They are the source of truth for what was built and why.

Repo-wide collaboration rules live in `AGENTS.md`. This document defines operational conventions: folder layout, naming, frontmatter schema, and checklists.

## Folder Layout

Recommended layout:

- `.workitems/_template.md`					Standard workitem template
- `.workitems/active/backlog/`				draft/refining (ideation and not-ready)
- `.workitems/active/ready/`				ready to start (not yet started)
- `.workitems/active/in-progress/`			currently being executed
- `.workitems/active/blocked/`				blocked workitems
- `.workitems/done/`						completed workitems (recent)
- `.workitems/archived/`					cancelled / superseded / not doing

A workitem’s `status` must match its folder location.

## Naming

Filename format:

- `YYYY-MM-DD-short-slug.md`

Workitem id format (frontmatter field `id`):

- `WI-YYYY-MM-DD-short-slug`

## Frontmatter Schema

All workitems must begin with YAML frontmatter.

Required fields:
- `id`: string (`WI-YYYY-MM-DD-short-slug`)
- `title`: string
- `status`: `draft|refining|ready|in_progress|blocked|done|archived`
- `priority`: `P0|P1|P2|P3`
- `risk`: `low|medium|high`
- `owner`: string
- `created`: `YYYY-MM-DD`
- `updated`: `YYYY-MM-DD`

Optional fields:
- `related`: array of strings (issues, PRs, docs, discussions)
- `requires`: array of strings (dependencies)
- `supersedes`: string (workitem id)
- `target_release`: string

Example:

```yaml
---
id: WI-2026-01-28-example-slug
title: "Example Workitem"
status: draft
priority: P2
risk: medium
owner: "Joseph"
created: 2026-01-28
updated: 2026-01-28
related:
	- "PR: <link or id>"
requires: []
supersedes: ""
target_release: ""
---
```

## Status-to-Folder Mapping

- `draft` or `refining` → `.workitems/active/backlog/`
- `ready` → `.workitems/active/ready/`
- `in_progress` → `.workitems/active/in-progress/`
- `blocked` → `.workitems/active/blocked/`
- `done` → `.workitems/done/`
- `archived` → `.workitems/archived/`

## Ready Gate Checklist

A workitem can move to `status: ready` only if:
- Goal, scope, and non-goals are explicit.
- System Context includes an exhaustive `src/` review summary.
- Acceptance Criteria are objective and testable.
- Execution Plan steps are atomic and each includes verification.
- Documentation deliverables are enumerated (end-user + contributor + rustdoc scope).
- Coverage plan exists and references the canonical measurement command/CI job.
- Open Questions are resolved or explicitly marked as blocking.

## Step Execution Checklist

For each step during execution:
- Re-read the workitem and confirm prerequisites.
- Implement only what the step requires.
- Run the step verification commands.
- Update the workitem:
	- decisions made,
	- open questions discovered,
	- plan adjustments (with rationale),
	- what was verified and how.
- Confirm the workitem is consistent before proceeding to the next step.

## Coverage Policy

Repo-wide minimum thresholds (from `AGENTS.md`):
- Line coverage: >= 80%
- Branch coverage: >= 70%

Each workitem must specify:
- the canonical measurement command(s),
- whether enforcement is repo-wide, per-crate, or no-regression for touched code.

Exclusions/deviations:
- Allowed only with a Decision in the workitem specifying:
	- exact excluded paths/modules,
	- rationale,
	- compensating validation,
	- revisit criteria.

## Documentation Policy

Repo-wide documentation requirements (from `AGENTS.md`):
- Extensive end-user docs (as applicable)
- Extensive contributor docs (as applicable)
- Rustdoc for public APIs, including examples where applicable

Each workitem must list:
- doc deliverables and file locations,
- rustdoc targets (modules/APIs),
- how docs are built/validated.

## Canonical Commands

This section must be filled in for your repository.

Coverage:
- Line coverage command:
	- `cargo llvm-cov --workspace --all-features --fail-under-lines 80 --fail-under-regions 70`
- Branch coverage command:
	- `cargo +nightly llvm-cov --workspace --all-features --branch`
- CI job enforcing coverage:
	- `None (not yet defined)`

Docs:
- End-user docs build command:
	- `None (manual review of README changes)`
- Contributor docs build command:
	- `None (manual review of contributor-facing docs)`
- Rustdoc build command:
	- `cargo doc --no-deps`
- CI job enforcing docs:
	- `None (not yet defined)`

Tests:
- Unit:
	- `cargo test`
- Integration:
	- `cargo test --tests`
- Lint/format:
	- `cargo fmt --check`
	- `cargo clippy --all-targets --all-features -- -D warnings`

## Archival Rules

Move a workitem to `.workitems/archived/` and set `status: archived` when:
- cancelled,
- superseded by another workitem,
- no longer aligned with product direction.

Record the reason in the workitem’s Change Log, and set `supersedes`/`related` links as needed.
