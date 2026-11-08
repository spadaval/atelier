# Sizing

Size work, whether a planned item or a finished change, against the outcome it
serves, on three scales:

- **Complexity:** *low*, held in one sitting, no new moving part; *medium*, a
  new moving part or several owners; *high*, new state, a subsystem or a
  contract others must keep, or too big to review in one pass.
- **Value:** *low*, a rare case, a hypothetical or a path planned for removal;
  *medium*, a path in current use; *high*, the outcome needs it (a Done when
  line, a regression, a current hazard).
- **Risk** of merging it: *low*, a failure shows and is cheap to undo;
  *medium*, it reaches users or other repositories but shows and can be undone;
  *high*, silent wrong results, lost data or a security hole.
