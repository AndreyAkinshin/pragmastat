use anyhow::{Context, Result};
use std::fmt::Write;
use std::path::{Path, PathBuf};

#[allow(clippy::struct_field_names)]
struct Language {
    slug: &'static str,
    title: &'static str,
    code_language: &'static str,
    demo_path: &'static str,
    readme_path: &'static str,
    package_url: &'static str,
    /// Install instructions in Markdown format with {version} and {major} placeholders
    install_md: &'static str,
}

const LANGUAGES: &[Language] = &[
    Language {
        slug: "cs",
        title: "C#",
        code_language: "cs",
        demo_path: "cs/Pragmastat.Demo/Program.cs",
        readme_path: "cs/README.md",
        package_url: "Pragmastat on NuGet: https://www.nuget.org/packages/Pragmastat/",
        install_md: r"Install from NuGet via .NET CLI:

```bash
dotnet add package Pragmastat --version {version}
```

Install from NuGet via Package Manager Console:

```ps1
NuGet\Install-Package Pragmastat -Version {version}
```",
    },
    Language {
        slug: "go",
        title: "Go",
        code_language: "go",
        demo_path: "go/demo/main.go",
        readme_path: "go/README.md",
        package_url: "",
        install_md: r"Install from GitHub:

```bash
go get github.com/AndreyAkinshin/pragmastat/go/v{major}@v{version}
```",
    },
    Language {
        slug: "kt",
        title: "Kotlin",
        code_language: "kotlin",
        demo_path: "kt/src/main/kotlin/dev/pragmastat/demo/Main.kt",
        readme_path: "kt/README.md",
        package_url: "Pragmastat on Maven Central Repository: https://central.sonatype.com/artifact/dev.pragmastat/pragmastat/overview",
        install_md: r#"Install from Maven Central Repository via Apache Maven:

```xml
<dependency>
    <groupId>dev.pragmastat</groupId>
    <artifactId>pragmastat</artifactId>
    <version>{version}</version>
</dependency>
```

Install from Maven Central Repository via Gradle:

```java
implementation 'dev.pragmastat:pragmastat:{version}'
```

Install from Maven Central Repository via Gradle (Kotlin):

```kotlin
implementation("dev.pragmastat:pragmastat:{version}")
```"#,
    },
    Language {
        slug: "py",
        title: "Python",
        code_language: "python",
        demo_path: "py/examples/demo.py",
        readme_path: "py/README.md",
        package_url: "Pragmastat on PyPI: https://pypi.org/project/pragmastat/",
        install_md: r"Install from PyPI:

```bash
pip install pragmastat=={version}
```",
    },
    Language {
        slug: "r",
        title: "R",
        code_language: "r",
        demo_path: "r/pragmastat/inst/examples/demo.R",
        readme_path: "r/pragmastat/README.md",
        package_url: "",
        install_md: r#"Install from GitHub:

```r
install.packages("remotes") # If 'remotes' is not installed
remotes::install_github("AndreyAkinshin/pragmastat",
                        subdir = "r/pragmastat", ref = "v{version}")
library(pragmastat)
```"#,
    },
    Language {
        slug: "rs",
        title: "Rust",
        code_language: "rust",
        demo_path: "rs/pragmastat/examples/demo.rs",
        readme_path: "rs/pragmastat/README.md",
        package_url: "Pragmastat on crates.io: https://crates.io/crates/pragmastat",
        install_md: r#"Install from crates.io via cargo:

```bash
cargo add pragmastat@{version}
```

Install from crates.io via `Cargo.toml`:

```toml
[dependencies]
pragmastat = "{version}"
```"#,
    },
    Language {
        slug: "ts",
        title: "TypeScript",
        code_language: "typescript",
        demo_path: "ts/examples/demo.ts",
        readme_path: "ts/README.md",
        package_url: "Pragmastat on npm: https://www.npmjs.com/package/pragmastat",
        install_md: r"Install from npm:

```bash
npm i pragmastat@{version}
```",
    },
];

/// Generate README.md files for each language directory.
/// The Typst implementation files are now pure Typst (no generation needed).
pub fn sync_templates(base_path: &Path, version: &str) -> Result<()> {
    println!("Syncing READMEs...");

    for lang in LANGUAGES {
        let demo_path = base_path.join(lang.demo_path);
        let demo_code = std::fs::read_to_string(&demo_path)
            .with_context(|| format!("Failed to read {}", demo_path.display()))?;
        let demo_code = demo_code.trim_end();

        let readme_content = generate_readme(lang, version, demo_code);
        let readme_output = base_path.join(lang.readme_path);
        write_if_changed(&readme_output, &readme_content)?;
    }

    Ok(())
}

fn generate_readme(lang: &Language, version: &str, demo: &str) -> String {
    let major = version.split('.').next().unwrap_or(version);
    let install = lang.install_md
        .replace("{version}", version)
        .replace("{major}", major);
    let source_url = format!(
        "https://github.com/AndreyAkinshin/pragmastat/tree/v{}/{}",
        version, lang.slug
    );

    let mut content = format!(
        "# {}\n\n{}\n\nSource code: {}\n\n",
        lang.title, install, source_url
    );

    if !lang.package_url.is_empty() {
        content.push_str(lang.package_url);
        content.push_str("\n\n");
    }

    let _ = write!(
        content,
        "## Demo\n\n```{}\n{}\n```\n",
        lang.code_language, demo
    );

    content
}

fn write_if_changed(path: &PathBuf, content: &str) -> Result<()> {
    let existing = std::fs::read_to_string(path).unwrap_or_default();
    if existing == content {
        println!("  Unchanged: {}", path.display());
        return Ok(());
    }

    std::fs::write(path, content).with_context(|| format!("Failed to write {}", path.display()))?;
    println!("  Updated: {}", path.display());
    Ok(())
}
