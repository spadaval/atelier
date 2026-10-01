//! Build script that injects the git commit into `atelier --version`.

fn main() {
    // Inject git commit hash into the binary for `atelier --version`
    println!("cargo:rerun-if-changed=../.git/HEAD");
    println!("cargo:rerun-if-changed=../.git/refs/");
    if let Ok(output) = std::process::Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
    {
        if output.status.success() {
            let hash = String::from_utf8_lossy(&output.stdout).trim().to_string();
            let dirty = std::process::Command::new("git")
                .args(["status", "--porcelain"])
                .output()
                .map(|o| !o.stdout.is_empty())
                .unwrap_or(false);
            let suffix = if dirty {
                format!("{}+{}-dirty", env!("CARGO_PKG_VERSION"), hash)
            } else {
                format!("{}+{}", env!("CARGO_PKG_VERSION"), hash)
            };
            println!("cargo:rustc-env=ATELIER_VERSION={}", suffix);
        }
    }
}
