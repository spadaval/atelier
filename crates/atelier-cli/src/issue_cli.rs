use anyhow::{bail, Result};
use atelier_app::use_cases::load_repository;
use atelier_core::IssuePriority;

use crate::commands;

fn issue_create_parts(
    priority: &str,
    template: Option<&str>,
    description: Option<&str>,
    body: Option<&str>,
    labels: &[String],
    issue_type: Option<&str>,
) -> Result<(String, Option<String>, Option<String>, Vec<String>, String)> {
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
    let final_description = if body.is_some() {
        None
    } else {
        description.map(str::to_string).or(final_description)
    };
    let final_body = body.map(str::to_string);

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
    Ok((
        final_priority,
        final_description,
        final_body,
        labels,
        final_issue_type,
    ))
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
            let (state_dir, index) = load_repository()?;
            let (final_priority, final_description, final_body, labels, issue_type) =
                issue_create_parts(
                    &priority,
                    template.as_deref(),
                    description.as_deref(),
                    body.as_deref(),
                    &label,
                    issue_type.as_deref(),
                )?;
            commands::issue::create_lifecycle(
                &index,
                &state_dir,
                commands::issue::LifecycleCreateInput {
                    title: &title,
                    description: final_description.as_deref(),
                    body: final_body.as_deref(),
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
            let (_, index) = load_repository()?;
            commands::issue::show(&index, &id)
        }

        super::IssueCommands::List {
            status,
            category,
            issue_type,
            label,
            priority,
            limit,
        } => {
            let (_, index) = load_repository()?;
            commands::issue::list_inventory(
                &index,
                &status,
                category.as_deref(),
                issue_type.as_deref(),
                label.as_deref(),
                priority.as_deref(),
                limit,
                quiet,
            )
        }

        super::IssueCommands::Transition {
            id,
            transition,
            close_reason,
            verbose,
        } => {
            let (state_dir, index) = load_repository()?;
            if let Some(transition) = transition {
                commands::workflow::transition_issue(
                    &index,
                    &state_dir,
                    &id,
                    &transition,
                    close_reason.as_deref(),
                )
            } else {
                commands::issue::transition_options(&index, &id, verbose)
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
            let (state_dir, index) = load_repository()?;
            commands::issue::update_lifecycle(
                &index,
                &state_dir,
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
                },
            )
        }

        super::IssueCommands::Note { id, text, kind } => {
            let (state_dir, index) = load_repository()?;
            let id = super::resolve_issue_arg(&index, &id)?;
            commands::issue::append_note(&state_dir, &id, &kind, &text)
        }

        super::IssueCommands::Link { id, target, role } => {
            let (state_dir, index) = load_repository()?;
            commands::relate::link_issue(&index, &state_dir, &id, &target, &role)
        }

        super::IssueCommands::Unlink { id, target, role } => {
            let (state_dir, index) = load_repository()?;
            commands::relate::unlink_issue(&index, &state_dir, &id, &target, &role)
        }
    }
}
