use anyhow::{bail, Result};
use atelier_app::cache_manager::state_and_db_paths;
use atelier_app::use_cases;
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
            let cache = use_cases::mutation_cache()?;
            drop(cache);
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
            let cache = use_cases::issue_detail_cache()?;
            commands::issue::show(cache.db(), &id)
        }

        super::IssueCommands::List {
            status,
            category,
            issue_type,
            label,
            priority,
            limit,
        } => {
            let cache = use_cases::issue_query_cache()?;
            commands::issue::list_inventory(
                cache.db(),
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
            if let Some(transition) = transition {
                let (state_dir, db_path) = state_and_db_paths()?;
                let cache = use_cases::mutation_cache()?;
                commands::workflow::transition_issue(
                    cache.db(),
                    &state_dir,
                    &db_path,
                    &id,
                    &transition,
                    close_reason.as_deref(),
                )
            } else {
                let cache = use_cases::issue_query_cache()?;
                commands::issue::transition_options(cache.db(), &id, verbose)
            }
        }

        super::IssueCommands::PlanReview { id, action } => {
            let actor = std::env::var("ATELIER_AUTHENTICATED_ACTOR").map_err(|_| {
                anyhow::anyhow!(
                    "mission-plan review requires ATELIER_AUTHENTICATED_ACTOR=actor-v1:<authenticated-authority>/<immutable-subject>"
                )
            })?;
            let cache = use_cases::mutation_cache()?;
            let mission_id = super::resolve_issue_arg(cache.db(), &id)?;
            let (state_dir, db_path) = state_and_db_paths()?;
            let mission_status = cache.db().require_issue(&mission_id)?.status;
            let request_transition =
                matches!(&action, super::PlanReviewCommands::Request) && mission_status == "draft";
            if request_transition {
                let options =
                    commands::workflow_planning::issue_transition_options(cache.db(), &mission_id)?;
                let option = options
                    .iter()
                    .find(|option| option.name == "request_plan_review")
                    .ok_or_else(|| {
                        anyhow::anyhow!(
                            "mission {} has no configured request_plan_review transition",
                            mission_id
                        )
                    })?;
                if !option.allowed {
                    bail!(
                        "mission {} cannot request plan review: {}",
                        mission_id,
                        option.blockers.join("; ")
                    );
                }
            }
            let mutation = match action {
                super::PlanReviewCommands::Request => {
                    if mission_status == "plan_review" {
                        atelier_app::mission_plan_review::MissionPlanReviewMutation::Rework
                    } else {
                        atelier_app::mission_plan_review::MissionPlanReviewMutation::Request
                    }
                }
                super::PlanReviewCommands::Rework => {
                    atelier_app::mission_plan_review::MissionPlanReviewMutation::Rework
                }
                super::PlanReviewCommands::Finding {
                    finding_id,
                    severity,
                    affected_issue_ids,
                    dependency_path,
                } => {
                    let severity = match severity.as_str() {
                        "blocking" => atelier_records::mission_plan_review::MissionPlanFindingSeverity::Blocking,
                        "non_blocking" | "non-blocking" => atelier_records::mission_plan_review::MissionPlanFindingSeverity::NonBlocking,
                        other => bail!("unsupported finding severity '{other}'; expected blocking or non-blocking"),
                    };
                    atelier_app::mission_plan_review::MissionPlanReviewMutation::Finding {
                        finding_id,
                        severity,
                        affected_issue_ids,
                        dependency_path,
                    }
                }
                super::PlanReviewCommands::ChangeRequest {
                    request_id,
                    affected_issue_ids,
                    dependency_path,
                } => atelier_app::mission_plan_review::MissionPlanReviewMutation::ChangeRequest {
                    request_id,
                    affected_issue_ids,
                    dependency_path,
                },
                super::PlanReviewCommands::Resolve {
                    target_id,
                    disposition,
                } => atelier_app::mission_plan_review::MissionPlanReviewMutation::Resolve {
                    target_id,
                    disposition,
                },
                super::PlanReviewCommands::Approve => {
                    atelier_app::mission_plan_review::MissionPlanReviewMutation::Approve
                }
            };
            let result = atelier_app::mission_plan_review::mutate(
                &state_dir,
                &mission_id,
                &actor,
                mutation,
            )?;
            if request_transition {
                commands::workflow::transition_issue(
                    cache.db(),
                    &state_dir,
                    &db_path,
                    &mission_id,
                    "request_plan_review",
                    None,
                )?;
            }
            if quiet {
                println!("{}", result.activity_id);
            } else {
                println!("Recorded mission-plan review event {}", result.activity_id);
                println!("Mission:  {mission_id}");
                println!("Revision: {}", result.graph_revision);
            }
            Ok(())
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
            let cache = use_cases::mutation_cache()?;
            drop(cache);
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
            let cache = use_cases::mutation_cache()?;
            let id = super::resolve_issue_arg(cache.db(), &id)?;
            commands::comment::run_issue_note(cache.db(), &id, &text, &kind)
        }

        super::IssueCommands::Link { id, target, role } => {
            let cache = use_cases::mutation_cache()?;
            drop(cache);
            let (state_dir, db_path) = state_and_db_paths()?;
            commands::relate::link_issue(&state_dir, &db_path, &id, &target, &role)
        }

        super::IssueCommands::Unlink { id, target, role } => {
            let cache = use_cases::mutation_cache()?;
            drop(cache);
            let (state_dir, db_path) = state_and_db_paths()?;
            commands::relate::unlink_issue(&state_dir, &db_path, &id, &target, &role)
        }
    }
}
