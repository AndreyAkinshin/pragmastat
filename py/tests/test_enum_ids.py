"""The canonical id of an enum member is the string every port emits and every fixture stores.

All four id-like types are StrEnum, so the member is its id and no accessor stands between the
two. That is what these pin: the exact strings, the fact that a member compares and formats as
one, and the rendered form of a Violation, which the fixtures reach only through an estimator
that happens to raise.
"""

import pytest

from pragmastat import AssumptionId, ComparisonVerdict, Metric, Subject
from pragmastat.assumptions import Violation


@pytest.mark.parametrize(
    ("family", "ids"),
    [
        (AssumptionId, ["validity", "domain", "positivity", "sparity", "distinctness", "cumulance"]),
        (Subject, ["x", "y", "misrate"]),
        (Metric, ["center", "spread", "shift", "ratio", "disparity"]),
        (ComparisonVerdict, ["less", "greater", "inconclusive"]),
    ],
)
def test_family_publishes_exactly_its_ids(family: type, ids: list[str]) -> None:
    assert [str(member) for member in family] == ids
    assert [family(i) for i in ids] == list(family)


@pytest.mark.parametrize("member", list(Subject) + list(AssumptionId) + list(Metric) + list(ComparisonVerdict))
def test_member_is_its_own_id(member: str) -> None:
    """A StrEnum member is the string, so both comparison and formatting reach the id."""
    assert member == member.value
    assert f"{member}" == member.value


def test_violation_renders_as_both_ids() -> None:
    violation = Violation(AssumptionId.SPARITY, Subject.Y)
    assert str(violation) == "sparity(y)"
