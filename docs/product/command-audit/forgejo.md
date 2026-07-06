# `atelier forgejo`

Primary role: Admin.

Primary question: "How do I configure and verify Forgejo-backed review
identity and permissions?"

## Assessment

- Name: Correct only as a provider-specific admin surface. It should not leak
  into normal worker, reviewer, or manager loops unless the repository is
  configured for Forgejo and a repair path names it.
- Documentation: Hidden from root help. It should remain out of ordinary role
  loops unless an explicit provider setup or recovery path names it.
- Design: The supported surface is intentionally small: check the configured
  role accounts or provision them from workflow-owned role parameters.
- Output hierarchy: Configured provider, role accounts, permission check
  result, remediation, then `review` or config file paths.

## Subcommands

| Form | Primary role | Operator purpose | Fit |
| --- | --- | --- | --- |
| `forgejo roles check` | Admin | Verify configured role author users, repo permissions, and sudo access. | Good. |
| `forgejo roles provision` | Admin | Create missing role author users and grant repository access. | Good as explicit admin setup. |

## Complexity Budget

`forgejo` is over budget as a normal visible root because it exposes one
provider's setup machinery. Keep it hidden or fold it behind provider-neutral
review/admin diagnostics unless a repository setup flow explicitly needs it.

## Cutting Note

Provider-specific setup is legitimate, but rejected options do not remain in the
help surface. Role authors are configured through workflow action parameters, not
through a provisioning flag that mutates configuration.
