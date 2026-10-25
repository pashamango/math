# Modal.lean provenance dossier

## Decision and roles

The project owner chose a separate implementation route and accepts responsibility for this licensing decision. No permission request to Christoph Benzmüller was required for this route.

- **Specifier:** source-aware publication editor, who had previously inspected the old `Modal.lean`.
- **Implementer:** separate AI task started with no inherited conversation history, no repository checkout, and an explicit instruction not to search for or consult existing source files.
- **Input:** formal definitions, Scott axiom fields and exact theorem statements. No old theorem proof bodies, source comments or repository path were supplied.
- **Output:** complete Lean module. The implementer attested: “I received only the specification in this prompt and did not access source files.”

The specifier's prior source exposure is disclosed; isolation applies to the implementer. The source-aware editor checked the returned proofs against the API and corrected two equality-orientation errors before integration.

## Integrity receipts

- Current module SHA-256: `07ba239a1c1cfef5806a480b4be3d11f0c8d88ab827f4492b876ae092bb25f65`
- Historical upstream article/source archive SHA-256: `27bbcd73582c190a0831c74de9710a6ab3a1e295d07050661c5e7cdf950c2a86`
- API specification: delivered directly in the isolated task; it contained definitions and theorem statements only, not proof bodies.
- CI run for this implementation: pending.

## Limits

This dossier records the task boundary and the implementer's attestation. It cannot prove what a model may have encountered during training, is not a legal opinion, and does not substitute for the Lean CI receipts. The project owner accepts responsibility for proceeding with the documented API-only implementation.
