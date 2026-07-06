# `atelier workflow`

Primary role: Admin.

Primary question: "How do I inspect raw workflow-policy diagnostics?"

## Assessment

- Name: Correct for a hidden diagnostic family.
- Documentation: Correctly hidden. Normal workflow users should use `check`,
  `issue show <objective-id>`, and `issue transition`.
- Design: Acceptable as an admin/debug surface. It should not appear in normal
  role guides.
- Output hierarchy: Raw validator or policy detail, failed configuration path,
  replacement normal commands.

## Role Use

| Form | Primary role | Operator purpose | Fit |
| --- | --- | --- | --- |
| hidden `workflow check` | Admin | Debug workflow policy evaluation. | Good only if hidden and referenced by diagnostics. |
