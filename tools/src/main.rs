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

/// Page metadata for web generation
struct Page {
    slug: &'static str,
    file: &'static str,
    title: &'static str,
    order: u8,
    group: Option<&'static str>,
    heading_offset: i8,
}

impl astro::AsPageInfo for Page {
    fn as_page_info(&self) -> astro::PageInfo {
        astro::PageInfo {
            slug: self.slug,
            title: self.title,
            group: self.group,
        }
    }
}

const PAGES: &[Page] = &[
    // Synopsis
    Page { slug: "synopsis", file: "synopsis/synopsis", title: "Synopsis", order: 0, group: None, heading_offset: 0 },
    // One-Sample Estimators
    Page { slug: "center", file: "center/center", title: "Center", order: 1, group: Some("One-Sample Estimators"), heading_offset: -1 },
    Page { slug: "center-bounds", file: "center-bounds/center-bounds", title: "CenterBounds", order: 2, group: Some("One-Sample Estimators"), heading_offset: -1 },
    Page { slug: "spread", file: "spread/spread", title: "Spread", order: 3, group: Some("One-Sample Estimators"), heading_offset: -1 },
    Page { slug: "spread-bounds", file: "spread-bounds/spread-bounds", title: "SpreadBounds", order: 4, group: Some("One-Sample Estimators"), heading_offset: -1 },
    // Two-Sample Estimators
    Page { slug: "shift", file: "shift/shift", title: "Shift", order: 5, group: Some("Two-Sample Estimators"), heading_offset: -1 },
    Page { slug: "shift-bounds", file: "shift-bounds/shift-bounds", title: "ShiftBounds", order: 6, group: Some("Two-Sample Estimators"), heading_offset: -1 },
    Page { slug: "ratio", file: "ratio/ratio", title: "Ratio", order: 7, group: Some("Two-Sample Estimators"), heading_offset: -1 },
    Page { slug: "ratio-bounds", file: "ratio-bounds/ratio-bounds", title: "RatioBounds", order: 8, group: Some("Two-Sample Estimators"), heading_offset: -1 },
    Page { slug: "disparity", file: "disparity/disparity", title: "Disparity", order: 9, group: Some("Two-Sample Estimators"), heading_offset: -1 },
    Page { slug: "disparity-bounds", file: "disparity-bounds/disparity-bounds", title: "DisparityBounds", order: 10, group: Some("Two-Sample Estimators"), heading_offset: -1 },
    // Randomization
    Page { slug: "rng", file: "rng/rng", title: "Rng", order: 11, group: Some("Randomization"), heading_offset: -1 },
    Page { slug: "uniform-float", file: "uniform-float/uniform-float", title: "UniformFloat", order: 12, group: Some("Randomization"), heading_offset: -1 },
    Page { slug: "uniform-int", file: "uniform-int/uniform-int", title: "UniformInt", order: 13, group: Some("Randomization"), heading_offset: -1 },
    Page { slug: "sample", file: "sample/sample", title: "Sample", order: 14, group: Some("Randomization"), heading_offset: -1 },
    Page { slug: "resample", file: "resample/resample", title: "Resample", order: 15, group: Some("Randomization"), heading_offset: -1 },
    Page { slug: "shuffle", file: "shuffle/shuffle", title: "Shuffle", order: 16, group: Some("Randomization"), heading_offset: -1 },
    // Distributions
    Page { slug: "additive", file: "additive/additive", title: "Additive", order: 17, group: Some("Distributions"), heading_offset: -1 },
    Page { slug: "multiplic", file: "multiplic/multiplic", title: "Multiplic", order: 18, group: Some("Distributions"), heading_offset: -1 },
    Page { slug: "exp", file: "exp/exp", title: "Exp", order: 19, group: Some("Distributions"), heading_offset: -1 },
    Page { slug: "power", file: "power/power", title: "Power", order: 20, group: Some("Distributions"), heading_offset: -1 },
    Page { slug: "uniform", file: "uniform/uniform", title: "Uniform", order: 21, group: Some("Distributions"), heading_offset: -1 },
    // Implementations
    Page { slug: "py", file: "implementations/py", title: "Python", order: 22, group: Some("Implementations"), heading_offset: -1 },
    Page { slug: "ts", file: "implementations/ts", title: "TypeScript", order: 23, group: Some("Implementations"), heading_offset: -1 },
    Page { slug: "r", file: "implementations/r", title: "R", order: 24, group: Some("Implementations"), heading_offset: -1 },
    Page { slug: "cs", file: "implementations/cs", title: "C#", order: 25, group: Some("Implementations"), heading_offset: -1 },
    Page { slug: "kt", file: "implementations/kt", title: "Kotlin", order: 26, group: Some("Implementations"), heading_offset: -1 },
    Page { slug: "rs", file: "implementations/rs", title: "Rust", order: 27, group: Some("Implementations"), heading_offset: -1 },
    Page { slug: "go", file: "implementations/go", title: "Go", order: 28, group: Some("Implementations"), heading_offset: -1 },
    // Auxiliary
    Page { slug: "avg-spread", file: "avg-spread/avg-spread", title: "AvgSpread", order: 29, group: Some("Auxiliary"), heading_offset: -1 },
    Page { slug: "avg-spread-bounds", file: "avg-spread-bounds/avg-spread-bounds", title: "AvgSpreadBounds", order: 30, group: Some("Auxiliary"), heading_offset: -1 },
    Page { slug: "median", file: "median/median", title: "Median", order: 31, group: Some("Auxiliary"), heading_offset: -1 },
    Page { slug: "sign-margin", file: "sign-margin/sign-margin", title: "SignMargin", order: 32, group: Some("Auxiliary"), heading_offset: -1 },
    Page { slug: "pairwise-margin", file: "pairwise-margin/pairwise-margin", title: "PairwiseMargin", order: 33, group: Some("Auxiliary"), heading_offset: -1 },
    Page { slug: "signed-rank-margin", file: "signed-rank-margin/signed-rank-margin", title: "SignedRankMargin", order: 34, group: Some("Auxiliary"), heading_offset: -1 },
    // Appendix
    Page { slug: "assumptions", file: "assumptions/assumptions", title: "Assumptions", order: 35, group: Some("Appendix"), heading_offset: 0 },
    Page { slug: "foundations", file: "foundations/foundations", title: "Foundations", order: 36, group: Some("Appendix"), heading_offset: 0 },
    Page { slug: "methodology", file: "methodology/methodology", title: "Methodology", order: 37, group: Some("Appendix"), heading_offset: 0 },
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
    let index_mdx = astro::generate_index_page(&abstract_content, PAGES);
    std::fs::write(web_content_path.join("index.mdx"), index_mdx)?;
    println!("  Generated: web/src/content/manual/index.mdx");

    // Create cross-reference map for internal links
    let xref_map = xref::XRefMap::new();

    // Generate each page and collect used citations
    let mut used_citations = std::collections::HashSet::new();
    for page in PAGES {
        let typ_path = manual_path.join(format!("{}.typ", page.file));
        let content = typst_parser::parse_typst_document(&typ_path, base_path)?;
        used_citations.extend(content.extract_citations());
        let mdx_content = astro::convert_typst_to_mdx(
            &content,
            &definitions,
            &references,
            &xref_map,
            page.title,
            page.order,
            page.group,
            page.heading_offset,
        );

        let output_file = format!("{}.mdx", page.slug);
        std::fs::write(web_content_path.join(&output_file), mdx_content)?;
        println!("  Generated: web/src/content/manual/{output_file}");
    }

    // Generate bibliography page (only includes actually used references)
    let bibliography_mdx =
        astro::generate_bibliography_page(&references, &used_citations, 50, Some("Appendix"));
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
    let colophon_mdx = astro::generate_colophon_page(&colophon_info, 51, Some("Appendix"));
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

    // Clean page MDX files
    for page in PAGES {
        let file = format!("web/src/content/manual/{}.mdx", page.slug);
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
