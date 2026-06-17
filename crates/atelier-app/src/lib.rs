//! Application use-case boundary.
//!
//! Command orchestration moves here as request, outcome, and view-model APIs
//! that do not write directly to stdout or stderr.

pub mod command_storage;
pub mod export;
pub mod health;
pub mod init;
pub mod lint;
pub mod projection;
pub mod rebuild;
pub mod storage_layout;
pub mod use_cases;
pub mod workflow_policy;

/// Minimal outcome wrapper for early app-layer extraction.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Outcome<T> {
    pub value: T,
}

/// Request wrapper for use-case entrypoints.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Request<T> {
    pub input: T,
}

/// View model wrapper returned to the CLI renderer.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ViewModel<T> {
    pub data: T,
}

pub fn handle<T>(request: Request<T>) -> Outcome<ViewModel<T>> {
    Outcome {
        value: ViewModel {
            data: request.input,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_entrypoint_returns_view_model_without_rendering() {
        let outcome = handle(Request { input: "status" });
        assert_eq!(outcome.value.data, "status");
    }
}
