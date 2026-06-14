## Atelier Task Management

Create issues before starting work to keep things organized and enable context handoff between sessions.

### Creating Issues
- Use `atelier issue create "title" -p <priority> --label <label> --work` for one-step create+label+work.
- Issue titles should be changelog-ready: start with a verb ("Add", "Fix", "Update"), describe the user-visible change.
- Add labels for changelog categories: `bug`/`fix` → Fixed, `feature`/`enhancement` → Added, `breaking` → Changed, `security` → Security.
- For multi-part features: create parent issue and child issues. Work one at a time.
- Add context as you discover things: `atelier issue note <id> "..."`

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
# One-step create, label, and start working
atelier issue create "Fix auth timeout" -p high --label bug --work

# Or use create with flags
atelier issue create "Add dark mode" -p medium --label feature --work

# Multi-part feature
atelier issue create "Add user auth" -p high --label feature
atelier issue create "Add registration endpoint" --parent 1
atelier issue create "Add login endpoint" --parent 1

# Track progress
atelier start <id>
atelier issue note <id> "Found existing helper in utils/"

# Close (auto-updates CHANGELOG.md)
atelier issue close <id> --reason "completed"
atelier issue close <id> --reason "completed"    # Skip changelog for internal work

# Quiet mode for scripting
atelier -q issue create "Fix bug" -p high  # Outputs just the ID number
```

### Session Management
Sessions auto-start. End them properly when you can:
```bash
atelier start <id>              # Mark current focus
atelier issue note <id> "handoff: ..." --kind handoff      # Save handoff context
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
atelier issue block 2 1     # Issue 2 blocked by issue 1
atelier issue list --ready         # Show unblocked work
```

### Large Implementations (500+ lines)
1. Create parent issue: `atelier issue create "<feature>" -p high`
2. Break into child issues: `atelier issue create "<component>" --parent <id>`
3. Work one child issue at a time, close each when done

### Context Window Management
When conversation is long or task needs many steps:
1. Create tracking issue: `atelier issue create "Continue: <summary>" -p high`
2. Add notes: `atelier issue note <id> "<what's done, what's next>"`
