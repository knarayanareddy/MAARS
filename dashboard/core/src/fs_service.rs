use std::path::{Path, PathBuf};

pub fn sanitize_relative_path(input: &str) -> String {
    input
        .chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '\0' => '_',
            _ => c,
        })
        .collect::<String>()
        .replace("..", "_")
}

pub fn ensure_within_root(root: &Path, candidate: &Path) -> anyhow::Result<PathBuf> {
    let root = root.canonicalize()?;
    let resolved = if candidate.exists() {
        candidate.canonicalize()?
    } else {
        candidate.to_path_buf()
    };
    if resolved.starts_with(&root) {
        Ok(resolved)
    } else {
        anyhow::bail!("path escapes project root")
    }
}
