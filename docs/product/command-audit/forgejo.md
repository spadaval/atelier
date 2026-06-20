# `atelier forgejo`

Primary role: Admin.

Primary question: "How do I configure and verify Forgejo-backed review
identity and permissions?"

## Assessment

- Name: Correct only as a provider-specific admin surface. It should not leak
  into normal worker, reviewer, or manager loops unless the repository is
  configured for Forgejo and a repair path names it.
- Documentation: Visible in root help because provider setup is a real admin
  job. It should remain out of ordinary role loops.
- Design: Mostly correct, but `roles provision --write-config` is a rejected
  legacy flag and should be removed instead of preserved as a callable option.
- Output hierarchy: Configured provider, role accounts, permission check
  result, remediation, then `review` or config file paths.

## Subcommands

| Form | Primary role | Operator purpose | Fit |
| --- | --- | --- | --- |
| `forgejo roles check` | Admin | Verify configured role author users, repo permissions, and sudo access. | Good. |
| `forgejo roles provision` | Admin | Create missing role author users and grant repository access. | Good as explicit admin setup. |
| `forgejo roles provision --write-config` | Admin | Rejected legacy compatibility flag. | Remove. |

## Cutting Note

Provider-specific setup is legitimate, but rejected options should not remain in
the help surface. Removing `--write-config` is cleaner than teaching operators
that intentionally invalid flags are part of the command contract.
