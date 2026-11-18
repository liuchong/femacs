"""Compare authored observations against their actual observable contracts.

The weak-table fixture deliberately permits conservative retention of its one
unreferenced entry. Its live-entry assertions and printed live observations
remain exact; no other authored language observation receives that latitude.
Raw observations are never rewritten by this module.
"""
import re

WEAK_ID = 'rtl08-weak-modes-after-explicit-full'
WEAK_ORDER = ('key', 'value', 'key-or-value', 'key-and-value')
WEAK_LIVE = ('((key (:value) 1) (value (:value) 1) '
             '(key-or-value (:value) 1) (key-and-value (:value) 1))')
WEAK_PATTERN = re.compile(re.escape('('+WEAK_LIVE+' ')
    + r'\(\(key \. ([01])\) \(value \. ([01])\) '
      r'\(key-or-value \. ([01])\) \(key-and-value \. ([01])\)\)\)')


def weak_counts(observations):
    """Return four permitted integer counts, or reject the entire structure."""
    if (not isinstance(observations, list) or len(observations) != 1
            or not isinstance(observations[0], list) or len(observations[0]) != 2
            or observations[0][0] != WEAK_ID
            or not isinstance(observations[0][1], str)
            or len(observations[0][1]) > 1024):
        return None
    match = WEAK_PATTERN.fullmatch(observations[0][1])
    return dict(zip(WEAK_ORDER, map(int, match.groups()))) if match else None


def compare(identity, reference, observed):
    """Keep exact equality except for the explicitly permitted weak counts."""
    literal_equal = observed == reference
    if identity != WEAK_ID:
        return dict(passed=bool(isinstance(observed, list) and observed
                                and isinstance(reference, list) and reference
                                and literal_equal),
                    contract='exact-language-observation-v1', literal_equal=literal_equal)
    left, right = weak_counts(reference), weak_counts(observed)
    return dict(passed=left is not None and right is not None,
                contract='conservative-weak-reachability-v1', literal_equal=literal_equal,
                reference_dead_counts=left, observed_dead_counts=right,
                live_observations_exact=left is not None and right is not None,
                raw_observations_modified=False)
