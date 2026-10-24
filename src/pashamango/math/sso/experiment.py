"""Bounded exhaustive H2 baseline; not evidence for H3 structural advantage."""

from collections import Counter
import random
from typing import Iterator

from . import AffineCycle, fingerprints


def cycles_for_rule(modulus: int, multiplier: int, increment: int) -> Iterator[AffineCycle]:
    yielded: set[tuple[int, ...]] = set()
    for start in range(modulus):
        seen: dict[int, int] = {}
        path: list[int] = []
        x = start
        while x not in seen:
            seen[x] = len(path)
            path.append(x)
            x = (multiplier * x + increment) % modulus
        cycle = AffineCycle(modulus, multiplier, increment, tuple(path[seen[x]:])).canonical()
        if cycle.states not in yielded:
            yielded.add(cycle.states)
            yield cycle


def run_pilot(max_modulus: int = 8, threshold: int = 3, seed: int = 42) -> dict:
    if type(max_modulus) is not int or not 2 <= max_modulus <= 25:
        raise ValueError("max_modulus outside [2,25]")
    if type(threshold) is not int or not 0 <= threshold <= 256:
        raise ValueError("threshold outside [0,256]")
    if type(seed) is not int:
        raise ValueError("seed must be an integer")
    candidates = [
        o for m in range(2, max_modulus + 1)
        for a in range(m) for b in range(m) for o in cycles_for_rule(m, a, b)
    ]
    random.Random(seed).shuffle(candidates)
    counts: Counter[tuple[int, int]] = Counter()
    single = double = joint = 0
    for obj in candidates:
        fp = fingerprints(obj)
        k1, k2 = fp["leading_zeros_sha256"], fp["leading_zeros_double_sha256"]
        counts[(k1, k2)] += 1
        h1, h2 = k1 >= threshold, k2 >= threshold
        single += h1
        double += h2
        joint += h1 and h2
    n = len(candidates)
    return {
        "format": "SSO-PILOT/1", "status": "observed",
        "max_modulus": max_modulus, "threshold": threshold, "seed": seed,
        "accepted_objects": n, "scientific_verifications": n, "sha256_calls": 2 * n,
        "hits_single": single, "hits_double": double, "hits_joint": joint,
        "expected_single_ideal_uniform": n * 2.0 ** (-threshold),
        "expected_double_ideal_uniform": n * 2.0 ** (-threshold),
        "measured_layers": {f"{a}:{b}": v for (a, b), v in sorted(counts.items())},
        "limitation": "No independence assumption, no H3 comparison, no semantic reward.",
    }
