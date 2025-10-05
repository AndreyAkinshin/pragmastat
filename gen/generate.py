#!/usr/bin/env python3
"""
Pragmastat content generation script.
Converts markdown content for web and PDF, unifies versions, and generates documentation.
"""

import argparse
import os
import re
import shutil
from datetime import datetime
from pathlib import Path
from typing import Dict, List, Optional, Set


# ANSI color codes
class Colors:
    GREEN = '\033[92m'
    YELLOW = '\033[93m'
    RED = '\033[91m'
    GREY = '\033[90m'
    RESET = '\033[0m'


def log(msg: str):
    """Regular log message in default color."""
    print(msg)


def log_success(msg: str):
    """Success message in green."""
    print(f"{Colors.GREEN}{msg}{Colors.RESET}")


def log_unchanged(msg: str):
    """Unchanged file message in grey."""
    print(f"{Colors.GREY}{msg}{Colors.RESET}")


def warning(msg: str):
    print(f"{Colors.YELLOW}WARNING: {msg}{Colors.RESET}")


def error(msg: str):
    print(f"{Colors.RED}ERROR: {msg}{Colors.RESET}")


def replace_with_warning(content: str, placeholder: str, replacement: str, context: str = "") -> str:
    """Replace placeholder in content, warn if not found."""
    if placeholder not in content:
        context_info = f" in {context}" if context else ""
        warning(f"Placeholder '{placeholder}' not found{context_info}")
        return content
    return content.replace(placeholder, replacement)


def write_file_if_changed(file_path: Path, content: str, description: str = None) -> bool:
    """Write file only if content changed. Returns True if file was written."""
    file_path.parent.mkdir(parents=True, exist_ok=True)

    desc = description or str(file_path)

    # Check if file exists and has same content
    if file_path.exists():
        existing_content = file_path.read_text(encoding='utf-8')
        if existing_content == content:
            log_unchanged(f"[gn] {desc}")
            return False

    # Write file
    file_path.write_text(content, encoding='utf-8')
    log_success(f"[gn] {desc}")
    return True


def copy_file_if_changed(source_path: Path, dest_path: Path, description: str = None) -> bool:
    """Copy file only if content changed. Returns True if file was copied."""
    dest_path.parent.mkdir(parents=True, exist_ok=True)

    desc = description or str(dest_path)

    # Check if destination exists and has same content as source
    if dest_path.exists():
        source_content = source_path.read_bytes()
        dest_content = dest_path.read_bytes()
        if source_content == dest_content:
            log_unchanged(f"[cp] {desc}")
            return False

    # Copy file
    shutil.copy2(source_path, dest_path)
    log_success(f"[cp] {desc}")
    return True


# ============================================================================
# Path Constants
# ============================================================================

class Paths:
    def __init__(self, root: Path):
        self.root = root

        # Project Structure
        self.root_marker = 'CITATION.cff'
        self.version_file = root / 'manual' / 'version.txt'

        # Source Files
        self.source_main = root / 'manual' / 'main.md'
        self.source_abstract = root / 'manual' / 'abstract.md'
        self.source_references_dir = root / 'manual' / 'references'
        self.source_images_dir = root / 'img'

        # Web Output
        self.web_index = root / 'web' / 'content' / '_index.md'
        self.web_references_dir = root / 'web' / 'content' / 'references'
        self.web_references_index = root / 'web' / 'content' / 'references' / '_index.md'
        self.web_images_dir = root / 'web' / 'content' / 'img'
        self.web_static_dir = root / 'web' / 'static'
        self.web_static_pragmastat_md = root / 'web' / 'static' / 'pragmastat.md'

        # PDF Output
        self.pdf_definitions = root / 'pdf' / 'tex' / 'definitions.tex'
        self.pdf_markdown = root / 'pdf' / 'pragmastat.md'
        self.pdf_template = root / 'pdf' / 'pragmastat.md.template'
        self.pdf_references = root / 'pdf' / 'references.bib'
        self.pdf_images_dir = root / 'pdf' / 'img'

        # Documentation Templates
        self.doc_template_impl = root / 'manual' / 'implementations' / 'template-impl.md'
        self.doc_template_readme = root / 'manual' / 'implementations' / 'template-readme.md'
        self.doc_impl_dir = root / 'manual' / 'implementations'

    def install_path(self, slug: str) -> Path:
        return self.root / 'manual' / 'implementations' / f'install-{slug}.md'


# ============================================================================
# Language Configurations
# ============================================================================

class Language:
    def __init__(self, slug: str, title: str, code_language: str, demo_path: str,
                 readme_path: str, package_url: str, version_file: Optional[str] = None,
                 version_pattern: Optional[str] = None, version_replace: Optional[str] = None,
                 version_file_2: Optional[str] = None, version_pattern_2: Optional[str] = None,
                 version_replace_2: Optional[str] = None):
        self.slug = slug
        self.title = title
        self.code_language = code_language
        self.demo_path = demo_path
        self.readme_path = readme_path
        self.package_url = package_url
        self.version_file = version_file
        self.version_pattern = version_pattern
        self.version_replace = version_replace
        self.version_file_2 = version_file_2
        self.version_pattern_2 = version_pattern_2
        self.version_replace_2 = version_replace_2


LANGUAGES = [
    Language(
        slug='dotnet',
        title='.NET',
        code_language='cs',
        demo_path='dotnet/Pragmastat.Demo/Program.cs',
        readme_path='dotnet/README.md',
        package_url='Pragmastat on NuGet: https://www.nuget.org/packages/Pragmastat/',
        version_file='dotnet/Directory.Build.props',
        version_pattern=r'<Version>.*?</Version>',
        version_replace='<Version>{version}</Version>'
    ),
    Language(
        slug='go',
        title='Go',
        code_language='go',
        demo_path='go/example/main.go',
        readme_path='go/README.md',
        package_url=''
    ),
    Language(
        slug='kotlin',
        title='Kotlin',
        code_language='kotlin',
        demo_path='kotlin/src/main/kotlin/dev/pragmastat/example/Main.kt',
        readme_path='kotlin/README.md',
        package_url='Pragmastat on Maven Central Repository: https://central.sonatype.com/artifact/dev.pragmastat/pragmastat/overview',
        version_file='kotlin/build.gradle.kts',
        version_pattern=r'version = ".*?"',
        version_replace='version = "{version}"'
    ),
    Language(
        slug='python',
        title='Python',
        code_language='python',
        demo_path='python/examples/demo.py',
        readme_path='python/README.md',
        package_url='Pragmastat on PyPI: https://pypi.org/project/pragmastat/',
        version_file='python/pyproject.toml',
        version_pattern=r'version = ".*?"',
        version_replace='version = "{version}"',
        version_file_2='python/pragmastat/__init__.py',
        version_pattern_2=r"__version__ = '.*?'",
        version_replace_2="__version__ = '{version}'"
    ),
    Language(
        slug='r',
        title='R',
        code_language='r',
        demo_path='r/pragmastat/inst/examples/demo.R',
        readme_path='r/pragmastat/README.md',
        package_url='',
        version_file='r/pragmastat/DESCRIPTION',
        version_pattern=r'Version: .*',
        version_replace='Version: {version}'
    ),
    Language(
        slug='rust',
        title='Rust',
        code_language='rust',
        demo_path='rust/pragmastat/examples/demo.rs',
        readme_path='rust/pragmastat/README.md',
        package_url='Pragmastat on crates.io: https://crates.io/crates/pragmastat',
        version_file='rust/pragmastat/Cargo.toml',
        version_pattern=r'(name = "pragmastat"[\s\S]*?)version = ".*?"',
        version_replace=r'\1version = "{version}"'
    ),
    Language(
        slug='ts',
        title='TypeScript',
        code_language='typescript',
        demo_path='ts/examples/demo.ts',
        readme_path='ts/README.md',
        package_url='Pragmastat on npm: https://www.npmjs.com/package/pragmastat',
        version_file='ts/package.json',
        version_pattern=r'"version": ".*?"',
        version_replace='"version": "{version}"'
    )
]


# ============================================================================
# Content Conversion Functions
# ============================================================================

class ContentConverter:
    def __init__(self, paths: Paths, version: str):
        self.paths = paths
        self.version = version
        self.include_stack: Set[str] = set()

    def convert_text(self, content: str, mode: str) -> str:
        """Apply transformations based on mode ('web' or 'pdf')."""
        # 1. Process INCLUDE directives (recursive)
        content = self._process_includes(content)

        # 2. Process IMG directives
        content = self._process_img(content, mode)

        # 3. Process BEGIN/END directives
        content = self._process_begin_end(content, mode)

        # 4. Process COPYRIGHT directive
        content = self._process_copyright(content)

        # 5. Header numbering (web only)
        if mode == 'web':
            content = self._number_headers(content)

        # 6. Delete markers
        content = self._delete_markers(content, mode)

        return content

    def _process_includes(self, content: str) -> str:
        """Process INCLUDE directives recursively."""
        pattern = re.compile(r'<!--\s*INCLUDE\s+([^>]+)\s*-->', re.IGNORECASE)

        def replace_include(match):
            include_path = match.group(1).strip()
            full_path = self.paths.root / include_path

            # Check for circular includes
            path_str = str(full_path)
            if path_str in self.include_stack:
                warning(f"Circular inclusion detected: {include_path}")
                return f"<!-- ERROR | INCLUDE {include_path} | CIRCULAR INCLUSION DETECTED -->"

            # Check if file exists
            if not full_path.exists():
                warning(f"Include file not found: {include_path}")
                return f"<!-- ERROR | INCLUDE {include_path} | FILE NOT FOUND -->"

            # Read and process file
            self.include_stack.add(path_str)
            try:
                included_content = full_path.read_text(encoding='utf-8').strip()
                # Recursively process includes in the included file
                included_content = self._process_includes(included_content)
                return included_content
            finally:
                self.include_stack.discard(path_str)

        # Keep replacing until no more includes
        prev_content = None
        while prev_content != content:
            prev_content = content
            content = pattern.sub(replace_include, content)

        return content

    def _process_img(self, content: str, mode: str) -> str:
        """Process IMG directives."""
        pattern = re.compile(r'<!--\s*IMG\s+([^>]+)\s*-->', re.IGNORECASE)

        def replace_img(match):
            img_name = match.group(1).strip()
            if mode == 'web':
                return f'{{{{< img {img_name} >}}}}'
            else:  # pdf
                return f'![](img/{img_name}.png){{width=100%}}\n'

        return pattern.sub(replace_img, content)

    def _process_begin_end(self, content: str, mode: str) -> str:
        """Process BEGIN/END directives."""
        if mode == 'web':
            content = re.sub(r'<!--\s*BEGIN\s+([^>]+)\s*-->', r'<div class="\1">', content, flags=re.IGNORECASE)
            content = re.sub(r'<!--\s*END\s+([^>]+)\s*-->', r'</div>', content, flags=re.IGNORECASE)
        else:  # pdf
            content = re.sub(r'<!--\s*BEGIN\s+([^>]+)\s*-->', r'::: {.\1 data-latex=""}\n', content, flags=re.IGNORECASE)
            content = re.sub(r'<!--\s*END\s+([^>]+)\s*-->', r':::', content, flags=re.IGNORECASE)

        return content

    def _process_copyright(self, content: str) -> str:
        """Process COPYRIGHT directive."""
        replacement = f"Pragmastat v{self.version} (c) 2025 Andrey Akinshin, MIT License"
        return re.sub(r'<!--\s*COPYRIGHT\s*-->', replacement, content, flags=re.IGNORECASE)

    def _number_headers(self, content: str) -> str:
        """Number markdown headers (web mode only)."""
        lines = content.split('\n')
        counters = [0, 0, 0, 0, 0, 0]  # H1-H6
        result = []

        for line in lines:
            # Check if line is a header
            match = re.match(r'^(#{1,6})\s+(.+)$', line)
            if match:
                hashes = match.group(1)
                header_text = match.group(2)
                level = len(hashes) - 1  # 0-indexed

                # Skip if already numbered with §
                if header_text.startswith('§'):
                    result.append(line)
                    continue

                # Increment counter at this level
                counters[level] += 1

                # Reset deeper levels
                for i in range(level + 1, 6):
                    counters[i] = 0

                # Build section number
                section_parts = [str(counters[i]) for i in range(level + 1) if counters[i] > 0]
                section_number = '.'.join(section_parts)

                # Format new header
                new_line = f"{hashes} §{section_number}. {header_text}"
                result.append(new_line)
            else:
                result.append(line)

        return '\n'.join(result)

    def _delete_markers(self, content: str, mode: str) -> str:
        """Delete mode-specific markers."""
        lines = content.split('\n')

        if mode == 'web':
            marker = '<!-- [web] DELETE -->'
        elif mode == 'pdf':
            marker = '<!-- [pdf] DELETE -->'
        elif mode == 'md':
            marker = '<!-- [md] DELETE -->'
        else:
            marker = None

        if marker:
            filtered_lines = [line for line in lines if marker not in line]
            return '\n'.join(filtered_lines)
        return content


# ============================================================================
# Resource Processing Functions
# ============================================================================

def process_references(paths: Paths, mode: str):
    """Process reference files."""
    if mode == 'web':
        # Create references directory
        paths.web_references_dir.mkdir(parents=True, exist_ok=True)

        # Copy all files from source to web
        for file_path in paths.source_references_dir.iterdir():
            if file_path.is_file():
                dest_path = paths.web_references_dir / file_path.name
                relative_dest = dest_path.relative_to(paths.root)
                copy_file_if_changed(file_path, dest_path, str(relative_dest))

        # Create index file
        index_content = "---\ntitle: References\n---"
        write_file_if_changed(paths.web_references_index, index_content, "web/content/references/_index.md")

    else:  # pdf
        # Accumulate BibTeX from all reference files
        bibtex_content = []

        for file_path in sorted(paths.source_references_dir.iterdir()):
            if file_path.is_file():
                content = file_path.read_text(encoding='utf-8')
                lines = content.split('\n')

                inside_bib = False
                for line in lines:
                    trimmed = line.strip()
                    if trimmed == '```bib':
                        inside_bib = True
                    elif trimmed == '```' and inside_bib:
                        inside_bib = False
                    elif inside_bib:
                        bibtex_content.append(line)

        # Write accumulated BibTeX
        final_bibtex = '\n'.join(bibtex_content) + '\n'
        write_file_if_changed(paths.pdf_references, final_bibtex, "pdf/references.bib")


def process_images(paths: Paths, mode: str):
    """Process image files."""
    if mode == 'web':
        # Ensure directory exists
        paths.web_images_dir.mkdir(parents=True, exist_ok=True)

        # Copy all PNG files
        for file_path in paths.source_images_dir.glob('*.png'):
            dest_path = paths.web_images_dir / file_path.name
            relative_dest = dest_path.relative_to(paths.root)
            copy_file_if_changed(file_path, dest_path, str(relative_dest))

    else:  # pdf
        # Ensure directory exists
        paths.pdf_images_dir.mkdir(parents=True, exist_ok=True)

        # Copy only *_light.png files, removing suffix
        for file_path in paths.source_images_dir.glob('*.png'):
            if file_path.stem.endswith('_light'):
                new_name = file_path.stem[:-6] + '.png'  # Remove '_light'
                dest_path = paths.pdf_images_dir / new_name
                relative_dest = dest_path.relative_to(paths.root)
                copy_file_if_changed(file_path, dest_path, str(relative_dest))


# ============================================================================
# Operation 1: Web Content Conversion
# ============================================================================

def convert_web_content(paths: Paths, version: str, is_release: bool):
    """Convert main content to web format."""
    log("=== Converting content for web ===")

    converter = ContentConverter(paths, version)

    # Read source
    content = paths.source_main.read_text(encoding='utf-8')

    # First pass
    content = converter.convert_text(content, 'web')

    # Replace placeholder
    definitions = paths.pdf_definitions.read_text(encoding='utf-8').strip()
    abstract = paths.source_abstract.read_text(encoding='utf-8')

    replacement = f'<div style="display: none;">\n$$\n{definitions}\n$$\n</div>\n\n{abstract}'
    content = replace_with_warning(content, '<!-- PLACEHOLDER Start -->', replacement, 'web content')

    # Second pass
    converter.include_stack.clear()  # Reset include stack
    content = converter.convert_text(content, 'web')

    # Write output
    write_file_if_changed(paths.web_index, content, 'web/_index.md')

    # Process references and images
    process_references(paths, 'web')
    process_images(paths, 'web')


def convert_md_content(paths: Paths, version: str, is_release: bool):
    """Convert main content to standalone markdown format."""
    log("=== Converting content for standalone markdown ===")

    converter = ContentConverter(paths, version)

    # Read source
    content = paths.source_main.read_text(encoding='utf-8')

    # First pass
    content = converter.convert_text(content, 'md')

    # Replace placeholder (same as web, but without version line)
    definitions = paths.pdf_definitions.read_text(encoding='utf-8').strip()
    abstract = paths.source_abstract.read_text(encoding='utf-8')

    replacement = f'<div style="display: none;">\n$$\n{definitions}\n$$\n</div>\n\n{abstract}'
    content = replace_with_warning(content, '<!-- PLACEHOLDER Start -->', replacement, 'md content')

    # Second pass
    converter.include_stack.clear()  # Reset include stack
    content = converter.convert_text(content, 'md')

    # Insert version into YAML frontmatter
    lines = content.split('\n')
    if len(lines) >= 3 and lines[0] == '---':
        # Find the title line and insert version after it
        for i in range(1, len(lines)):
            if lines[i].startswith('title:'):
                lines.insert(i + 1, f'version: {version}')
                break
        content = '\n'.join(lines)

    # Write to static directory
    write_file_if_changed(paths.web_static_pragmastat_md, content, 'web/static/pragmastat.md')


# ============================================================================
# Operation 2: PDF Content Conversion
# ============================================================================

def convert_pdf_content(paths: Paths, version: str, is_release: bool):
    """Convert main content to PDF format."""
    log("=== Converting content for PDF ===")

    converter = ContentConverter(paths, version)

    # Read source
    content = paths.source_main.read_text(encoding='utf-8')

    # Apply transformations
    content = converter.convert_text(content, 'pdf')

    # Replace placeholder
    content = replace_with_warning(content, '<!-- PLACEHOLDER Start -->', '\\clearpage', 'PDF content')

    # Remove YAML frontmatter
    lines = content.split('\n')
    separator_count = 0
    start_index = 0

    for i, line in enumerate(lines):
        if line.strip() == '---':
            separator_count += 1
            if separator_count == 2:
                start_index = i + 1
                break

    content = '\n'.join(lines[start_index:])

    # Process template
    template = paths.pdf_template.read_text(encoding='utf-8')
    abstract = paths.source_abstract.read_text(encoding='utf-8')

    # Process abstract (trim left, prepend 2 spaces)
    abstract_lines = [('  ' + line.lstrip()) for line in abstract.split('\n')]
    processed_abstract = '\n'.join(abstract_lines)

    template = replace_with_warning(template, '<!-- PLACEHOLDER Abstract -->', processed_abstract, 'PDF template')

    # Determine edition info
    if is_release:
        edition_info = f"Version {version}"
    else:
        current_date = datetime.now().strftime('%Y-%m-%d')
        edition_info = f"*Draft of Version {version} ({current_date})*"

    template = replace_with_warning(template, '<!-- PLACEHOLDER Version -->', edition_info, 'PDF template')

    # Concatenate template and content
    final_content = template + content

    # Write output
    write_file_if_changed(paths.pdf_markdown, final_content, 'pdf/pragmastat.md')

    # Process references and images
    process_references(paths, 'pdf')
    process_images(paths, 'pdf')


# ============================================================================
# Operation 3: Version Unification
# ============================================================================

def unify_versions(paths: Paths, version: str):
    """Update version strings in all language files."""
    log(f"=== Unifying versions to {version} ===")

    for lang in LANGUAGES:
        if not lang.version_file:
            continue

        file_path = paths.root / lang.version_file

        if not file_path.exists():
            warning(f"Version file not found: {file_path}")
            continue

        # Read file
        content = file_path.read_text(encoding='utf-8')

        # Apply regex replacement
        pattern = lang.version_pattern
        replacement = lang.version_replace.replace('{version}', version)
        updated_content = re.sub(pattern, replacement, content, flags=re.MULTILINE)

        # Compare and write if changed
        relative_path = file_path.relative_to(paths.root)
        if updated_content != content:
            file_path.write_text(updated_content, encoding='utf-8')
            log_success(f"[gn] {relative_path}")
        else:
            log_unchanged(f"[gn] {relative_path}")

        # Handle secondary version file (Python)
        if lang.version_file_2:
            file_path_2 = paths.root / lang.version_file_2

            if not file_path_2.exists():
                warning(f"Version file not found: {file_path_2}")
                continue

            content_2 = file_path_2.read_text(encoding='utf-8')
            pattern_2 = lang.version_pattern_2
            replacement_2 = lang.version_replace_2.replace('{version}', version)
            updated_content_2 = re.sub(pattern_2, replacement_2, content_2, flags=re.MULTILINE)

            relative_path_2 = file_path_2.relative_to(paths.root)
            if updated_content_2 != content_2:
                file_path_2.write_text(updated_content_2, encoding='utf-8')
                log_success(f"[gn] {relative_path_2}")
            else:
                log_unchanged(f"[gn] {relative_path_2}")


# ============================================================================
# Operation 4: Documentation Generation
# ============================================================================

def generate_documentation(paths: Paths, version: str):
    """Generate implementation docs and READMEs for all languages."""
    log("=== Generating documentation ===")

    template_impl = paths.doc_template_impl.read_text(encoding='utf-8')
    template_readme = paths.doc_template_readme.read_text(encoding='utf-8')

    for lang in LANGUAGES:

        # A. Generate Implementation Doc
        install_path = paths.install_path(lang.slug)
        install_content = install_path.read_text(encoding='utf-8')
        install_content = replace_with_warning(install_content, '$VERSION$', version, f'install-{lang.slug}.md')

        demo_path = paths.root / lang.demo_path
        demo_code = demo_path.read_text(encoding='utf-8').rstrip()

        impl_content = template_impl
        impl_content = replace_with_warning(impl_content, '$LANG_SLUG$', lang.slug, f'{lang.slug} implementation doc')
        impl_content = replace_with_warning(impl_content, '$LANG_TITLE$', lang.title, f'{lang.slug} implementation doc')
        impl_content = replace_with_warning(impl_content, '$LANG_CODE$', lang.code_language, f'{lang.slug} implementation doc')
        impl_content = replace_with_warning(impl_content, '$VERSION$', version, f'{lang.slug} implementation doc')
        impl_content = replace_with_warning(impl_content, '$INSTALL$', install_content.rstrip(), f'{lang.slug} implementation doc')
        impl_content = replace_with_warning(impl_content, '$DEMO$', demo_code, f'{lang.slug} implementation doc')
        impl_content = replace_with_warning(impl_content, '$PACKAGE$', lang.package_url, f'{lang.slug} implementation doc')

        impl_output_path = paths.doc_impl_dir / f"{lang.slug}.md"
        write_file_if_changed(impl_output_path, impl_content, f"manual/implementations/{lang.slug}.md")

        # B. Generate README
        readme_install_content = install_path.read_text(encoding='utf-8')
        readme_install_content = replace_with_warning(readme_install_content, '$VERSION$', version, f'install-{lang.slug}.md for README')

        readme_content = template_readme
        readme_content = replace_with_warning(readme_content, '$LANG_TITLE$', lang.title, f'{lang.slug} README')
        readme_content = replace_with_warning(readme_content, '$LANG_CODE$', lang.code_language, f'{lang.slug} README')
        readme_content = replace_with_warning(readme_content, '$VERSION$', version, f'{lang.slug} README')
        readme_content = replace_with_warning(readme_content, '$LANG_SLUG$', lang.slug, f'{lang.slug} README')
        readme_content = replace_with_warning(readme_content, '$INSTALL$', readme_install_content.rstrip(), f'{lang.slug} README')
        readme_content = replace_with_warning(readme_content, '$DEMO$', demo_code, f'{lang.slug} README')

        readme_output_path = paths.root / lang.readme_path
        relative_readme = readme_output_path.relative_to(paths.root)
        write_file_if_changed(readme_output_path, readme_content, str(relative_readme))


# ============================================================================
# Main Function
# ============================================================================

def find_project_root() -> Path:
    """Find project root by walking up until finding CITATION.cff."""
    current = Path(__file__).resolve().parent

    while current != current.parent:
        if (current / 'CITATION.cff').exists():
            return current
        current = current.parent

    raise RuntimeError("Could not find project root (CITATION.cff not found)")


def main():
    parser = argparse.ArgumentParser(description='Generate Pragmastat content')
    parser.add_argument('--release', action='store_true', help='Generate release version')
    args = parser.parse_args()

    print()  # Empty line for readability
    log("=" * 60)
    log("  PRAGMASTAT CONTENT GENERATOR")
    log("=" * 60)
    print()

    # Find project root
    root = find_project_root()
    log(f"Project root: {root}")

    # Initialize paths
    paths = Paths(root)

    # Read version
    version = paths.version_file.read_text(encoding='utf-8').strip()
    mode = "Release" if args.release else "Draft"
    log(f"Version: {version} ({mode})")
    print()

    # Execute operations in order
    generate_documentation(paths, version)
    print()
    convert_md_content(paths, version, args.release)
    print()
    convert_web_content(paths, version, args.release)
    print()
    convert_pdf_content(paths, version, args.release)
    print()
    unify_versions(paths, version)
    print()

    log("=" * 60)
    log("  COMPLETED SUCCESSFULLY")
    log("=" * 60)
    print()


if __name__ == '__main__':
    main()
