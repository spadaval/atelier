use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use atelier_sqlite::Database;
use std::env;

use crate::forgejo::{ForgejoRequest, ForgejoResponse, ForgejoTransport, UreqForgejoTransport};
use crate::project_config::{
    ForgejoConfig, ProjectConfig, ReviewConfig, ReviewProviderConfig, ReviewProviderKind,
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
    let config = match ProjectConfig::load(repo_root) {
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
    review_backend_health_from_config(
        &config,
        |name| env::var(name).ok(),
        |forgejo, token| probe_forgejo_repo(forgejo, token),
    )
}

fn review_backend_health_from_config<TokenLookup, Probe>(
    config: &ProjectConfig,
    token_lookup: TokenLookup,
    probe: Probe,
) -> ReviewBackendView
where
    TokenLookup: FnOnce(&str) -> Option<String>,
    Probe: FnOnce(&ForgejoConfig, &str) -> Result<ForgejoResponse>,
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
                Ok(response) if (200..300).contains(&response.status) => ReviewBackendView {
                    mode: "provider".to_string(),
                    provider: Some("forgejo".to_string()),
                    status: "ok",
                    token_env: Some(token_env),
                    detail: format!(
                        "Forgejo repository {}/{} is reachable and role mappings are configured",
                        forgejo.owner, forgejo.repo
                    ),
                },
                Ok(response) => ReviewBackendView {
                    mode: "provider".to_string(),
                    provider: Some("forgejo".to_string()),
                    status: "not ok",
                    token_env: Some(token_env),
                    detail: forgejo_status_detail(response.status, &response.body),
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

fn probe_forgejo_repo(forgejo: &ForgejoConfig, token: &str) -> Result<ForgejoResponse> {
    let transport = UreqForgejoTransport::new(&forgejo.host, token);
    transport.send(ForgejoRequest {
        method: "GET",
        path: format!("/api/v1/repos/{}/{}", forgejo.owner, forgejo.repo),
        query: Vec::new(),
        headers: BTreeMap::new(),
        body: None,
    })
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
    use crate::project_config::{ForgejoRoleAuthors, ProjectPaths};

    fn room_config() -> ProjectConfig {
        ProjectConfig {
            project_slug: "atelier".to_string(),
            paths: ProjectPaths {
                state_root: ".atelier".to_string(),
                runtime_dir: ".atelier/runtime".to_string(),
                runtime_database: ".atelier/runtime/state.db".to_string(),
                cache_dir: ".atelier/cache".to_string(),
            },
            compatibility_state_root: None,
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
                Ok(ForgejoResponse {
                    status: 200,
                    body: "{}".to_string(),
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
                Ok(ForgejoResponse {
                    status: 401,
                    body: "bad token".to_string(),
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
                Ok(ForgejoResponse {
                    status: 404,
                    body: String::new(),
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
}
