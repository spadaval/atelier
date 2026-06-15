# `atelier import-beads`

Primary role: Admin.

Primary question: "How do I import predecessor Beads data from an explicit
backup file?"

## Assessment

- Name: Correct for migration, but predecessor-specific.
- Documentation: Correctly hidden. Normal setup should use `init --import-beads`
  for the standard repo-local import path.
- Design: Acceptable as a one-off migration escape hatch.
- Output hierarchy: Input path, imported record counts, canonical output path,
  validation commands.

## Role Use

| Form | Primary role | Operator purpose | Fit |
| --- | --- | --- | --- |
| hidden `atelier import-beads <input>` | Admin | Import external predecessor backup. | Good as hidden migration-only surface. |
