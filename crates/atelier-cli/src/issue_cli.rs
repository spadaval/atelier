use anyhow::Result;
use atelier_app::use_cases::load_repository;

use crate::commands;
use atelier::commands::record_commit::RecordCommit;

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
            discovered_from,
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
                    discovered_from: discovered_from.as_deref(),
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
            record_commit,
        } => {
            let (state_dir, index) = load_repository()?;
            if let Some(transition) = transition {
                let group = RecordCommit::begin(record_commit)?;
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
                )?;
                group.finish(&transition, quiet)
            } else {
                anyhow::ensure!(
                    !record_commit.commit_records && !record_commit.stage_records,
                    "--commit-records and --stage-records need a transition to run"
                );
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
            adr,
            no_adr,
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
                    adr: if no_adr {
                        Some(None)
                    } else {
                        adr.as_deref().map(Some)
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

        super::IssueCommands::Note {
            id,
            text,
            kind,
            reason,
            rejected,
            one_way,
        } => {
            let (state_dir, index) = load_repository()?;
            let id = commands::issue::resolve_id(&index, &id)?;
            let decision = commands::issue::DecisionFlags {
                reason: reason.as_deref(),
                rejected: &rejected,
                one_way,
            };
            commands::issue::append_note(&state_dir, &id, &kind, &text, decision, quiet)
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
