//! Help for options whose values the repository's workflow defines.
//!
//! Statuses and categories come from `.atelier/workflow.yaml`, so the static
//! clap tree cannot list them. When help is shown, the command is rebuilt with
//! the loaded workflow's values as each option's possible values. The rebuilt
//! command only renders help; parsing and validation stay with the static
//! tree and the commands themselves.

use std::collections::BTreeSet;

use atelier_app::workflow_policy::{self, WorkflowPolicy};
use clap::builder::PossibleValuesParser;
use clap::error::ErrorKind;
use clap::{Command, CommandFactory};

use crate::Cli;

/// Returns help naming this repository's workflow values when `error` is a
/// help request and the workflow loads; otherwise returns `error` unchanged.
pub(crate) fn with_workflow_values(error: clap::Error) -> clap::Error {
    if error.kind() != ErrorKind::DisplayHelp {
        return error;
    }
    let Some(policy) = load_policy() else {
        return error;
    };
    match command_with_workflow_values(&policy).try_get_matches() {
        Err(help) if help.kind() == ErrorKind::DisplayHelp => help,
        _ => error,
    }
}

fn load_policy() -> Option<WorkflowPolicy> {
    let repo_root = atelier_app::storage_layout::find_repo_root().ok()?;
    workflow_policy::load(&repo_root).ok()
}

fn command_with_workflow_values(policy: &WorkflowPolicy) -> Command {
    let statuses = policy
        .statuses
        .keys()
        .cloned()
        .chain(["all".to_string()])
        .collect::<Vec<_>>();
    let categories = policy
        .statuses
        .values()
        .map(|status| status.category.clone())
        .collect::<BTreeSet<_>>();
    let creatable = policy
        .issue_types
        .keys()
        .filter_map(|issue_type| workflow_policy::creatable_statuses(policy, issue_type).ok())
        .flatten()
        .collect::<BTreeSet<_>>();
    Cli::command().mut_subcommand("issue", |issue| {
        issue
            .mut_subcommand("list", |list| {
                list.mut_arg("status", |arg| arg.value_parser(values(statuses)))
                    .mut_arg("category", |arg| arg.value_parser(values(categories)))
            })
            .mut_subcommand("create", |create| {
                create.mut_arg("status", |arg| arg.value_parser(values(creatable)))
            })
    })
}

fn values(values: impl IntoIterator<Item = String>) -> PossibleValuesParser {
    PossibleValuesParser::new(values.into_iter().map(clap::builder::PossibleValue::new))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn help(policy: &WorkflowPolicy, path: &[&str]) -> String {
        let mut command = command_with_workflow_values(policy);
        let mut command = &mut command;
        for name in path {
            command = command.find_subcommand_mut(name).expect("subcommand");
        }
        command.render_help().to_string()
    }

    #[test]
    fn help_lists_the_workflow_statuses_and_categories() {
        let policy = atelier_workflow::parse_policy_text(
            workflow_policy::STARTER_POLICY_YAML,
            workflow_policy::WORKFLOW_POLICY_PATH,
        )
        .expect("starter policy");

        let list = help(&policy, &["issue", "list"]);
        let mut statuses = policy.statuses.keys().cloned().collect::<Vec<_>>();
        statuses.push("all".into());
        assert!(
            list.contains(&format!("[possible values: {}]", statuses.join(", "))),
            "{list}"
        );
        assert!(
            list.contains("[possible values: active, blocked, done, todo]"),
            "{list}"
        );

        let create = help(&policy, &["issue", "create"]);
        let creatable = policy
            .issue_types
            .keys()
            .flat_map(|issue_type| {
                workflow_policy::creatable_statuses(&policy, issue_type).expect("issue type")
            })
            .collect::<BTreeSet<_>>();
        let status = create
            .lines()
            .find(|line| line.contains("--status <STATUS>"))
            .expect("create --status");
        assert!(
            status.ends_with(&format!(
                "[possible values: {}]",
                creatable.into_iter().collect::<Vec<_>>().join(", ")
            )),
            "{status}"
        );
    }
}
