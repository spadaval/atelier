use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::io::Read;
use std::path::Path;

use anyhow::{bail, Context, Result};
use atelier_app::forgejo::{ForgejoClient, ForgejoTransport, UreqForgejoTransport};
use atelier_app::project_config::{
    load_forgejo_with_default_role_authors, ForgejoConfig, ForgejoRoleAuthors, ProjectConfig,
    FORGEJO_ROLES,
};
use atelier_app::storage_layout::StorageLayout;

const ROLE_PERMISSION: &str = "write";

pub fn roles_check(repo_root: &Path) -> Result<()> {
    let config_path = repo_root.join(".atelier/config.toml");
    let config = ProjectConfig::load(repo_root)?;
    let forgejo = config.require_forgejo(&config_path)?.clone();
    let token = env::var(&forgejo.admin_token_env).with_context(|| {
        format!(
            "forgejo_config_missing_token: environment variable {} is required for `atelier forgejo roles check`",
            forgejo.admin_token_env
        )
    })?;
    let client = ForgejoClient::new(
        forgejo.clone(),
        UreqForgejoTransport::new(&forgejo.host, token),
    );
    let report = inspect_roles(&client, &forgejo)?;
    print_report("Forgejo Role Authors", &report);
    ensure_report_passes(&report)?;
    Ok(())
}

pub fn roles_provision(repo_root: &Path, write_config: bool) -> Result<()> {
    let default_authors = default_role_authors();
    let forgejo = load_forgejo_for_provisioning(repo_root, default_authors)?;
    let token = env::var(&forgejo.admin_token_env).with_context(|| {
        format!(
            "forgejo_config_missing_token: environment variable {} is required for `atelier forgejo roles provision`",
            forgejo.admin_token_env
        )
    })?;
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

    let config_block = role_authors_config_block(&forgejo)?;
    if write_config {
        write_role_authors_config(repo_root, &config_block)?;
        println!("Config:  updated .atelier/config.toml");
    } else {
        println!();
        println!("Config block to apply:");
        print!("{config_block}");
    }
    Ok(())
}

fn load_forgejo_for_provisioning(
    repo_root: &Path,
    default_authors: ForgejoRoleAuthors,
) -> Result<ForgejoConfig> {
    let config_path = repo_root.join(".atelier/config.toml");
    match ProjectConfig::load(repo_root) {
        Ok(config) => Ok(config.require_forgejo(&config_path)?.clone()),
        Err(_) => load_forgejo_with_default_role_authors(repo_root, default_authors),
    }
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

fn default_role_authors() -> ForgejoRoleAuthors {
    ForgejoRoleAuthors {
        worker: "atelier-worker".to_string(),
        reviewer: "atelier-reviewer".to_string(),
        validator: "atelier-validator".to_string(),
        manager: "atelier-manager".to_string(),
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

fn role_authors_config_block(forgejo: &ForgejoConfig) -> Result<String> {
    let role_authors = forgejo.role_authors.as_ref().ok_or_else(|| {
        anyhow::anyhow!(
            "forgejo_config_missing_role_authors: Forgejo role authors are required for provisioning"
        )
    })?;
    Ok(format!(
        "[review.providers.forgejo.role_authors]\nworker = \"{}\"\nreviewer = \"{}\"\nvalidator = \"{}\"\nmanager = \"{}\"\n",
        role_authors.worker,
        role_authors.reviewer,
        role_authors.validator,
        role_authors.manager
    ))
}

fn write_role_authors_config(repo_root: &Path, block: &str) -> Result<()> {
    let config_path = StorageLayout::new(repo_root).config_path();
    let text = fs::read_to_string(&config_path)
        .with_context(|| format!("failed to read {}", config_path.display()))?;
    let mut output = String::new();
    let mut skipping_role_table = false;
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed == "[forgejo.role_authors]"
            || trimmed == "[review.providers.forgejo.role_authors]"
        {
            skipping_role_table = true;
            continue;
        }
        if skipping_role_table && trimmed.starts_with('[') {
            skipping_role_table = false;
        }
        if !skipping_role_table {
            output.push_str(line);
            output.push('\n');
        }
    }
    while output.ends_with("\n\n") {
        output.pop();
    }
    if !output.ends_with('\n') {
        output.push('\n');
    }
    output.push('\n');
    output.push_str(block);
    fs::write(&config_path, output)
        .with_context(|| format!("failed to write {}", config_path.display()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;
    use atelier_app::forgejo::{ForgejoRequest, ForgejoResponse};
    use std::cell::RefCell;
    use tempfile::tempdir;

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
            admin_token_env: "FORGEJO_ADMIN_TOKEN".to_string(),
            role_authors: Some(default_role_authors()),
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

    #[test]
    fn write_config_replaces_role_authors_table() {
        let dir = tempdir().unwrap();
        fs::create_dir_all(dir.path().join(".atelier")).unwrap();
        fs::write(
            dir.path().join(".atelier/config.toml"),
            "schema = \"atelier.project_config\"\n\n[review]\nmode = \"provider\"\nprovider = \"forgejo\"\n\n[review.providers.forgejo]\nhost = \"forge\"\n\n[review.providers.forgejo.role_authors]\nworker = \"old\"\nreviewer = \"old\"\nvalidator = \"old\"\nmanager = \"old\"\n",
        )
        .unwrap();

        write_role_authors_config(
            dir.path(),
            &role_authors_config_block(&forgejo_config()).unwrap(),
        )
        .unwrap();
        let text = fs::read_to_string(dir.path().join(".atelier/config.toml")).unwrap();

        assert!(
            text.contains("[review.providers.forgejo.role_authors]\nworker = \"atelier-worker\"")
        );
        assert!(!text.contains("worker = \"old\""));
    }
}
