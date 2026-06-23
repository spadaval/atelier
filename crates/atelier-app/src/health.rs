use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use atelier_sqlite::Database;
use std::env;

use crate::forgejo::{
    ForgejoClient, ForgejoRequest, ForgejoResponse, ForgejoTransport, UreqForgejoTransport,
};
use crate::project_config::{
    workflow_forgejo_role_authors, ForgejoConfig, ProjectConfig, ReviewConfig,
    ReviewProviderConfig, ReviewProviderKind,
};

pub struct DoctorRequest<'a> {
    pub db: &'a Database,
    pub repo_root: PathBuf,
    pub state_dir: PathBuf,
    pub db_path: PathBuf,
    pub projection_db_existed: bool,
    pub fix: bool,
    pub diagnostics_enabled: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DoctorView {
    pub db_path: PathBuf,
    pub state_dir: PathBuf,
    pub fix: bool,
    pub config_ok: bool,
    pub ignore_rules_current: bool,
    pub state_dir_ok: bool,
    pub rebuild_ready: bool,
    pub projection_fresh: bool,
    pub cache_dir_status: &'static str,
    pub runtime_db_available: bool,
    pub diagnostics: &'static str,
    pub review_backend: ReviewBackendView,
    pub health: BTreeMap<&'static str, bool>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReviewBackendView {
    pub mode: String,
    pub provider: Option<String>,
    pub status: &'static str,
    pub token_env: Option<String>,
    pub detail: String,
}

pub fn doctor(
    request: crate::Request<DoctorRequest<'_>>,
) -> Result<crate::Outcome<crate::ViewModel<DoctorView>>> {
    let input = request.input;
    let layout = crate::storage_layout::StorageLayout::new(&input.repo_root);
    let config_path = layout.config_path();
    let cache_dir = layout.cache_dir();

    let repaired_db;
    let active_db = if input.fix {
        crate::rebuild::validate_canonical_state(&input.state_dir).with_context(|| {
            "doctor --fix refused to edit tracked `.atelier/` canonical records; \
             run `atelier lint`, fix the named canonical Markdown record, then rerun `atelier doctor --fix`"
        })?;
        crate::rebuild::refresh_projection(&input.state_dir, &input.db_path).with_context(
            || {
                format!(
                    "doctor --fix failed while repairing ignored local projection state at {}",
                    input.db_path.display()
                )
            },
        )?;
        repaired_db =
            Database::open(&input.db_path).context("Failed to reopen repaired database")?;
        &repaired_db
    } else {
        input.db
    };

    let rebuild_ready = crate::rebuild::validate_canonical_state(&input.state_dir).is_ok();
    let projection_fresh = atelier_sqlite::projection_index::check(active_db, &input.state_dir)
        .map(|report| report.is_fresh())
        .unwrap_or(false);
    let runtime_db_available = if input.fix {
        input.db_path.exists()
    } else {
        input.projection_db_existed
    };
    let state_dir_ok = input.state_dir.is_dir();
    let ignore_rules_current = runtime_gitignore_entries_present(&input.repo_root);
    let diagnostics = if input.diagnostics_enabled {
        "enabled"
    } else {
        "disabled"
    };
    let mut health = BTreeMap::new();
    health.insert("config", config_path.exists());
    health.insert("database", runtime_db_available);
    health.insert("ignore_rules", ignore_rules_current);
    health.insert("projection_fresh", projection_fresh);
    health.insert("rebuild_ready", rebuild_ready);
    let review_backend = review_backend_health(&input.repo_root);
    health.insert("review_backend", review_backend.status != "not ok");

    Ok(crate::Outcome {
        value: crate::ViewModel {
            data: DoctorView {
                db_path: input.db_path,
                state_dir: input.state_dir,
                fix: input.fix,
                config_ok: config_path.exists(),
                ignore_rules_current,
                state_dir_ok,
                rebuild_ready,
                projection_fresh,
                cache_dir_status: optional_dir_status(&cache_dir),
                runtime_db_available,
                diagnostics,
                review_backend,
                health,
            },
        },
    })
}

fn review_backend_health(repo_root: &Path) -> ReviewBackendView {
    let mut config = match ProjectConfig::load(repo_root) {
        Ok(config) => config,
        Err(error) => {
            return ReviewBackendView {
                mode: "unknown".to_string(),
                provider: None,
                status: "not ok",
                token_env: None,
                detail: format!("project config is invalid: {error:#}"),
            };
        }
    };
    if let ReviewConfig::Provider(ReviewProviderConfig {
        provider: ReviewProviderKind::Forgejo(forgejo),
    }) = &mut config.review
    {
        match workflow_forgejo_role_authors(repo_root) {
            Ok(role_authors) => forgejo.role_authors = Some(role_authors),
            Err(error) => {
                return ReviewBackendView {
                    mode: "provider".to_string(),
                    provider: Some("forgejo".to_string()),
                    status: "not ok",
                    token_env: Some(forgejo.admin_token_env.clone()),
                    detail: format!(
                        "{error:#}; define role_authors on Forgejo review.open actions in .atelier/workflow.yaml, then run `atelier workflow check`"
                    ),
                };
            }
        }
    }
    review_backend_health_from_config(
        &config,
        |name| env::var(name).ok(),
        |forgejo, token| probe_forgejo_readiness(forgejo, token),
    )
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ForgejoReadiness {
    repo: ForgejoResponse,
    role_failures: Vec<String>,
}

fn review_backend_health_from_config<TokenLookup, Probe>(
    config: &ProjectConfig,
    token_lookup: TokenLookup,
    probe: Probe,
) -> ReviewBackendView
where
    TokenLookup: FnOnce(&str) -> Option<String>,
    Probe: FnOnce(&ForgejoConfig, &str) -> Result<ForgejoReadiness>,
{
    match &config.review {
        ReviewConfig::Room => ReviewBackendView {
            mode: "room".to_string(),
            provider: None,
            status: "skipped",
            token_env: None,
            detail: "native review rooms do not require provider credentials".to_string(),
        },
        ReviewConfig::Provider(ReviewProviderConfig {
            provider: ReviewProviderKind::Forgejo(forgejo),
        }) => {
            let token_env = forgejo.admin_token_env.clone();
            if forgejo.role_authors.is_none() {
                return ReviewBackendView {
                    mode: "provider".to_string(),
                    provider: Some("forgejo".to_string()),
                    status: "not ok",
                    token_env: Some(token_env.clone()),
                    detail: "missing Forgejo role author configuration; define role_authors on Forgejo review.open actions in .atelier/workflow.yaml, then run `atelier workflow check`".to_string(),
                };
            }
            let Some(token) = token_lookup(&token_env).filter(|value| !value.trim().is_empty())
            else {
                return ReviewBackendView {
                    mode: "provider".to_string(),
                    provider: Some("forgejo".to_string()),
                    status: "not ok",
                    token_env: Some(token_env.clone()),
                    detail: format!(
                        "missing token environment variable {}; set it before running provider review commands",
                        token_env
                    ),
                };
            };
            match probe(forgejo, &token) {
                Ok(readiness)
                    if (200..300).contains(&readiness.repo.status)
                        && readiness.role_failures.is_empty() =>
                {
                    ReviewBackendView {
                    mode: "provider".to_string(),
                    provider: Some("forgejo".to_string()),
                    status: "ok",
                    token_env: Some(token_env),
                    detail: format!(
                        "Forgejo repository {}/{} is reachable and role authors have write access plus sudo verification",
                        forgejo.owner, forgejo.repo
                    ),
                    }
                }
                Ok(readiness) if (200..300).contains(&readiness.repo.status) => ReviewBackendView {
                    mode: "provider".to_string(),
                    provider: Some("forgejo".to_string()),
                    status: "not ok",
                    token_env: Some(token_env),
                    detail: format!(
                        "Forgejo role author readiness failed: {}; run `atelier forgejo roles check` or `atelier forgejo roles provision`",
                        readiness.role_failures.join("; ")
                    ),
                },
                Ok(readiness) => ReviewBackendView {
                    mode: "provider".to_string(),
                    provider: Some("forgejo".to_string()),
                    status: "not ok",
                    token_env: Some(token_env),
                    detail: forgejo_status_detail(readiness.repo.status, &readiness.repo.body),
                },
                Err(error) => ReviewBackendView {
                    mode: "provider".to_string(),
                    provider: Some("forgejo".to_string()),
                    status: "not ok",
                    token_env: Some(token_env),
                    detail: format!("Forgejo provider is unreachable: {error:#}"),
                },
            }
        }
    }
}

fn probe_forgejo_readiness(forgejo: &ForgejoConfig, token: &str) -> Result<ForgejoReadiness> {
    let transport = UreqForgejoTransport::new(&forgejo.host, token);
    let repo = transport.send(ForgejoRequest {
        method: "GET",
        path: format!("/api/v1/repos/{}/{}", forgejo.owner, forgejo.repo),
        query: Vec::new(),
        headers: BTreeMap::new(),
        body: None,
    })?;
    if !(200..300).contains(&repo.status) {
        return Ok(ForgejoReadiness {
            repo,
            role_failures: Vec::new(),
        });
    }
    let client = ForgejoClient::new(forgejo.clone(), transport);
    Ok(ForgejoReadiness {
        repo,
        role_failures: forgejo_role_failures(&client, forgejo)?,
    })
}

fn forgejo_role_failures<T: ForgejoTransport>(
    client: &ForgejoClient<T>,
    forgejo: &ForgejoConfig,
) -> Result<Vec<String>> {
    let mut failures = Vec::new();
    for role in crate::project_config::FORGEJO_ROLES {
        let username = forgejo.role_author_for_role(role)?;
        let user_exists = client.user_exists(username)?;
        if !user_exists {
            failures.push(format!("{role} user {username} does not exist"));
            continue;
        }
        if !matches!(
            client.collaborator_permission(username)?.as_deref(),
            Some("write" | "admin")
        ) {
            failures.push(format!(
                "{role} user {username} does not have write permission"
            ));
        }
        if !client.verify_sudo_user(username)? {
            failures.push(format!("{role} user {username} failed sudo verification"));
        }
    }
    Ok(failures)
}

fn forgejo_status_detail(status: u16, body: &str) -> String {
    let reason = match status {
        401 | 403 => "invalid or unauthorized Forgejo credentials",
        404 => "configured Forgejo repository was not found",
        _ => "Forgejo provider check failed",
    };
    if body.trim().is_empty() {
        format!("{reason} (status {status})")
    } else {
        format!("{reason} (status {status}): {}", body.trim())
    }
}

fn runtime_gitignore_entries_present(repo_root: &Path) -> bool {
    let Ok(gitignore) = std::fs::read_to_string(repo_root.join(".gitignore")) else {
        return false;
    };
    crate::init::ROOT_GITIGNORE_ENTRIES
        .iter()
        .all(|entry| gitignore.lines().any(|line| line.trim() == *entry))
}

fn optional_dir_status(path: &Path) -> &'static str {
    if path.is_dir() {
        "ok"
    } else {
        "missing (optional)"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::project_config::{
        ForgejoRoleAuthors, IssueLinkConfig, ProjectPaths, PruneConfig,
        DEFAULT_CANONICAL_PRUNE_RETENTION_DAYS,
    };

    fn room_config() -> ProjectConfig {
        ProjectConfig {
            project_slug: "atelier".to_string(),
            paths: ProjectPaths {
                state_root: ".atelier".to_string(),
            },
            issue_links: IssueLinkConfig {
                custom_context_types: Vec::new(),
            },
            prune: PruneConfig {
                canonical_retention_days: DEFAULT_CANONICAL_PRUNE_RETENTION_DAYS,
            },
            review: ReviewConfig::Room,
        }
    }

    fn forgejo_config() -> ForgejoConfig {
        ForgejoConfig {
            host: "https://forge.example.test".to_string(),
            owner: "tools".to_string(),
            repo: "atelier".to_string(),
            admin_token_env: "FORGEJO_ADMIN_TOKEN".to_string(),
            role_authors: Some(ForgejoRoleAuthors {
                worker: "atelier-worker".to_string(),
                reviewer: "atelier-reviewer".to_string(),
                validator: "atelier-validator".to_string(),
                manager: "atelier-manager".to_string(),
            }),
        }
    }

    fn provider_config() -> ProjectConfig {
        ProjectConfig {
            review: ReviewConfig::Provider(ReviewProviderConfig {
                provider: ReviewProviderKind::Forgejo(forgejo_config()),
            }),
            ..room_config()
        }
    }

    #[test]
    fn review_backend_health_skips_room_mode() {
        let view = review_backend_health_from_config(
            &room_config(),
            |_| Some("unused".to_string()),
            |_, _| unreachable!("room mode must not probe providers"),
        );
        assert_eq!(view.status, "skipped");
        assert_eq!(view.mode, "room");
        assert!(view.token_env.is_none());
    }

    #[test]
    fn review_backend_health_reports_missing_provider_token_without_secret() {
        let view = review_backend_health_from_config(
            &provider_config(),
            |_| None,
            |_, _| unreachable!("missing token must not probe providers"),
        );
        assert_eq!(view.status, "not ok");
        assert_eq!(view.token_env.as_deref(), Some("FORGEJO_ADMIN_TOKEN"));
        assert!(view.detail.contains("missing token"));
        assert!(!view.detail.contains("secret-token"));
    }

    #[test]
    fn review_backend_health_reports_provider_success() {
        let view = review_backend_health_from_config(
            &provider_config(),
            |_| Some("secret-token".to_string()),
            |forgejo, token| {
                assert_eq!(forgejo.repo, "atelier");
                assert_eq!(token, "secret-token");
                Ok(ForgejoReadiness {
                    repo: ForgejoResponse {
                        status: 200,
                        body: "{}".to_string(),
                    },
                    role_failures: Vec::new(),
                })
            },
        );
        assert_eq!(view.status, "ok");
        assert!(!view.detail.contains("secret-token"));
    }

    #[test]
    fn review_backend_health_reports_provider_auth_and_missing_repo() {
        let unauthorized = review_backend_health_from_config(
            &provider_config(),
            |_| Some("secret-token".to_string()),
            |_, _| {
                Ok(ForgejoReadiness {
                    repo: ForgejoResponse {
                        status: 401,
                        body: "bad token".to_string(),
                    },
                    role_failures: Vec::new(),
                })
            },
        );
        assert_eq!(unauthorized.status, "not ok");
        assert!(unauthorized.detail.contains("invalid or unauthorized"));
        assert!(!unauthorized.detail.contains("secret-token"));

        let missing_repo = review_backend_health_from_config(
            &provider_config(),
            |_| Some("secret-token".to_string()),
            |_, _| {
                Ok(ForgejoReadiness {
                    repo: ForgejoResponse {
                        status: 404,
                        body: String::new(),
                    },
                    role_failures: Vec::new(),
                })
            },
        );
        assert_eq!(missing_repo.status, "not ok");
        assert!(missing_repo.detail.contains("repository was not found"));
    }

    #[test]
    fn review_backend_health_reports_unreachable_provider() {
        let view = review_backend_health_from_config(
            &provider_config(),
            |_| Some("secret-token".to_string()),
            |_, _| anyhow::bail!("connection refused"),
        );
        assert_eq!(view.status, "not ok");
        assert!(view.detail.contains("unreachable"));
        assert!(!view.detail.contains("secret-token"));
    }

    #[test]
    fn review_backend_health_reports_missing_role_authors_before_token_lookup() {
        let mut config = provider_config();
        let ReviewConfig::Provider(ReviewProviderConfig {
            provider: ReviewProviderKind::Forgejo(forgejo),
        }) = &mut config.review
        else {
            unreachable!();
        };
        forgejo.role_authors = None;

        let view = review_backend_health_from_config(
            &config,
            |_| panic!("role author failures should not require a token"),
            |_, _| unreachable!("role author failures must not probe providers"),
        );

        assert_eq!(view.status, "not ok");
        assert!(view.detail.contains("role author configuration"));
    }

    #[test]
    fn review_backend_health_reports_role_author_readiness_failures() {
        let view = review_backend_health_from_config(
            &provider_config(),
            |_| Some("secret-token".to_string()),
            |_, _| {
                Ok(ForgejoReadiness {
                    repo: ForgejoResponse {
                        status: 200,
                        body: "{}".to_string(),
                    },
                    role_failures: vec![
                        "worker user atelier-worker does not have write permission".to_string(),
                    ],
                })
            },
        );

        assert_eq!(view.status, "not ok");
        assert!(view.detail.contains("worker user atelier-worker"));
        assert!(view.detail.contains("forgejo roles check"));
        assert!(!view.detail.contains("secret-token"));
    }
}
