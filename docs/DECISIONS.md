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

## ADR-005: Defer content/substance work in favor of engineering phases
Date: 2026-04-11
Status: Accepted (with reservations documented)
Context: The initial audit identified that the biggest weakness of the site for its stated goal (landing roles) is content-layer: title drift, aspirational language, no completed certs, no PDF resume, unquantified work history, broken sitemap, placeholder demo pages. The audit recommended fixing these first. Owner chose to proceed with engineering phases instead.
Decision: Proceed with the phased engineering upgrade. Document the deferred content work in ROADMAP.md so it is not forgotten. Any session should remind the owner this work exists when it becomes relevant.
Consequences: Engineering phases will land on a site whose underlying content still has the identity-crisis problem. The engineering upgrades will not, on their own, produce the "land roles" outcome. Revisiting this decision is expected.
