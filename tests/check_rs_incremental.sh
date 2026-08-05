#!/usr/bin/env bash
#
# Packaging must not answer for the build that follows it.
#
# `cargo package` verifies the archive by compiling the extracted copy under
# target/package/pragmastat-<version>/. Done in the shared target directory, that compilation
# lands under the same unit fingerprint as the ordinary build, and the dep-info deciding freshness
# is left naming the snapshot's paths. Every later `cargo build` then compares against those
# instead of against src/ and reports Fresh: a deliberately broken src/compare.rs built clean and
# the test run used the previous binary. rs:ci begins with rs:clean, so CI never saw it, and every
# local build after a packaging run was answering about a copy.
#
# This asks the question behaviorally rather than by reading dep-info paths: package, break a
# source file into one that cannot compile, and require the build to fail. A guard that inspected
# the fingerprint files would pass the moment cargo changed their layout while the hazard stayed.
#
# It reports on the tree it is given, not only on the configuration. A target/ already poisoned by
# an earlier packaging run fails this even once the task is fixed, which is the right answer and
# is cleared by `mise run rs:clean`. CI reaches it with no rs build behind it, so the question it
# asks there is about the task alone.

set -euo pipefail

crate_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/../rs/pragmastat" && pwd)"
victim="$crate_dir/src/compare.rs"
backup="$(mktemp)"

cleanup() {
  cp "$backup" "$victim"
  rm -f "$backup"
}
trap cleanup EXIT

cp "$victim" "$backup"

cd "$crate_dir"

echo "packaging (the step that used to poison the shared fingerprint)"
cargo package --quiet --allow-dirty --target-dir target/pack

printf '\nthis line cannot compile;\n' >>"$victim"

echo "building with a source file that cannot compile"
if cargo build --quiet 2>/dev/null; then
  echo "FAIL: the build succeeded over a source file that cannot compile." >&2
  echo "Packaging left the shared target directory answering about its own extracted copy." >&2
  exit 1
fi

echo "the build reads src/ after packaging"
