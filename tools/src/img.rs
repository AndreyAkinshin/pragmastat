//! Image generation module for converting SVG logo to PNG variants.

use anyhow::{Context, Result};
use std::path::Path;

/// Logo output specifications
struct LogoSpec {
    name: &'static str,
    size: u32,
}

const LOGO_SPECS: &[LogoSpec] = &[
    LogoSpec {
        name: "logo.png",
        size: 800,
    },
    LogoSpec {
        name: "apple-touch-icon.png",
        size: 180,
    },
    LogoSpec {
        name: "favicon-32.png",
        size: 32,
    },
];

const ICO_SIZES: &[u32] = &[256, 128, 64, 48, 32, 16];

/// Generate all logo variants from SVG source.
pub fn generate_logo(base_path: &Path) -> Result<()> {
    let img_path = base_path.join("img");
    let svg_path = img_path.join("logo.svg");

    // Load and parse SVG
    let svg_data = std::fs::read(&svg_path)
        .with_context(|| format!("Failed to read SVG: {}", svg_path.display()))?;

    let tree = resvg::usvg::Tree::from_data(&svg_data, &resvg::usvg::Options::default())
        .context("Failed to parse SVG")?;

    // Generate PNG variants
    for spec in LOGO_SPECS {
        let output_path = img_path.join(spec.name);
        render_png(&tree, spec.size, &output_path)?;
        println!("  Generated: img/{}", spec.name);
    }

    // Generate ICO with multiple sizes
    let ico_path = img_path.join("logo.ico");
    generate_ico(&tree, &ico_path)?;
    println!("  Generated: img/logo.ico");

    Ok(())
}

/// Render SVG tree to PNG at specified size.
fn render_png(tree: &resvg::usvg::Tree, size: u32, output_path: &Path) -> Result<()> {
    let pixmap = render_to_pixmap(tree, size)?;

    // Encode as PNG
    let mut png_data = Vec::new();
    {
        let mut encoder = png::Encoder::new(&mut png_data, size, size);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header()?;
        writer.write_image_data(pixmap.data())?;
    }

    std::fs::write(output_path, &png_data)
        .with_context(|| format!("Failed to write PNG: {}", output_path.display()))?;

    Ok(())
}

/// Render SVG tree to pixmap at specified size.
fn render_to_pixmap(tree: &resvg::usvg::Tree, size: u32) -> Result<resvg::tiny_skia::Pixmap> {
    let mut pixmap =
        resvg::tiny_skia::Pixmap::new(size, size).context("Failed to create pixmap")?;

    let svg_size = tree.size();
    // Precision loss acceptable for image dimension calculations
    #[expect(clippy::cast_precision_loss)]
    let scale = size as f32 / svg_size.width().max(svg_size.height());

    let transform = resvg::tiny_skia::Transform::from_scale(scale, scale);
    resvg::render(tree, transform, &mut pixmap.as_mut());

    Ok(pixmap)
}

/// Generate ICO file with multiple sizes.
#[expect(clippy::cast_possible_truncation)] // ICO format requires specific byte sizes
fn generate_ico(tree: &resvg::usvg::Tree, output_path: &Path) -> Result<()> {
    let mut ico_data = Vec::new();

    // ICO header
    ico_data.extend_from_slice(&[0, 0]); // Reserved
    ico_data.extend_from_slice(&[1, 0]); // Type: 1 = ICO
    ico_data.extend_from_slice(&(ICO_SIZES.len() as u16).to_le_bytes()); // Number of images

    // Collect PNG data for each size
    let mut png_images: Vec<Vec<u8>> = Vec::new();
    for &size in ICO_SIZES {
        let pixmap = render_to_pixmap(tree, size)?;

        let mut png_data = Vec::new();
        {
            let mut encoder = png::Encoder::new(&mut png_data, size, size);
            encoder.set_color(png::ColorType::Rgba);
            encoder.set_depth(png::BitDepth::Eight);
            let mut writer = encoder.write_header()?;
            writer.write_image_data(pixmap.data())?;
        }
        png_images.push(png_data);
    }

    // Calculate offsets (header + directory entries)
    let header_size = 6 + ICO_SIZES.len() * 16;
    let mut offset = header_size;

    // Write directory entries
    for (i, &size) in ICO_SIZES.iter().enumerate() {
        let png_size = png_images[i].len();

        // Width (0 means 256)
        ico_data.push(if size == 256 { 0 } else { size as u8 });
        // Height (0 means 256)
        ico_data.push(if size == 256 { 0 } else { size as u8 });
        ico_data.push(0); // Color palette
        ico_data.push(0); // Reserved
        ico_data.extend_from_slice(&[1, 0]); // Color planes
        ico_data.extend_from_slice(&[32, 0]); // Bits per pixel
        ico_data.extend_from_slice(&(png_size as u32).to_le_bytes()); // Image size
        ico_data.extend_from_slice(&(offset as u32).to_le_bytes()); // Offset

        offset += png_size;
    }

    // Write image data
    for png_data in &png_images {
        ico_data.extend_from_slice(png_data);
    }

    std::fs::write(output_path, &ico_data)
        .with_context(|| format!("Failed to write ICO: {}", output_path.display()))?;

    Ok(())
}
