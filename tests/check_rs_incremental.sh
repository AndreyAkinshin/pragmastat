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
# This asks the question behaviorally rather than by reading dep-info paths: break a source file
# into one that cannot compile, and require the build to fail. A guard that inspected the
# fingerprint files would pass the moment cargo changed their layout while the hazard stayed.
#
# THE PACKAGING STEP IS NOT RUN HERE. mise runs the real rs:pack immediately before this script,
# and that separation is the whole point. This file used to call
# `cargo package --allow-dirty --target-dir target/pack` itself, and with the flag written here it
# was checking a property of cargo rather than the configuration of this repository: deleting
# --target-dir from rs:pack in mise.toml left this gate green, verified by mutation on 2026-08-08.
# A guard about how a task is configured has to run that task.
#
# It reports on the tree it is given, not only on the configuration. A target/ already poisoned by
# an earlier packaging run fails this even once the task is fixed, which is the right answer and
# is cleared by `mise run rs:clean`.

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

printf '\nthis line cannot compile;\n' >>"$victim"

echo "building with a source file that cannot compile"
if cargo build --quiet 2>/dev/null; then
  echo "FAIL: the build succeeded over a source file that cannot compile." >&2
  echo "Packaging left the shared target directory answering about its own extracted copy." >&2
  echo "Check that rs:pack still sends its verification build to a separate --target-dir." >&2
  exit 1
fi

echo "the build reads src/ after packaging"
