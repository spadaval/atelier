# `atelier review`

Primary role: Reviewer.

Primary question: "How do I manage the configured review artifact for issue or
epic work?"

## Decision Record

| Operator question | Role | Product/cognitive cost | Architecture/code cost | Verdict | Next action |
| --- | --- | --- | --- | --- | --- |
| How do I act on the configured review artifact? | Reviewer | Manual provider verbs and inputs make routine review costly. | Provider adapters and issue/branch context risk duplicated policy. | Simplify | Infer routine context and keep one show surface plus one submit surface. |

## Assessment

- Name: Correct. Review is a first-class workflow concern, but it should not
  decide lifecycle state by itself.
- Documentation: Visible in root help. Role guides should send operators here
  only when status, transition, or configured review output names a review
  action.
- Design: Simplified. `atelier review open` derives routine context from the
  issue, branch policy, workflow state, and configured review mode.
  `atelier review show` owns inspection; `atelier review submit` owns comment,
  approval, and change-request mutations.
- Output hierarchy: Issue/owner, review artifact identifier or URL, provider
  mode, role source, action result, then `issue transition`.

## Subcommands

| Form | Primary role | Operator purpose | Fit |
| --- | --- | --- | --- |
| `review open [--issue <id>] [--existing <url-or-number>]` | Reviewer | Open an issue-derived artifact or attach an existing provider artifact during recovery/import. | Simplify and keep. Routine creation derives issue, owner, title, body, source branch, target branch, mode/provider, and role context. |
| `review show [--issue <id>] [--comments] [--unresolved]` | Reviewer | Inspect review authority, state, detail, and optional discussion. | Keep; folds former status and comments verbs. |
| `review submit [--issue <id>] --approve|--request-changes|--comment <text>` | Reviewer | Submit exactly one review decision or comment. | Keep; folds former comment, approve, and request-changes verbs. |
| `review merge [--issue <id>]` | Manager/orchestrator | Merge or confirm the linked artifact without changing workflow state. | Advanced; should follow workflow guidance. |
| `review resolve <finding> [--issue <id>]` | Reviewer | Resolve a native room finding. | Good. |

## Explicit Complexity-Budget Outcomes

- Keep `review`, `review show`, `review resolve`, and `review merge`: each owns a
  distinct operator job.
- Simplify `review open`: the routine path accepts only optional issue and role
  selection, then derives title, body, source branch, target branch, owner, and
  review mode/provider context. `--existing` is the explicit provider
  recovery/import path.
- Fold `review link` into `review open --existing`.
- Fold `review status` and `review comments` into `review show`; comments and
  unresolved filtering are flags on the surviving inspection surface.
- Fold `review comment`, `review approve`, and `review request-changes` into
  `review submit`, which requires exactly one submit action.
- Hide no review verb as a compatibility path. The folded verbs are removed
  from parsing and help.
- Remove manual `review open --title`, `--body`, `--source-branch`, and
  `--target-branch` plumbing. There is no routine or hidden fallback alias.

## Complexity Budget

`review` stays within budget by exposing artifact jobs rather than mirroring
provider operations. Provider-specific implementation remains behind the
configured review mode, while issue and workflow state own routine context.

## Human Output Debt

Review output should make the review authority and state obvious before showing
provider details. Interactive color may distinguish approved, changes
requested, unresolved findings, mergeable, and blocked states, but the text must
remain complete without color. Comments and findings should be bounded by
default and route to focused commands for full discussion.
