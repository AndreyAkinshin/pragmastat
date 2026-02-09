use anyhow::{Context, Result};
use regex::Regex;
use std::path::{Path, PathBuf};

struct VersionTarget {
    path: &'static str,
    pattern: &'static str,
    replacement: &'static str,
}

const VERSION_TARGETS: &[VersionTarget] = &[
    VersionTarget {
        path: "cs/Directory.Build.props",
        pattern: r"<Version>.*?</Version>",
        replacement: "<Version>{version}</Version>",
    },
    VersionTarget {
        path: "kt/build.gradle.kts",
        pattern: r#"version = ".*?""#,
        replacement: r#"version = "{version}""#,
    },
    VersionTarget {
        path: "py/pyproject.toml",
        pattern: r#"version = ".*?""#,
        replacement: r#"version = "{version}""#,
    },
    VersionTarget {
        path: "py/pragmastat/__init__.py",
        pattern: r#"__version__ = ".*?""#,
        replacement: r#"__version__ = "{version}""#,
    },
    VersionTarget {
        path: "r/pragmastat/DESCRIPTION",
        pattern: r"Version: .*",
        replacement: "Version: {version}",
    },
    VersionTarget {
        path: "ts/package.json",
        pattern: r#""version": ".*?""#,
        replacement: r#""version": "{version}""#,
    },
    VersionTarget {
        path: "ts/package-lock.json",
        pattern: r#"("name":\s*"pragmastat",\s*)"version":\s*"[^"]*""#,
        replacement: r#"$1"version": "{version}""#,
    },
    // Version in version.typ is used by all Typst files
    VersionTarget {
        path: "manual/version.typ",
        pattern: r#"#let version = ".*?""#,
        replacement: r#"#let version = "{version}""#,
    },
    // Web frontpage version display
    VersionTarget {
        path: "web/src/pages/index.astro",
        pattern: r">v\d+\.\d+\.\d+<",
        replacement: ">v{version}<",
    },
];

pub fn read_version(base_path: &Path) -> Result<String> {
    let version_path = base_path.join("VERSION");
    let content = std::fs::read_to_string(&version_path)
        .with_context(|| format!("Failed to read {}", version_path.display()))?;
    let version = content.trim().to_string();
    if version.is_empty() {
        anyhow::bail!("VERSION file is empty");
    }
    Ok(version)
}

pub fn sync_versions(base_path: &Path, version: &str) -> Result<()> {
    println!("Syncing versions to {version}...");

    sync_rust_version(base_path, version)?;

    for target in VERSION_TARGETS {
        let file_path = base_path.join(target.path);
        if !file_path.exists() {
            println!("  Skipped: {} (missing)", target.path);
            continue;
        }

        let content = std::fs::read_to_string(&file_path)
            .with_context(|| format!("Failed to read {}", file_path.display()))?;
        let regex = Regex::new(target.pattern)
            .with_context(|| format!("Invalid regex for {}", target.path))?;
        if !regex.is_match(&content) {
            anyhow::bail!(
                "Pattern not found in {} (pattern: {})",
                target.path,
                target.pattern
            );
        }

        let replacement = target.replacement.replace("{version}", version);
        let updated = regex.replace_all(&content, replacement.as_str());
        write_if_changed(&file_path, updated.as_ref(), target.path)?;
    }

    Ok(())
}

fn write_if_changed(path: &PathBuf, content: &str, label: &str) -> Result<()> {
    let existing = std::fs::read_to_string(path).unwrap_or_default();
    if existing == content {
        println!("  Unchanged: {label}");
        return Ok(());
    }

    std::fs::write(path, content).with_context(|| format!("Failed to write {}", path.display()))?;
    println!("  Updated: {label}");
    Ok(())
}

fn sync_rust_version(base_path: &Path, version: &str) -> Result<()> {
    let file_path = base_path.join("rs/pragmastat/Cargo.toml");
    if !file_path.exists() {
        println!("  Skipped: rs/pragmastat/Cargo.toml (missing)");
        return Ok(());
    }

    let content = std::fs::read_to_string(&file_path)
        .with_context(|| format!("Failed to read {}", file_path.display()))?;
    let mut lines: Vec<String> = Vec::new();
    let mut in_package = false;
    let mut updated = false;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed == "[package]" {
            in_package = true;
            lines.push(line.to_string());
            continue;
        }
        if trimmed.starts_with('[') && trimmed != "[package]" {
            in_package = false;
        }

        if in_package && trimmed.starts_with("version =") {
            let indent: String = line.chars().take_while(|c| c.is_whitespace()).collect();
            lines.push(format!("{indent}version = \"{version}\""));
            updated = true;
            continue;
        }

        lines.push(line.to_string());
    }

    if !updated {
        anyhow::bail!("Version line not found in rs/pragmastat/Cargo.toml");
    }

    let mut rebuilt = lines.join("\n");
    if content.ends_with('\n') {
        rebuilt.push('\n');
    }

    write_if_changed(&file_path, &rebuilt, "rs/pragmastat/Cargo.toml")
}
