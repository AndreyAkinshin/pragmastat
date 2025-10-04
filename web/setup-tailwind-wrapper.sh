#!/usr/bin/env bash
# setup-tailwind-wrapper.sh
# macOS-only: rename ./.bin/tailwind -> ./.bin/tailwind-bin
# and install a wrapper that runs a clean cached copy with a clean TMPDIR.

set -euo pipefail

# --- macOS guard ---
if [[ "$(uname -s)" != "Darwin" ]]; then
  echo "This script is macOS-only." >&2
  exit 1
fi

repo_root="$(cd "$(dirname "$0")" && pwd)"
bin_dir="$repo_root/.bin"
orig="$bin_dir/tailwind"
real="$bin_dir/tailwind-bin"

[[ -d "$bin_dir" ]] || { echo "Missing $bin_dir" >&2; exit 1; }
[[ -x "$orig" || -x "$real" ]] || { echo "No tailwind binary at $orig or $real" >&2; exit 1; }

# --- Step 1: rename original to tailwind-bin (idempotent) ---
if [[ -x "$orig" ]]; then
  mv -f "$orig" "$real"
fi
chmod +x "$real"

# --- Step 2: install wrapper at ./.bin/tailwind ---
wrapper="$orig"
cat > "$wrapper" <<'WRAP'
#!/usr/bin/env bash
set -euo pipefail

# Clean, user-local cache OUTSIDE the repo (prevents provenance inheritance).
CACHE_ROOT="$HOME/Library/Caches/tailwind-cli-clean"
RUNTIME_BIN="$CACHE_ROOT/tailwind-bin"
RUNTIME_TMP="$CACHE_ROOT/tmp"

mkdir -p "$CACHE_ROOT" "$RUNTIME_TMP"

# Keep cache paths clean; cheap and idempotent.
xattr -dr com.apple.quarantine "$CACHE_ROOT" "$RUNTIME_TMP" 2>/dev/null || true
xattr -dr com.apple.provenance "$CACHE_ROOT" "$RUNTIME_TMP" 2>/dev/null || true

# Source binary lives next to this wrapper as tailwind-bin.
self_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
src_bin="$self_dir/tailwind-bin"

# Re-copy to cache when hash or size differs (strip xattrs on copy).
need_copy=1
if [[ -x "$RUNTIME_BIN" ]]; then
  if command -v shasum >/dev/null 2>&1; then
    src_hash="$(shasum -a 256 "$src_bin" | awk '{print $1}')"
    dst_hash="$(shasum -a 256 "$RUNTIME_BIN" | awk '{print $1}' || true)"
    [[ "${src_hash:-}" == "${dst_hash:-}" ]] && need_copy=0
  else
    [[ "$(stat -f%z "$src_bin")" == "$(stat -f%z "$RUNTIME_BIN")" ]] && need_copy=0
  fi
fi

if [[ $need_copy -eq 1 ]]; then
  /bin/cp -fX "$src_bin" "$RUNTIME_BIN"     # -X strips extended attrs
  chmod +x "$RUNTIME_BIN"
  # Ad-hoc sign the cached copy to avoid network notarization checks.
  codesign --force -s - --timestamp=none "$RUNTIME_BIN" 2>/dev/null || true
fi

# Warm-up: force extraction of native addon (lightningcss) into our clean TMPDIR.
env TMPDIR="$RUNTIME_TMP" "$RUNTIME_BIN" --help >/dev/null 2>&1 || true

# Clean & ad-hoc sign any extracted .node so GK won't re-scan them.
for f in "$RUNTIME_TMP"/.*.node; do
  [[ -e "$f" ]] || continue
  xattr -d com.apple.quarantine "$f" 2>/dev/null || true
  xattr -d com.apple.provenance "$f" 2>/dev/null || true
  codesign --force -s - --timestamp=none "$f" 2>/dev/null || true
done

# Run the clean binary with the clean TMPDIR.
exec env TMPDIR="$RUNTIME_TMP" "$RUNTIME_BIN" "$@"
WRAP

chmod +x "$wrapper"

echo "✔ Renamed ./.bin/tailwind -> ./.bin/tailwind-bin"
echo "✔ Installed fast wrapper at ./.bin/tailwind (uses clean cache & TMP)"
echo "Try: ./.bin/tailwind --help"
