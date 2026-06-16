//! Workflow policy boundaries for Atelier.
//!
//! Policy parsing and transition evaluation move here from the root crate during
//! the crate migration.

pub use atelier_core::RecordId;

pub const STARTER_POLICY_YAML: &str = r#"schema: atelier.workflow
schema_version: 1

branch_lifecycle:
  base_branch: main
  merge_strategy: squash
  branch_templates:
    epic: epic/{{ issue.id }}
    issue: codex/{{ issue.id }}

issue_types:
  bug: standard_proof
  epic: standard_review_proof
  feature: standard_proof
  spike: lightweight_spike
  task: standard_proof
  validation: standard_review_proof

statuses:
  todo:
    category: todo
  in_progress:
    category: active
  blocked:
    category: blocked
  review:
    category: review
  validation:
    category: validation
  done:
    category: done
  archived:
    category: done

validators:
  durable_current:
    builtin: durable_state_current
  review_ready:
    builtin: review_complete
  proof_attached:
    builtin: evidence_attached
    params:
      min_count: 1
  blockers_clear:
    builtin: no_open_blockers
  epic_child_proof:
    builtin: epic_child_proof_complete
  lint_clear:
    builtin: no_blocking_lints
  closeout_clean:
    builtin: git_worktree_clean

guidance_templates:
  close_with_proof:
    format: markdown
    template: |
      Closing {{ issue.id }} requires attached evidence and no open blockers.
  record_spike_outcome:
    format: markdown
    template: |
      Record a concise close reason that captures what {{ issue.id }} learned
      and what follow-up work remains.

workflows:
  standard_proof:
    initial_status: todo
    done_statuses: [done, archived]
    transitions:
      start:
        from: [todo, blocked]
        to: in_progress
      block:
        from: [todo, in_progress, validation]
        to: blocked
      close:
        from: [in_progress, validation]
        to: done
        required_fields: [close_reason]
        validators:
          - proof_attached
          - blockers_clear
          - lint_clear
          - durable_current
        guidance: [close_with_proof]

  standard_review_proof:
    initial_status: todo
    done_statuses: [done, archived]
    transitions:
      start:
        from: [todo, blocked]
        to: in_progress
      block:
        from: [todo, in_progress, review, validation]
        to: blocked
      request_review:
        from: [in_progress]
        to: review
      request_validation:
        from: [in_progress, review]
        to: validation
        validators: [review_ready]
      close:
        from: [validation]
        to: done
        required_fields: [close_reason]
        validators:
          - proof_attached
          - epic_child_proof
          - blockers_clear
          - lint_clear
          - durable_current
          - closeout_clean
        guidance: [close_with_proof]

  lightweight_spike:
    initial_status: todo
    done_statuses: [done]
    transitions:
      start:
        from: [todo, blocked]
        to: in_progress
      block:
        from: [todo, in_progress, review]
        to: blocked
      request_review:
        from: [in_progress]
        to: review
      revise:
        from: [review]
        to: in_progress
      close:
        from: [review]
        to: done
        required_fields: [close_reason]
        validators:
          - review_ready
          - durable_current
        guidance: [record_spike_outcome]
"#;

/// Minimal transition descriptor used by early extraction tests.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct TransitionName(String);

impl TransitionName {
    pub fn new(value: impl Into<String>) -> Option<Self> {
        let value = value.into();
        (!value.trim().is_empty()).then_some(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Minimal workflow status category vocabulary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StatusCategory {
    Todo,
    Active,
    Done,
    Blocked,
}

pub fn status_category(status: &str) -> StatusCategory {
    match status {
        "done" | "archived" => StatusCategory::Done,
        "blocked" => StatusCategory::Blocked,
        "in_progress" | "review" | "validation" => StatusCategory::Active,
        _ => StatusCategory::Todo,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transition_name_rejects_empty_values() {
        assert_eq!(TransitionName::new(" "), None);
    }

    #[test]
    fn transition_name_keeps_text() {
        assert_eq!(TransitionName::new("start").unwrap().as_str(), "start");
    }

    #[test]
    fn status_categories_match_workflow_groups() {
        assert_eq!(status_category("todo"), StatusCategory::Todo);
        assert_eq!(status_category("in_progress"), StatusCategory::Active);
        assert_eq!(status_category("validation"), StatusCategory::Active);
        assert_eq!(status_category("blocked"), StatusCategory::Blocked);
        assert_eq!(status_category("done"), StatusCategory::Done);
    }

    #[test]
    fn starter_policy_lives_in_workflow_crate() {
        assert!(STARTER_POLICY_YAML.contains("schema: atelier.workflow"));
        assert!(STARTER_POLICY_YAML.contains("standard_review_proof"));
    }
}
