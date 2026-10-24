# SSO — Scientific Objects on a Hash Sphere

This is an experimental alpha, not an established useful-proof-of-work protocol, security theorem, or public package release.

## Usage

From the checkout:

    python -m pip install -e .
    python -m unittest discover -s tests -v
    python -m pashamango.math.sso verify --modulus 5 --multiplier 1 --increment 1 --states 3,4,0,1,2
    python -m pashamango.math.sso pilot --max-modulus 8 --threshold 3 --seed 42

Python API: import AffineCycle, fingerprints, linked_pair and rarity_match from pashamango.math.sso.

## Mathematical contract

C: fixed modulus m and affine rule f(x)=(a*x+b) mod m.
A: finite ordered list of cycle states.
Witness: each transition, including last-to-first, plus uniqueness/primitive property.
V: deterministic replay of all transitions, enforcing modulus >= 2, normalized a,b, and nonrepeating in-range states. Rotations normalize to the smallest state first.

Scientific provenance, novelty and value are NOT verified in v0.1.

Canonical bytes: versioned domain identifier and compact sorted-key JSON payload, each length-prefixed in binary (2-byte field count and 8-byte field lengths). Ambiguous string delimiters, e.g. ||, never serve as parsing separators. This custom encoding is not NIST TupleHash and needs independent interoperability review.

h1 = SHA256(canonical bytes); h2 = SHA256(h1 RAW bytes), not its hex representation. K(h) counts leading zero bits. Theoretical ideal-uniform expectation P(K>=k)=2^-k, exact P(K=k)=2^-(k+1) for k<256, and P(K=256)=2^-256. These model probabilities are not proven statistical properties of every SHA-256 input family.

Linked pairs are two different verified periodic cycles under the SAME affine map. Their relation is scientific and separately verified; prefix similarity is only a measured feature.

## H1-H5 status

H1: executable pilot validator exists; independent reproduction pending.
H2: ideal-random theoretical reference; bounded frequency experiment implemented, statistical analysis pending.
H3: structured vs random fair-benchmark and holdout experiment pending. No SHA speed-up claim.
H4: single/double and pair constraints described; independence of h1 and h2 not assumed.
H5: nonce grinding may reuse one scientific result without producing new science. Protocol design is not yet a useful-PoW solution.

Pilot enumerates each primitive orbit once for every affine map for m in 2..max_modulus; shuffling with seeded Python RNG changes order, NOT membership. It counts SHA calls, accepted objects, joint/individual threshold hits and histograms. No runtime advantage is asserted.

## Governance

Linear SSO project defines acceptance gates; GitHub issue #2 tracks the implementation. This branch is a review candidate. No merge, package publication, licensing assertion or external deployment is implied.
