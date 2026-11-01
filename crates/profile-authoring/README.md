# Profile authoring

Extracts the existing metric `Profile` and `ProfileLimits` operation from
`poet-spatial-authoring`, with the operation implementation copied byte-for-byte.
`Profile::points` returns an implicitly closed counterclockwise f64 XY polygon
for a rounded rectangle, circle, open channel or supplied polygon. Dimensions
are caller metric units (native Poet uses metres). Errors retain the existing
`AuthoringError` categories; topology validity beyond area remains the Core
polygon-validation caller's responsibility.

The caller supplies point/scratch budgets. Defaults are 4096 points and 4 MiB.
The implementation refuses invalid/degenerate dimensions, nonfinite output and
over-budget estimates, with a final point-count check. This is a cold allocating
operation; estimates do not establish exact peak memory or allocation-free behavior.
No session, revision, stale job, history, file, authority or package owner is added.
The caller retains source and owns preview/commit/cancel and existing Vibe binding.

The only dependency is the sibling drafting-geometry crate. Its dependency alias
preserves `poet_geometry` imports without changing original Rust source. The
small extracted lib.rs contains the existing error/helper definitions. Original
lib.rs and full source tests are retained as provenance, not misrepresented as
tests of this smaller crate. Geometry source tests remain in drafting-geometry.
Three active profile regression tests adapt the original malformed-profile,
parameter-change and channel-area cases; their exact mapping is retained in
provenance/test-adaptations.md. Mesh/assembly assertions stay with their owner.

Core mesh/sweep/triangulation, material/frame assembly and HMC resources remain
outside this crate. Native Poet integration needs an explicit re-export of these
types and shared `AuthoringError` from its existing spatial-authoring facade;
replacing only the dependency does not unify Rust type identities.

MIT selected from the existing source grant; see LICENSE-MIT and provenance.
Extraction-worker state: source review and staged extraction, no build/execution
or native human acceptance claim.
