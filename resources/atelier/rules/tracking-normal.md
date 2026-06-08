## Atelier Task Management

Create issues before starting work to keep things organized and enable context handoff between sessions.

### Creating Issues
- Use `atelier quick "title" -p <priority> -l <label>` for one-step create+label+work.
- Issue titles should be changelog-ready: start with a verb ("Add", "Fix", "Update"), describe the user-visible change.
- Add labels for changelog categories: `bug`/`fix` → Fixed, `feature`/`enhancement` → Added, `breaking` → Changed, `security` → Security.
- For multi-part features: create parent issue + subissues. Work one at a time.
- Add context as you discover things: `atelier comment <id> "..."`

### Labels for Changelog Categories
- `bug`, `fix` → **Fixed**
- `feature`, `enhancement` → **Added**
- `breaking`, `breaking-change` → **Changed**
- `security` → **Security**
- `deprecated` → **Deprecated**
- `removed` → **Removed**
- (no label) → **Changed** (default)

### Quick Reference
```bash
# One-step create + label + start working
atelier quick "Fix auth timeout" -p high -l bug

# Or use create with flags
atelier create "Add dark mode" -p medium --label feature --work

# Multi-part feature
atelier create "Add user auth" -p high --label feature
atelier subissue 1 "Add registration endpoint"
atelier subissue 1 "Add login endpoint"

# Track progress
atelier session work <id>
atelier comment <id> "Found existing helper in utils/"

# Close (auto-updates CHANGELOG.md)
atelier close <id>
atelier close <id> --no-changelog    # Skip changelog for internal work
atelier close-all --no-changelog     # Batch close

# Quiet mode for scripting
atelier -q create "Fix bug" -p high  # Outputs just the ID number
```

### Session Management
Sessions auto-start. End them properly when you can:
```bash
atelier session work <id>              # Mark current focus
atelier session end --notes "..."      # Save handoff context
```

End sessions when: context is getting long, user indicates stopping, or you've completed significant work.

Handoff notes should include: what was accomplished, what's in progress, what's next.

### Priority Guide
- `critical`: Blocking other work, security issue, production down
- `high`: User explicitly requested, core functionality
- `medium`: Standard features, improvements
- `low`: Nice-to-have, cleanup, optimization

### Dependencies
```bash
atelier block 2 1     # Issue 2 blocked by issue 1
atelier ready         # Show unblocked work
```

### Large Implementations (500+ lines)
1. Create parent issue: `atelier create "<feature>" -p high`
2. Break into subissues: `atelier subissue <id> "<component>"`
3. Work one subissue at a time, close each when done

### Context Window Management
When conversation is long or task needs many steps:
1. Create tracking issue: `atelier create "Continue: <summary>" -p high`
2. Add notes: `atelier comment <id> "<what's done, what's next>"`
