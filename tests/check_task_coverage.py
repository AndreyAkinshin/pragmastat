#!/usr/bin/env python3
"""Fail when a task exists that its aggregate does not reach, or that the workflow never runs.

Every language pipeline is spelled twice: once as a task, once as an entry in the aggregate that
runs it. Nothing tied the two together, so adding a task and forgetting the aggregate produced a
check that runs nowhere, and looked exactly like a check that passes.

That is not hypothetical. `docs:check`, `tools:check` and `rs:sim:check` existed and were green
and were reachable from `ci` alone, so `mise run check` (the documented pre-commit entry point)
did not run them and only GitHub did. `tools:test`, covering the Typst to MDX converter whose
output ships on the website, belonged to no aggregate at all: its tests ran when someone typed the
task name and never otherwise.

The rule is containment by suffix. A task named <something>:check belongs in `check`, :test in
`test`, :ci in `ci`, :check:fix in `check:fix`, and anything under tests:check: is a repository
guard, which belongs in `ci`. Exemptions are named individually below, so the set this reads is
everything minus a short list rather than a list that has to be kept complete.

The second direction is the workflow. `.github/workflows/ci.yml` does not run `mise run ci`; it
names each guard in its own step, so a guard added to the ci aggregate reaches every developer and
no pull request. This check found exactly that on its first run: tests:check:task-coverage, added
one commit earlier, was in the aggregate and in no job.
"""

import re
import sys
import tomllib
from pathlib import Path

WORKFLOW = Path(".github/workflows/ci.yml")

# Guards the workflow deliberately does not run, each with the reason. Empty today.
NOT_IN_THE_WORKFLOW: dict[str, str] = {}

# suffix -> the aggregate that must depend on every task carrying it.
# Order matters: ":check:fix" is tested before ":check", because a name ending in the former also
# ends in the latter.
AGGREGATES = [
    (":check:fix", "check:fix"),
    (":check", "check"),
    (":test", "test"),
    (":ci", "ci"),
]

# Every tests:check:* task is a repository guard and belongs in ci regardless of its suffix.
GUARD_PREFIX = "tests:check:"
GUARD_AGGREGATE = "ci"

# Tasks that deliberately belong to no aggregate, each with the reason. Empty today, and meant to
# stay that way: a task worth having is a task worth running.
EXEMPT: dict[str, str] = {}

# How many tasks the file declares. Pinned because this check reports success by finding no gap,
# and finding no gap is also what happens when the parse returns nothing.
EXPECTED_TASKS = 125

# How many of those tasks a rule above actually claims. Pinned separately, because the count of
# tasks read and the count of tasks judged are independent: emptying AGGREGATES leaves the first
# untouched and drops the second to zero, and without this the check stayed green on exactly that
# mutation. A guard that proves it read something still has to prove it can refuse something.
EXPECTED_CHECKED = 44


def main() -> int:
    root = Path(sys.argv[1] if len(sys.argv) > 1 else ".").resolve()
    manifest = root / "mise.toml"
    if not manifest.is_file():
        print(f"no mise.toml under {root}", file=sys.stderr)
        return 1
    tasks = tomllib.loads(manifest.read_text()).get("tasks", {})

    if not tasks:
        print(f"no tasks parsed from {manifest}", file=sys.stderr)
        return 1

    findings = []
    checked = 0
    for name in sorted(tasks):
        if name in EXEMPT:
            continue

        if name.startswith(GUARD_PREFIX):
            wanted = GUARD_AGGREGATE
        else:
            wanted = next((agg for suffix, agg in AGGREGATES if name.endswith(suffix)), None)
            if wanted is None or name == wanted:
                continue

        checked += 1
        depends = tasks.get(wanted, {}).get("depends", [])
        if name not in depends:
            findings.append((name, wanted))

    if findings:
        print("tasks that no aggregate reaches:", file=sys.stderr)
        for name, wanted in findings:
            print(f"  {name} is not in {wanted}", file=sys.stderr)
        print("add it to the aggregate, or name it in EXEMPT with a reason", file=sys.stderr)
        return 1

    stale = sorted(set(EXEMPT) - set(tasks))
    if stale:
        print(f"EXEMPT names tasks that no longer exist: {', '.join(stale)}", file=sys.stderr)
        return 1

    if len(tasks) != EXPECTED_TASKS:
        print(f"mise.toml declares {len(tasks)} tasks, expected {EXPECTED_TASKS}", file=sys.stderr)
        print("re-measure and update EXPECTED_TASKS if the change is intended", file=sys.stderr)
        return 1

    if checked != EXPECTED_CHECKED:
        print(f"a rule claimed {checked} tasks, expected {EXPECTED_CHECKED}", file=sys.stderr)
        print("re-measure and update EXPECTED_CHECKED if the change is intended", file=sys.stderr)
        return 1

    workflow_path = root / WORKFLOW
    if not workflow_path.is_file():
        print(f"no workflow at {workflow_path}", file=sys.stderr)
        return 1
    invoked = set(re.findall(r"mise run ([\w:-]+)", workflow_path.read_text()))
    if not invoked:
        print(f"{WORKFLOW} invokes no mise task at all", file=sys.stderr)
        return 1

    guards = {n for n in tasks if n.startswith(GUARD_PREFIX)}
    unrun = sorted(guards - invoked - set(NOT_IN_THE_WORKFLOW))
    if unrun:
        print(f"guards the workflow never runs, so they gate no pull request:", file=sys.stderr)
        for name in unrun:
            print(f"  {name}", file=sys.stderr)
        print(f"add a step to {WORKFLOW}, or name it in NOT_IN_THE_WORKFLOW with a reason", file=sys.stderr)
        return 1

    unknown = sorted(n for n in invoked if n.startswith(GUARD_PREFIX) and n not in tasks)
    if unknown:
        print(f"{WORKFLOW} runs guards that mise.toml does not declare:", file=sys.stderr)
        for name in unknown:
            print(f"  {name}", file=sys.stderr)
        return 1

    stale_workflow = sorted(set(NOT_IN_THE_WORKFLOW) - guards)
    if stale_workflow:
        print(f"NOT_IN_THE_WORKFLOW names guards that no longer exist: {', '.join(stale_workflow)}", file=sys.stderr)
        return 1

    print(
        f"every aggregate reaches its tasks: {checked} judged, {len(tasks)} declared; "
        f"{len(guards)} guards all run in the workflow"
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
