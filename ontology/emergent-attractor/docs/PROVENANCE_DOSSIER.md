# Modal.lean provenance dossier

## Decision and roles

The project owner chose a separate implementation route and accepts responsibility for this licensing decision. No permission request to Christoph Benzmüller was required for this route.

- **Specifier:** source-aware publication editor, who had previously inspected the old `Modal.lean`.
- **Implementer:** separate AI task started with no inherited conversation history, no repository checkout, and an explicit instruction not to search for or consult existing source files.
- **Input:** formal definitions, Scott axiom fields and exact theorem statements. No old theorem proof bodies, source comments or repository path were supplied.
- **Output:** complete Lean module. The implementer attested: “I received only the specification in this prompt and did not access source files.”

The specifier's prior source exposure is disclosed; isolation applies to the implementer. The source-aware editor checked the returned proofs against the API and corrected two equality-orientation errors before integration. After the first CI run, the source-aware integrator applied Lean elaboration fixes using only compiler diagnostics and the stated API; no previous proof bodies were consulted during those fixes.

## Integrity receipts

- Current module SHA-256: `5a636d8e3d637281ff0e3415163ceb4392455f5798e1b3794a819b0dd3cea928`
- Historical upstream article/source archive SHA-256: `27bbcd73582c190a0831c74de9710a6ab3a1e295d07050661c5e7cdf950c2a86`
- API specification: delivered directly in the isolated task; it contained definitions and theorem statements only, not proof bodies.
- PR head: `53e275114990631e7309a988483f5444fc3d3c9d`.
- CI run: [40165513435](https://github.com/pashamango/math/actions/runs/40165513435), successful.
- Artifact: [12588072185](https://github.com/pashamango/math/actions/runs/40165513435/artifacts/12588072185), SHA-256 `6c378d8223e1b73e2349a8da691ea0c339376626fffefc914a5c488dfee318f3`. The artifact's `Emergent/Modal.lean` hash matches the current module hash.

## Limits

This dossier records the task boundary and the implementer's attestation. It cannot prove what a model may have encountered during training, is not a legal opinion, and does not substitute for the Lean CI receipts. The project owner accepts responsibility for proceeding with the documented API-only implementation.
