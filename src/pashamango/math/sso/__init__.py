"""SSO/1: verified finite cycles and SHA-256 rarity; experimental, not useful-PoW."""

from __future__ import annotations

from dataclasses import dataclass
import hashlib
import json
from typing import Any

__version__ = "0.1.0a0"


class ValidationError(ValueError):
    """A mathematical witness is invalid."""


def _integer(value: object, name: str, minimum: int = 0) -> int:
    if type(value) is not int or value < minimum:
        raise ValidationError(f"{name} must be an integer >= {minimum}")
    return value


def frame_fields(*fields: bytes) -> bytes:
    """Unambiguous length-prefixed tuple encoding, including arbitrary delimiters."""
    if len(fields) > 65535:
        raise ValueError("too many fields")
    result = [len(fields).to_bytes(2, "big")]
    for field in fields:
        if not isinstance(field, bytes):
            raise TypeError("framed fields must be bytes")
        result.extend((len(field).to_bytes(8, "big"), field))
    return b"".join(result)


@dataclass(frozen=True)
class AffineCycle:
    """Witness for f(x)=(a*x+b) mod m with primitive periodic orbit."""
    modulus: int
    multiplier: int
    increment: int
    states: tuple[int, ...]

    def canonical(self) -> "AffineCycle":
        m = _integer(self.modulus, "modulus", 2)
        a = _integer(self.multiplier, "multiplier") % m
        b = _integer(self.increment, "increment") % m
        if not isinstance(self.states, (list, tuple)):
            raise ValidationError("states must be a tuple or list")
        states = tuple(self.states)
        if not states or len(states) > m:
            raise ValidationError("cycle length outside [1, modulus]")
        if any(type(x) is not int or not 0 <= x < m for x in states):
            raise ValidationError("every state must be an in-range integer")
        if len(set(states)) != len(states):
            raise ValidationError("non-primitive cycle: repeated state")
        for i, state in enumerate(states):
            if (a * state + b) % m != states[(i + 1) % len(states)]:
                raise ValidationError(f"incorrect transition at index {i}")
        pivot = states.index(min(states))
        return AffineCycle(m, a, b, states[pivot:] + states[:pivot])

    def record(self) -> dict[str, Any]:
        o = self.canonical()
        return {
            "class": "affine-cycle", "version": 1,
            "conditions": {"modulus": o.modulus, "multiplier": o.multiplier, "increment": o.increment},
            "answer": {"cycle": list(o.states)},
            "verifier": "affine-modular-cycle/1",
        }

    def canonical_bytes(self) -> bytes:
        payload = json.dumps(self.record(), sort_keys=True, ensure_ascii=True,
                             separators=(",", ":"), allow_nan=False).encode("ascii")
        return frame_fields(b"SSO:SCIENTIFIC_OBJECT:v1", payload)


def leading_zero_bits(digest: bytes) -> int:
    if not isinstance(digest, bytes) or not digest:
        raise ValueError("digest must be nonempty bytes")
    value = int.from_bytes(digest, "big")
    return 8 * len(digest) if value == 0 else 8 * len(digest) - value.bit_length()


def rarity_match(digest: bytes, k: int, exact: bool = False) -> bool:
    if not isinstance(digest, bytes) or len(digest) != 32:
        raise ValueError("expected a SHA-256 digest of 32 bytes")
    if type(k) is not int or not 0 <= k <= 256:
        raise ValueError("k outside [0,256]")
    n = leading_zero_bits(digest)
    return n == k if exact else n >= k


def prefix_bits(a: bytes, b: bytes, complementary: bool = False) -> int:
    if not isinstance(a, bytes) or not isinstance(b, bytes) or not a or len(a) != len(b):
        raise ValueError("expected equally sized nonempty byte strings")
    delta = bytes(x ^ y ^ (255 if complementary else 0) for x, y in zip(a, b))
    return leading_zero_bits(delta)


def fingerprints(obj: AffineCycle) -> dict[str, Any]:
    """Verify before hash; second SHA-256 hashes raw digest, NOT hex characters."""
    verified = obj.record()
    encoded = obj.canonical_bytes()
    h1 = hashlib.sha256(encoded).digest()
    h2 = hashlib.sha256(h1).digest()
    return {
        "format": "SSO-FINGERPRINT/1", "accepted": True,
        "object": verified, "canonical_bytes_hex": encoded.hex(),
        "sha256": h1.hex(), "double_sha256": h2.hex(),
        "leading_zeros_sha256": leading_zero_bits(h1),
        "leading_zeros_double_sha256": leading_zero_bits(h2),
        "hash_calls": 2,
    }


def linked_pair(a: AffineCycle, b: AffineCycle) -> dict[str, Any]:
    """Two distinct primitive cycles of one affine rule, a verifiable pilot relation."""
    aa, bb = a.canonical(), b.canonical()
    if (aa.modulus, aa.multiplier, aa.increment) != (bb.modulus, bb.multiplier, bb.increment):
        raise ValidationError("different transition rules")
    if aa.states == bb.states:
        raise ValidationError("not two distinct cycles")
    if aa.canonical_bytes() > bb.canonical_bytes():
        aa, bb = bb, aa
    fa, fb = fingerprints(aa), fingerprints(bb)
    h1, h2 = bytes.fromhex(fa["sha256"]), bytes.fromhex(fb["sha256"])
    return {
        "relation": "distinct-cycles-same-affine-rule/1",
        "verified": True, "a": fa["sha256"], "b": fb["sha256"],
        "k_a": fa["leading_zeros_sha256"], "k_b": fb["leading_zeros_sha256"],
        "common_prefix_bits": prefix_bits(h1, h2),
        "complementary_prefix_bits": prefix_bits(h1, h2, complementary=True),
        "hash_calls": 4,
    }


__all__ = ["AffineCycle", "ValidationError", "frame_fields", "leading_zero_bits",
           "rarity_match", "prefix_bits", "fingerprints", "linked_pair"]
