import hashlib
import unittest

from pashamango.math.sso import (
    AffineCycle, ValidationError, frame_fields, fingerprints, leading_zero_bits,
    linked_pair, prefix_bits, rarity_match,
)
from pashamango.math.sso.experiment import cycles_for_rule, run_pilot


class SSOTests(unittest.TestCase):
    def test_rotation_and_parameters(self):
        x = AffineCycle(5, 1, 1, (0, 1, 2, 3, 4))
        y = AffineCycle(5, 6, 11, (3, 4, 0, 1, 2))
        self.assertEqual(x.canonical_bytes(), y.canonical_bytes())
        self.assertEqual(fingerprints(x)["sha256"], fingerprints(y)["sha256"])

    def test_invalid_cycles(self):
        cases = [
            AffineCycle(5, 1, 1, (0, 1)),
            AffineCycle(2, 1, 1, (0, 1, 0, 1)),
            AffineCycle(5, 1, 1, (5,)),
            AffineCycle(True, 1, 1, (0,)),
            AffineCycle(5, -1, 0, (0,)),
            AffineCycle(5, 1, 1, (False,)),
        ]
        for case in cases:
            with self.subTest(case=case), self.assertRaises(ValidationError):
                case.canonical()

    def test_frame_unambiguous(self):
        self.assertNotEqual(frame_fields(b"a||b", b"c"), frame_fields(b"a", b"b||c"))
        with self.assertRaises(TypeError):
            frame_fields("a")  # type: ignore

    def test_raw_double_sha(self):
        x = AffineCycle(2, 1, 1, (0, 1))
        fp = fingerprints(x)
        raw = hashlib.sha256(x.canonical_bytes()).digest()
        self.assertEqual(fp["sha256"], raw.hex())
        self.assertEqual(fp["double_sha256"], hashlib.sha256(raw).hexdigest())
        self.assertNotEqual(fp["double_sha256"], hashlib.sha256(raw.hex().encode()).hexdigest())
        self.assertEqual(fp["hash_calls"], 2)

    def test_rarity_boundary(self):
        self.assertEqual(leading_zero_bits(bytes(32)), 256)
        self.assertEqual(leading_zero_bits(b"\x80"), 0)
        self.assertTrue(rarity_match(bytes(32), 256, exact=True))
        self.assertTrue(rarity_match(bytes(32), 0))
        self.assertFalse(rarity_match(bytes(32), 0, exact=True))
        with self.assertRaises(ValueError):
            rarity_match(b"\x00", 1)

    def test_pair(self):
        x, y = AffineCycle(3, 1, 0, (1,)), AffineCycle(3, 1, 0, (2,))
        self.assertEqual(linked_pair(x, y), linked_pair(y, x))
        with self.assertRaises(ValidationError):
            linked_pair(x, x)
        with self.assertRaises(ValidationError):
            linked_pair(x, AffineCycle(3, 1, 1, (0, 1, 2)))
        self.assertEqual(prefix_bits(b"\x00", b"\xff", True), 8)
        self.assertEqual(prefix_bits(b"\x00", b"\x00"), 8)

    def test_cycles(self):
        self.assertEqual([o.states for o in cycles_for_rule(3, 1, 0)],
                         [(0,), (1,), (2,)])
        self.assertEqual([o.states for o in cycles_for_rule(5, 1, 1)],
                         [(0, 1, 2, 3, 4)])

    def test_pilot(self):
        a, b = run_pilot(4, 2, 17), run_pilot(4, 2, 17)
        self.assertEqual(a, b)
        self.assertEqual(a["sha256_calls"], 2 * a["accepted_objects"])
        self.assertGreaterEqual(a["hits_single"], a["hits_joint"])
        with self.assertRaises(ValueError):
            run_pilot(26)


if __name__ == "__main__":
    unittest.main()
