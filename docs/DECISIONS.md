# Architectural Decision Records

## ADR-001: Stay on the Rust + Leptos + WASM stack
Date: 2026-04-11
Status: Accepted
Context: Considered whether the revamp should migrate to a JS framework (Next.js, Astro, SvelteKit) for easier iteration.
Decision: Stack stays. The engineering signal of a Rust/WASM portfolio is part of the value proposition for the target roles. Migration would throw away genuine differentiation.
Consequences: All phases must respect the stack. No JS framework shortcuts. SSR must be solved within Leptos.

## ADR-002: Two-remote git setup (staging separate from live)
Date: 2026-04-11
Status: Accepted
Context: Owner has two GitHub accounts (richardmussell owns the live site, richmuscle is the new account). Needed to decide where revamp work lives during development.
Decision: Single local repo with two remotes. `origin` points at the live site repo (untouched). `revamp-origin` points at a new staging repo on the richmuscle account (github.com/richmuscle/portfolio-revamp). The `revamp` branch tracks revamp-origin and is where all upgrade work happens. Merging back to the live site is a deliberate future action, not ongoing sync.
Consequences: Live site stays stable during a potentially long revamp. Staging work is backed up off-machine. Branch-to-remote tracking prevents accidental pushes to the wrong place. Site URL stays at richardmussell.github.io (the stronger URL for recruiters) regardless of where development happens.

## ADR-003: Baseline inventory before any changes
Date: 2026-04-11
Status: Accepted
Context: About to make significant engineering changes to a codebase I had not fully mapped.
Decision: Produce a comprehensive read-only inventory (INVENTORY.md, 17 sections, 987 lines) covering stack, build pipeline, routing, state management, browser API surface, async boundaries, and all custom subsystems before touching code. Update sections as they change during later phases.
Consequences: Every subsequent plan can be written against verified ground truth rather than guesses. The 80+ browser API call sites documented in section 7 are the direct input to the Phase 1 SSR migration risk list.

## ADR-004: Fix pre-existing bugs before Phase 1 SSR migration
Date: 2026-04-11
Status: Accepted
Context: The inventory surfaced three unrelated pre-existing bugs (ReadingProgress not mounted, static/docs not copied in Trunk, CI RUSTFLAGS overriding config). These were not SSR blockers but would entangle with SSR debugging if left in place.
Decision: Fix all three as separate clean commits on revamp before starting Phase 1.
Consequences: Phase 1 SSR work begins on a codebase where the build pipeline is honest (local and CI use identical linker flags), the docs route works in production, and the UI chrome is correctly wired. Hydration debugging in Phase 1 will not be confused by pre-existing brokenness.

## ADR-006: Retain CSP unsafe-inline — Trunk module script hash instability
Date: 2026-04-15
Status: Accepted
Context: The meta CSP in index.html includes `script-src 'self' 'unsafe-inline' 'wasm-unsafe-eval'`.
`unsafe-inline` was flagged in the security audit. The fix would be to compute SHA-256 hashes of each
inline script and add `'sha256-...'` to the CSP, removing `unsafe-inline`.
However, Trunk 0.21 generates a `<script type="module">` in dist/index.html whose body contains a
content-hashed filename (e.g. `richardmussell-a299326bdc081096.js`). This hash changes on every
`trunk build --release` as any source file changes. Pinning a static SHA-256 hash in index.html
would require recomputing and committing the hash after every build — fragile and CI-incompatible.
Decision: Retain `unsafe-inline`. Accept the audit finding. Document it here. Revisit when either:
(a) Trunk supports nonce injection or stable hash computation for its generated scripts, or
(b) a custom post-build script is added to compute the Trunk module hash and inject it into a
response-header CSP (requires moving off GitHub Pages to a CDN that supports custom headers).
The two manually-authored inline scripts (WASM preload hint and init timeout fallback) have stable
bodies and COULD be hashed, but there is no value in hashing those while the Trunk script remains
unhashed — `unsafe-inline` would still be required for the Trunk script.
Consequences: CSP audit finding remains open. The actual XSS risk is low (static site, no user
input processed server-side). Priority: low. Do not spend engineering time on this until Trunk
or hosting supports it cleanly.

## ADR-005: Defer content/substance work in favor of engineering phases
Date: 2026-04-11
Status: Accepted (with reservations documented)
Context: The initial audit identified that the biggest weakness of the site for its stated goal (landing roles) is content-layer: title drift, aspirational language, no completed certs, no PDF resume, unquantified work history, broken sitemap, placeholder demo pages. The audit recommended fixing these first. Owner chose to proceed with engineering phases instead.
Decision: Proceed with the phased engineering upgrade. Document the deferred content work in ROADMAP.md so it is not forgotten. Any session should remind the owner this work exists when it becomes relevant.
Consequences: Engineering phases will land on a site whose underlying content still has the identity-crisis problem. The engineering upgrades will not, on their own, produce the "land roles" outcome. Revisiting this decision is expected.
