use std::collections::BTreeMap;
use std::fs;
use std::io::Read;
use std::path::Path;

use anyhow::{bail, Context, Result};
use atelier_app::forgejo::{ForgejoClient, ForgejoTransport, UreqForgejoTransport};
use atelier_app::project_config::{
    load_forgejo_admin_token, load_forgejo_with_workflow_role_authors, ForgejoConfig, FORGEJO_ROLES,
};

const ROLE_PERMISSION: &str = "write";

pub fn roles_check(repo_root: &Path) -> Result<()> {
    let forgejo = load_forgejo_with_workflow_role_authors(repo_root)?;
    let token = load_forgejo_admin_token()?;
    let client = ForgejoClient::new(
        forgejo.clone(),
        UreqForgejoTransport::new(&forgejo.host, token),
    );
    let report = inspect_roles(&client, &forgejo)?;
    print_report("Forgejo Role Authors", &report);
    ensure_report_passes(&report)?;
    Ok(())
}

pub fn roles_provision(repo_root: &Path) -> Result<()> {
    let forgejo = load_forgejo_with_workflow_role_authors(repo_root)?;
    let token = load_forgejo_admin_token()?;
    let client = ForgejoClient::new(
        forgejo.clone(),
        UreqForgejoTransport::new(&forgejo.host, token),
    );

    for role in FORGEJO_ROLES {
        let username = forgejo.role_author_for_role(role)?;
        if !client.user_exists(username)? {
            client.create_user(
                username,
                &format!("{username}@localhost.invalid"),
                &role_full_name(role),
                &random_password()?,
            )?;
        }
        client.add_collaborator(username, ROLE_PERMISSION)?;
    }

    let report = inspect_roles(&client, &forgejo)?;
    print_report("Forgejo Role Provisioning", &report);
    ensure_report_passes(&report)?;

    println!();
    println!("Config:  role authors sourced from .atelier/workflow.yaml actions");
    Ok(())
}

#[derive(Debug)]
struct RoleReport {
    rows: Vec<RoleRow>,
    collapsed: Vec<String>,
}

#[derive(Debug)]
struct RoleRow {
    role: String,
    username: String,
    user_exists: bool,
    permission: Option<String>,
    sudo_ok: bool,
}

fn inspect_roles<T: ForgejoTransport>(
    client: &ForgejoClient<T>,
    forgejo: &ForgejoConfig,
) -> Result<RoleReport> {
    let mut rows = Vec::new();
    let mut by_user: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for role in FORGEJO_ROLES {
        let username = forgejo.role_author_for_role(role)?;
        by_user
            .entry(username.to_string())
            .or_default()
            .push((*role).to_string());
        let user_exists = client.user_exists(username)?;
        let permission = if user_exists {
            client.collaborator_permission(username)?
        } else {
            None
        };
        let sudo_ok = user_exists && client.verify_sudo_user(username)?;
        rows.push(RoleRow {
            role: (*role).to_string(),
            username: username.to_string(),
            user_exists,
            permission,
            sudo_ok,
        });
    }
    let collapsed = by_user
        .into_iter()
        .filter_map(|(user, roles)| {
            if roles.len() > 1 {
                Some(format!("{} -> {}", user, roles.join(", ")))
            } else {
                None
            }
        })
        .collect();
    Ok(RoleReport { rows, collapsed })
}

fn print_report(title: &str, report: &RoleReport) {
    println!("{title}");
    println!("{}", "=".repeat(title.len()));
    for row in &report.rows {
        let permission = row.permission.as_deref().unwrap_or("none");
        println!(
            "{:<9} {:<24} user={} permission={} sudo={}",
            row.role,
            row.username,
            pass_fail(row.user_exists),
            permission,
            pass_fail(row.sudo_ok)
        );
    }
    if report.collapsed.is_empty() {
        println!("Collapsed: none");
    } else {
        println!("Collapsed: {}", report.collapsed.join("; "));
    }
}

fn pass_fail(passed: bool) -> &'static str {
    if passed {
        "pass"
    } else {
        "fail"
    }
}

fn ensure_report_passes(report: &RoleReport) -> Result<()> {
    let mut failures = Vec::new();
    if !report.collapsed.is_empty() {
        failures.push(format!(
            "collapsed role authors: {}",
            report.collapsed.join("; ")
        ));
    }
    for row in &report.rows {
        if !row.user_exists {
            failures.push(format!("{} user {} does not exist", row.role, row.username));
        }
        if !matches!(row.permission.as_deref(), Some("write" | "admin")) {
            failures.push(format!(
                "{} user {} does not have write permission",
                row.role, row.username
            ));
        }
        if !row.sudo_ok {
            failures.push(format!(
                "{} user {} failed sudo verification",
                row.role, row.username
            ));
        }
    }
    if failures.is_empty() {
        Ok(())
    } else {
        bail!("forgejo_role_authors_invalid: {}", failures.join("; "))
    }
}

fn role_full_name(role: &str) -> String {
    format!("Atelier {}", role_name(role))
}

fn role_name(role: &str) -> String {
    let mut chars = role.chars();
    match chars.next() {
        Some(first) => first.to_ascii_uppercase().to_string() + chars.as_str(),
        None => "Role".to_string(),
    }
}

fn random_password() -> Result<String> {
    let mut bytes = [0u8; 32];
    fs::File::open("/dev/urandom")
        .and_then(|mut file| file.read_exact(&mut bytes))
        .context("failed to read random bytes for Forgejo role password")?;
    Ok(bytes.iter().map(|byte| format!("{byte:02x}")).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;
    use atelier_app::forgejo::{ForgejoRequest, ForgejoResponse};
    use atelier_app::project_config::ForgejoRoleAuthors;
    use std::cell::RefCell;

    #[derive(Debug)]
    struct MockTransport {
        requests: RefCell<Vec<ForgejoRequest>>,
        responses: RefCell<Vec<ForgejoResponse>>,
    }

    impl MockTransport {
        fn new(responses: Vec<ForgejoResponse>) -> Self {
            Self {
                requests: RefCell::new(Vec::new()),
                responses: RefCell::new(responses.into_iter().rev().collect()),
            }
        }
    }

    impl ForgejoTransport for &MockTransport {
        fn send(&self, request: ForgejoRequest) -> Result<ForgejoResponse> {
            self.requests.borrow_mut().push(request);
            self.responses
                .borrow_mut()
                .pop()
                .ok_or_else(|| anyhow!("missing mock response"))
        }
    }

    fn forgejo_config() -> ForgejoConfig {
        ForgejoConfig {
            host: "forge.example.test".to_string(),
            owner: "tools".to_string(),
            repo: "atelier".to_string(),
            role_authors: Some(ForgejoRoleAuthors {
                worker: "atelier-worker".to_string(),
                reviewer: "atelier-reviewer".to_string(),
                validator: "atelier-validator".to_string(),
                manager: "atelier-manager".to_string(),
            }),
        }
    }

    #[test]
    fn inspect_roles_reports_success_and_collapsed_mappings() {
        let transport = MockTransport::new(vec![
            ForgejoResponse {
                status: 200,
                body: "{}".to_string(),
            },
            ForgejoResponse {
                status: 200,
                body: r#"{"permission":"write"}"#.to_string(),
            },
            ForgejoResponse {
                status: 200,
                body: r#"{"login":"atelier-worker"}"#.to_string(),
            },
            ForgejoResponse {
                status: 200,
                body: "{}".to_string(),
            },
            ForgejoResponse {
                status: 200,
                body: r#"{"permission":"write"}"#.to_string(),
            },
            ForgejoResponse {
                status: 200,
                body: r#"{"login":"atelier-reviewer"}"#.to_string(),
            },
            ForgejoResponse {
                status: 200,
                body: "{}".to_string(),
            },
            ForgejoResponse {
                status: 200,
                body: r#"{"permission":"write"}"#.to_string(),
            },
            ForgejoResponse {
                status: 200,
                body: r#"{"login":"atelier-validator"}"#.to_string(),
            },
            ForgejoResponse {
                status: 200,
                body: "{}".to_string(),
            },
            ForgejoResponse {
                status: 200,
                body: r#"{"permission":"write"}"#.to_string(),
            },
            ForgejoResponse {
                status: 200,
                body: r#"{"login":"atelier-manager"}"#.to_string(),
            },
        ]);
        let client = ForgejoClient::new(forgejo_config(), &transport);

        let report = inspect_roles(&client, &forgejo_config()).unwrap();

        assert!(report.collapsed.is_empty());
        ensure_report_passes(&report).unwrap();
    }
}
