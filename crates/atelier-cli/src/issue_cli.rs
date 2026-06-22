use anyhow::{bail, Result};
use atelier_app::command_storage::{
    canonical_mutation_db, command_storage, degraded_projection_query_db, projection_query_db,
    state_and_db_paths, CommandStorageAccess,
};
use atelier_core::IssuePriority;
use atelier_records::RecordStore;

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

        super::IssueCommands::List {
            status,
            category,
            label,
            priority,
            ready,
            blocked,
        } => {
            let db = degraded_projection_query_db()?;
            if blocked {
                if ready {
                    bail!("--blocked cannot be combined with --ready");
                }
                if status != "todo" || category.is_some() || label.is_some() || priority.is_some() {
                    bail!("--blocked cannot be combined with --status, --category, --label, or --priority");
                }
                commands::deps::list_blocked(&db, quiet)
            } else {
                commands::issue::list(
                    &db,
                    Some(&status),
                    category.as_deref(),
                    label.as_deref(),
                    priority.as_deref(),
                    ready,
                    quiet,
                )
            }
        }

        super::IssueCommands::Table {
            kind,
            status,
            issue_type,
        } => {
            let db = degraded_projection_query_db()?;
            commands::issue::table(&db, &kind, &status, issue_type.as_deref(), quiet)
        }

        super::IssueCommands::Show { id } => {
            let db = degraded_projection_query_db()?;
            commands::issue::show(&db, &id)
        }

        super::IssueCommands::Status { id, verbose } => {
            let storage = command_storage(CommandStorageAccess::DegradedProjectionQuery)?;
            let db = storage.db();
            if is_mission_objective(db, &id)? {
                commands::mission::status(db, &storage.state_dir(), Some(&id), quiet, verbose)
            } else {
                if verbose {
                    bail!("--verbose is only available for mission objective records");
                }
                commands::issue_status::run(db, &id, quiet)
            }
        }

        super::IssueCommands::Transition {
            id,
            transition,
            options,
            close_reason,
        } => {
            if options {
                if transition.is_some() {
                    bail!("--options cannot be combined with a transition name");
                }
                let db = degraded_projection_query_db()?;
                commands::issue::transition_options(&db, &id)
            } else {
                let transition = transition.ok_or_else(|| {
                    anyhow::anyhow!(
                        "Specify a transition name or rerun with `atelier issue transition {} --options`",
                        id
                    )
                })?;
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

        super::IssueCommands::Block { id, blocker } => {
            let db = canonical_mutation_db()?;
            let (state_dir, db_path) = state_and_db_paths()?;
            let store = RecordStore::new(&state_dir);
            commands::issue::dep_add_canonical(&db, &store, &id, &blocker)?;
            drop(db);
            atelier_app::projection::refresh_after_canonical_write(&state_dir, &db_path)
        }

        super::IssueCommands::Unblock { id, blocker } => {
            let db = canonical_mutation_db()?;
            let (state_dir, db_path) = state_and_db_paths()?;
            let store = RecordStore::new(&state_dir);
            commands::issue::dep_remove_canonical(&db, &store, &id, &blocker)?;
            drop(db);
            atelier_app::projection::refresh_after_canonical_write(&state_dir, &db_path)
        }

        super::IssueCommands::Blocked { id } => {
            let db = projection_query_db()?;
            if let Some(id) = id {
                commands::issue::dep_list(&db, Some(&id))
            } else {
                commands::deps::list_blocked(&db, quiet)
            }
        }
    }
}

fn is_mission_objective(db: &atelier_sqlite::Database, id: &str) -> Result<bool> {
    Ok(db
        .get_issue(id)?
        .is_some_and(|issue| issue.issue_type == "mission"))
}
