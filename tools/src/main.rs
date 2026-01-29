mod astro;
mod definitions;
mod hayagriva;
mod img;
mod math_conv;
mod templates;
mod typst_eval;
mod typst_parser;
mod version;
mod xref;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(name = "pragmastat-tools")]
#[command(about = "Documentation conversion tool for Pragmastat")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Build documentation outputs
    Build {
        /// Target format: pdf, web, or all
        target: String,
    },
    /// Sync versions and templated docs
    Sync {
        /// Target: version, templates, or all
        target: String,
    },
    /// Clean generated files
    Clean,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Determine base path (project root)
    let base_path = find_project_root()?;

    match cli.command {
        Commands::Build { target } => match target.as_str() {
            "web" => build_web(&base_path)?,
            "img" => build_img(&base_path)?,
            "all" => {
                build_img(&base_path)?;
                build_web(&base_path)?;
            }
            _ => anyhow::bail!("Unknown target: {target}. Use 'web', 'img', or 'all'"),
        },
        Commands::Sync { target } => match target.as_str() {
            "version" => sync_version(&base_path)?,
            "templates" => sync_templates(&base_path)?,
            "all" => {
                sync_version(&base_path)?;
                sync_templates(&base_path)?;
            }
            _ => anyhow::bail!("Unknown target: {target}. Use 'version', 'templates', or 'all'"),
        },
        Commands::Clean => clean(&base_path)?,
    }

    Ok(())
}

fn find_project_root() -> Result<PathBuf> {
    let current = std::env::current_dir()?;
    let mut path = current.as_path();

    loop {
        if path.join("CITATION.cff").exists() {
            return Ok(path.to_path_buf());
        }
        match path.parent() {
            Some(parent) => path = parent,
            None => break,
        }
    }

    anyhow::bail!("Could not find project root (CITATION.cff not found)")
}

/// Chapter metadata for web generation
struct Chapter {
    slug: &'static str,
    file: &'static str,
    title: &'static str,
    order: u8,
}

impl astro::ChapterInfoProvider<astro::ChapterInfoRef> for Chapter {
    fn chapter_info(&self) -> astro::ChapterInfoRef {
        astro::ChapterInfoRef {
            slug: self.slug,
            title: self.title,
        }
    }
}

const CHAPTERS: &[Chapter] = &[
    Chapter {
        slug: "toolkit",
        file: "chapters/toolkit",
        title: "Toolkit",
        order: 1,
    },
    Chapter {
        slug: "distributions",
        file: "chapters/distributions",
        title: "Distributions",
        order: 2,
    },
    Chapter {
        slug: "algorithms",
        file: "chapters/algorithms",
        title: "Algorithms",
        order: 3,
    },
    Chapter {
        slug: "studies",
        file: "chapters/studies",
        title: "Studies",
        order: 4,
    },
    Chapter {
        slug: "implementations",
        file: "chapters/implementations",
        title: "Implementations",
        order: 5,
    },
    Chapter {
        slug: "tests",
        file: "chapters/tests",
        title: "Tests",
        order: 6,
    },
    Chapter {
        slug: "methodology",
        file: "chapters/methodology",
        title: "Methodology",
        order: 7,
    },
];

fn build_web(base_path: &Path) -> Result<()> {
    println!("Building web output...");

    let manual_path = base_path.join("manual");
    let web_content_path = base_path.join("web/src/content/manual");
    let web_public_path = base_path.join("web/public");
    let web_public_img_path = web_public_path.join("img");

    // Create output directories
    std::fs::create_dir_all(&web_content_path)?;
    std::fs::create_dir_all(&web_public_img_path)?;

    // Load definitions
    let definitions = definitions::load_definitions(&manual_path.join("definitions.yaml"))?;

    // Load references from Hayagriva YAML
    let yaml_content = std::fs::read_to_string(base_path.join("manual/references.yaml"))?;
    let references = hayagriva::parse_hayagriva(&yaml_content)?;

    // Generate references.json
    let refs_json = serde_json::to_string_pretty(&references)?;
    std::fs::write(web_public_path.join("references.json"), &refs_json)?;
    println!(
        "  Generated: web/public/references.json ({} references)",
        references.len()
    );

    // Generate index page (abstract only)
    let abstract_content = std::fs::read_to_string(manual_path.join("abstract.typ"))?;
    let index_mdx = astro::generate_index_page(&abstract_content, CHAPTERS);
    std::fs::write(web_content_path.join("index.mdx"), index_mdx)?;
    println!("  Generated: web/src/content/manual/index.mdx");

    // Create cross-reference map for internal links
    let xref_map = xref::XRefMap::new();

    // Generate each chapter as a separate page and collect used citations
    let mut used_citations = std::collections::HashSet::new();
    for chapter in CHAPTERS {
        let typ_path = manual_path.join(format!("{}.typ", chapter.file));
        let content = typst_parser::parse_typst_document(&typ_path, base_path)?;
        used_citations.extend(content.extract_citations());
        let mdx_content = astro::convert_typst_to_mdx(
            &content,
            &definitions,
            &references,
            &xref_map,
            chapter.title,
            chapter.order,
        );

        let output_file = format!("{}.mdx", chapter.slug);
        std::fs::write(web_content_path.join(&output_file), mdx_content)?;
        println!("  Generated: web/src/content/manual/{output_file}");
    }

    // Generate bibliography page (only includes actually used references)
    let bibliography_mdx = astro::generate_bibliography_page(&references, &used_citations, 12);
    std::fs::write(web_content_path.join("bibliography.mdx"), bibliography_mdx)?;
    println!(
        "  Generated: web/src/content/manual/bibliography.mdx ({} cited)",
        used_citations.len()
    );

    // Generate colophon page
    let colophon_info = astro::ColophonInfo {
        author: "Andrey Akinshin",
        email: "andrey.akinshin@gmail.com",
        doi: "10.5281/zenodo.17236778",
        github_url: "https://github.com/AndreyAkinshin/pragmastat",
    };
    let colophon_mdx = astro::generate_colophon_page(&colophon_info, 13);
    std::fs::write(web_content_path.join("colophon.mdx"), colophon_mdx)?;
    println!("  Generated: web/src/content/manual/colophon.mdx");

    // Generate astro config with KaTeX macros
    let katex_config = astro::generate_katex_config(&definitions);
    let config_path = base_path.join("web/katex-macros.json");
    std::fs::write(&config_path, katex_config)?;
    println!("  Generated: web/katex-macros.json");

    // Copy themed images from img/ to web/public/img
    // The img/ directory contains both _light.png and _dark.png variants for theme switching
    let img_path = base_path.join("img");
    if img_path.exists() {
        let mut copied = 0;
        for entry in std::fs::read_dir(&img_path)? {
            let entry = entry?;
            let path = entry.path();
            if path
                .extension()
                .is_some_and(|ext| ext == "png" || ext == "jpg" || ext == "svg")
            {
                let dest = web_public_img_path.join(entry.file_name());
                std::fs::copy(&path, &dest)?;
                copied += 1;
            }
        }
        if copied > 0 {
            println!("  Copied: {copied} images to web/public/img/");
        }

        // Copy favicon files to web/public/
        let favicon_ico = img_path.join("logo.ico");
        if favicon_ico.exists() {
            std::fs::copy(&favicon_ico, web_public_path.join("favicon.ico"))?;
            println!("  Copied: favicon.ico to web/public/");
        }
        let favicon_32 = img_path.join("favicon-32.png");
        if favicon_32.exists() {
            std::fs::copy(&favicon_32, web_public_path.join("favicon-32.png"))?;
            println!("  Copied: favicon-32.png to web/public/");
        }
        let apple_touch = img_path.join("apple-touch-icon.png");
        if apple_touch.exists() {
            std::fs::copy(&apple_touch, web_public_path.join("apple-touch-icon.png"))?;
            println!("  Copied: apple-touch-icon.png to web/public/");
        }
    }

    println!("Web generation complete.");
    Ok(())
}

fn build_img(base_path: &Path) -> Result<()> {
    println!("Building image assets...");
    img::generate_logo(base_path)?;
    println!("Image generation complete.");
    Ok(())
}

fn sync_version(base_path: &Path) -> Result<()> {
    let version = version::read_version(base_path)?;
    version::sync_versions(base_path, &version)
}

fn sync_templates(base_path: &Path) -> Result<()> {
    let version = version::read_version(base_path)?;
    templates::sync_templates(base_path, &version)
}

fn clean(base_path: &Path) -> Result<()> {
    println!("Cleaning generated files...");

    let files_to_remove = [
        "manual/pragmastat.pdf",
        "web/src/content/manual/index.mdx",
        "web/src/content/manual/bibliography.mdx",
        "web/src/content/manual/colophon.mdx",
        "web/katex-macros.json",
        "web/public/references.json",
    ];

    for file in &files_to_remove {
        let path = base_path.join(file);
        if path.exists() {
            std::fs::remove_file(&path)?;
            println!("  Removed: {file}");
        }
    }

    // Clean chapter MDX files
    for chapter in CHAPTERS {
        let file = format!("web/src/content/manual/{}.mdx", chapter.slug);
        let path = base_path.join(&file);
        if path.exists() {
            std::fs::remove_file(&path)?;
            println!("  Removed: {file}");
        }
    }

    // Clean copied images in web/public/img
    let web_img_path = base_path.join("web/public/img");
    if web_img_path.exists() {
        let mut removed = 0;
        for entry in std::fs::read_dir(&web_img_path)? {
            let entry = entry?;
            std::fs::remove_file(entry.path())?;
            removed += 1;
        }
        if removed > 0 {
            println!("  Removed: {removed} images from web/public/img/");
        }
        // Remove the directory if empty
        let _ = std::fs::remove_dir(&web_img_path);
    }

    // Clean favicon files
    let favicon_files = [
        "web/public/favicon.ico",
        "web/public/favicon-32.png",
        "web/public/apple-touch-icon.png",
    ];
    for file in &favicon_files {
        let path = base_path.join(file);
        if path.exists() {
            std::fs::remove_file(&path)?;
            println!("  Removed: {file}");
        }
    }

    println!("Clean complete.");
    Ok(())
}
