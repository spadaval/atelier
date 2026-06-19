# Removed `atelier start`

Primary historical role: Worker.

Historical question: "How do I make this issue part of the current work in this
checkout?"

`atelier start` is removed from the normal command surface. The workflow
definition now owns issue lifecycle movement, so beginning work is the `start`
transition:

```text
atelier issue transition <issue-id> --options
atelier issue transition <issue-id> start
```

Branch preparation, tracker commits, and other mutating lifecycle behavior are
declared as transition effects in `.atelier/workflow.yaml`.
