"""Version-pinned cross-implementation interoperability regression vectors."""

import hashlib
import json
from pathlib import Path
import unittest

from pashamango.math.sso import AffineCycle, fingerprints


class GoldenVectorTest(unittest.TestCase):
    def test_framed_canonical_bytes_and_both_hashes(self):
        fixture = json.loads(
            (Path(__file__).parent.parent / "docs" / "golden_vectors.json").read_text()
        )
        obj = AffineCycle(5, 1, 1, (0, 1, 2, 3, 4))
        expected = bytes.fromhex(fixture["canonical_bytes_hex"])
        self.assertEqual(len(expected), 201)
        self.assertEqual(obj.canonical_bytes(), expected)
        self.assertEqual(hashlib.sha256(expected).hexdigest(), fixture["sha256"])
        first = hashlib.sha256(expected).digest()
        self.assertEqual(hashlib.sha256(first).hexdigest(), fixture["double_sha256"])
        observed = fingerprints(obj)
        for key in (
            "canonical_bytes_hex", "sha256", "double_sha256",
            "leading_zeros_sha256", "leading_zeros_double_sha256",
        ):
            self.assertEqual(observed[key], fixture[key])


if __name__ == "__main__":
    unittest.main()
