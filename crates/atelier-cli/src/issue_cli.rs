use anyhow::{bail, Result};
use atelier_app::command_storage::{
    canonical_mutation_db, degraded_projection_query_db, state_and_db_paths,
};
use atelier_core::IssuePriority;

use crate::commands;

fn issue_create_parts(
    priority: &str,
    template: Option<&str>,
    description: Option<&str>,
    body: Option<&str>,
    labels: &[String],
    issue_type: Option<&str>,
) -> Result<(String, Option<String>, Vec<String>, String)> {
    if description.is_some() && body.is_some() {
        bail!("--description and --body cannot be combined");
    }
    let mut labels = labels.to_vec();
    let (final_priority, final_description, template_issue_type) =
        if let Some(template_name) = template {
            let template = commands::create::get_template(template_name).ok_or_else(|| {
                anyhow::anyhow!(
                    "Unknown template '{}'. Available: {}",
                    template_name,
                    commands::create::list_templates().join(", ")
                )
            })?;
            if !labels.iter().any(|label| label == template.label) {
                labels.push(template.label.to_string());
            }
            let priority = if priority != "medium" {
                priority
            } else {
                template.priority
            };
            (
                priority.to_string(),
                template.description_prefix.map(str::to_string),
                Some(template_default_issue_type(template_name)),
            )
        } else {
            (priority.to_string(), None, None)
        };
    let final_description = description
        .or(body)
        .map(str::to_string)
        .or(final_description);

    IssuePriority::from_cli_input(&final_priority)?;
    let final_issue_type = match (issue_type, template_issue_type) {
        (Some(explicit), Some(default)) if explicit != default => {
            bail!(
                "Conflicting work type options: --issue-type {explicit} does not match --template {} (default type {default}). Choose one work type or use a matching template.",
                template.unwrap_or("(none)")
            );
        }
        (Some(explicit), _) => explicit.to_string(),
        (None, Some(default)) => default.to_string(),
        (None, None) => "task".to_string(),
    };
    Ok((final_priority, final_description, labels, final_issue_type))
}

fn template_default_issue_type(template: &str) -> &'static str {
    match template {
        "bug" => "bug",
        "feature" => "feature",
        "research" | "investigation" => "spike",
        "audit" => "validation",
        _ => "task",
    }
}

pub(crate) fn dispatch(action: super::IssueCommands, quiet: bool) -> Result<()> {
    match action {
        super::IssueCommands::Create {
            title,
            description,
            body,
            constraint,
            risk,
            validation,
            priority,
            template,
            label,
            issue_type,
            parent,
        } => {
            let (state_dir, db_path) = state_and_db_paths()?;
            let (final_priority, final_description, labels, issue_type) = issue_create_parts(
                &priority,
                template.as_deref(),
                description.as_deref(),
                body.as_deref(),
                &label,
                issue_type.as_deref(),
            )?;
            commands::issue::create_lifecycle(
                &state_dir,
                &db_path,
                commands::issue::LifecycleCreateInput {
                    title: &title,
                    description: final_description.as_deref(),
                    priority: &final_priority,
                    issue_type: &issue_type,
                    labels: &labels,
                    parent: parent.as_deref(),
                    constraints: constraint,
                    risks: risk,
                    validation,
                    quiet,
                },
            )
        }

        super::IssueCommands::Show { id } => {
            let db = degraded_projection_query_db()?;
            commands::issue::show(&db, &id)
        }

        super::IssueCommands::Transition {
            id,
            transition,
            close_reason,
        } => {
            if let Some(transition) = transition {
                let (state_dir, db_path) = state_and_db_paths()?;
                let db = canonical_mutation_db()?;
                commands::workflow::transition_issue(
                    &db,
                    &state_dir,
                    &db_path,
                    &id,
                    &transition,
                    close_reason.as_deref(),
                )
            } else {
                let db = degraded_projection_query_db()?;
                commands::issue::transition_options(&db, &id)
            }
        }

        super::IssueCommands::Update {
            id,
            title,
            priority,
            issue_type,
            status,
            body,
            constraint,
            risk,
            validation,
            label,
            remove_label,
            parent,
            no_parent,
        } => {
            let (state_dir, db_path) = state_and_db_paths()?;
            if status.is_some() {
                bail!(
                    "issue status changes use `atelier issue transition <issue-id> <transition>`"
                );
            }
            if body.is_some()
                || !constraint.is_empty()
                || !risk.is_empty()
                || !validation.is_empty()
            {
                bail!("mission section flags are not supported for issue records");
            }
            commands::issue::update_lifecycle(
                &state_dir,
                &db_path,
                commands::issue::UpdateInput {
                    issue_ref: &id,
                    title: title.as_deref(),
                    priority: priority.as_deref(),
                    issue_type: issue_type.as_deref(),
                    labels: &label,
                    remove_labels: &remove_label,
                    parent: if no_parent {
                        Some(None)
                    } else {
                        parent.as_deref().map(Some)
                    },
                    append_notes: None,
                },
            )
        }

        super::IssueCommands::Note { id, text, kind } => {
            let db = canonical_mutation_db()?;
            let id = super::resolve_issue_arg(&db, &id)?;
            commands::comment::run_issue_note(&db, &id, &text, &kind)
        }

        super::IssueCommands::Link { id, target, role } => {
            let (state_dir, db_path) = state_and_db_paths()?;
            commands::relate::link_issue(&state_dir, &db_path, &id, &target, &role)
        }

        super::IssueCommands::Unlink { id, target, role } => {
            let (state_dir, db_path) = state_and_db_paths()?;
            commands::relate::unlink_issue(&state_dir, &db_path, &id, &target, &role)
        }
    }
}
