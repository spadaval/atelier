use anyhow::Result;
use atelier_app::use_cases::load_repository;

use crate::commands;

pub(crate) fn dispatch(action: super::IssueCommands, quiet: bool) -> Result<()> {
    match action {
        super::IssueCommands::Create {
            title,
            body,
            priority,
            label,
            issue_type,
            parent,
            status,
            strategy,
        } => {
            let (state_dir, index) = load_repository()?;
            commands::issue::create_lifecycle(
                &index,
                &state_dir,
                commands::issue::LifecycleCreateInput {
                    title: &title,
                    body: body.as_deref(),
                    priority: &priority,
                    issue_type: issue_type.as_deref().unwrap_or("task"),
                    labels: &label,
                    parent: parent.as_deref(),
                    status: status.as_deref(),
                    strategy: strategy.as_deref(),
                    quiet,
                },
            )
        }

        super::IssueCommands::Show { ids, full } => {
            let (_, index) = load_repository()?;
            commands::issue::show(&index, &ids, full, quiet)
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
            reason_file,
            verbose,
        } => {
            let (state_dir, index) = load_repository()?;
            if let Some(transition) = transition {
                commands::workflow::transition_issue(
                    &index,
                    &state_dir,
                    &id,
                    &transition,
                    commands::workflow::CloseReasonInput {
                        reason: close_reason.as_deref(),
                        reason_file: reason_file.as_deref(),
                    },
                    quiet,
                )
            } else {
                commands::issue::transition_options(&index, &id, verbose, quiet)
            }
        }

        super::IssueCommands::Update {
            id,
            title,
            priority,
            issue_type,
            body,
            label,
            remove_label,
            parent,
            no_parent,
            strategy,
            no_strategy,
        } => {
            let (state_dir, index) = load_repository()?;
            commands::issue::update_lifecycle(
                &index,
                &state_dir,
                commands::issue::UpdateInput {
                    issue_ref: &id,
                    title: title.as_deref(),
                    priority: priority.as_deref(),
                    issue_type: issue_type.as_deref(),
                    body: body.as_deref(),
                    strategy: if no_strategy {
                        Some(None)
                    } else {
                        strategy.as_deref().map(Some)
                    },
                    labels: &label,
                    remove_labels: &remove_label,
                    parent: if no_parent {
                        Some(None)
                    } else {
                        parent.as_deref().map(Some)
                    },
                    quiet,
                },
            )
        }

        super::IssueCommands::Note { id, text, kind } => {
            let (state_dir, index) = load_repository()?;
            let id = commands::issue::resolve_id(&index, &id)?;
            commands::issue::append_note(&state_dir, &id, &kind, &text, quiet)
        }

        super::IssueCommands::Link { id, target, role } => {
            let (state_dir, index) = load_repository()?;
            commands::relate::change_issue_link(
                &index, &state_dir, &id, &target, &role, false, quiet,
            )
        }

        super::IssueCommands::Unlink { id, target, role } => {
            let (state_dir, index) = load_repository()?;
            commands::relate::change_issue_link(
                &index, &state_dir, &id, &target, &role, true, quiet,
            )
        }
    }
}
