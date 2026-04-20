# Phase 1.6 Complete

Committed: `9f6a0a2` on `revamp`, 2026-04-20.

## Per-project render state

| Project | Badge | Detail page |
|---|---|---|
| security-baseline-audit | **● LIVE** (green) | Full V2 case study: hero metrics, problem, decisions, code highlights, outcomes, lessons. Tab strip visible. |
| identity-access-lifecycle | **◐ IN DEVELOPMENT** (muted) | Notice bar: "This case study is currently at V1 depth. A fuller treatment is in progress." Then V1 HTML content renders below tab strip. |
| observability-operational-intelligence | **◐ IN DEVELOPMENT** (muted) | Same pattern: notice bar + V1 HTML content. |
| endpoint-management-compliance | **○ PLANNED** (dim) | No tab strip. Centered status badge, one-liner scope statement, and "Work has not begun" notice. No empty sections. |
| backup-recovery-continuity | **○ PLANNED** (dim) | Same Planned render: badge + scope + notice. |
| operational-foundation | **○ PLANNED** (dim) | Same Planned render: badge + scope + notice. |

## Gaps closed

1. **ProjectStatus enum** added: `Shipped | InDevelopment | Planned`. Parallel to `SystemStatus` (telemetry health), not a replacement. Each of the six projects assigned honest values.
2. **Home page badges** now gate on `ProjectStatus`: 1 LIVE, 2 IN DEVELOPMENT, 3 PLANNED. No planned project shows "● LIVE."
3. **Detail page Planned render path**: suppresses tab strip, case study body, and all content sections. Shows title, status badge, scope statement, and "work has not begun" notice only.
4. **Detail page InDevelopment notice**: renders a V1-depth advisory bar above the existing content.
5. **Empty filter tabs hidden**: CloudInfrastructure tab no longer renders (0 projects in that category). Tabs shown: Cyber Security (1), Systems Admin (4), Network Operations (1).
6. **one_liner field** added to `ProjectIndex` struct. The `one_liner_for_project()` function is deleted. One edit site to add a project, not two.
7. **ProjectDomain enum** added: `Identity | Endpoints | Security | DataProtection | Operations`. Five domains for six projects (Operations holds observability + foundation). Assigned per project. Not yet consumed by any page — ready for /platform.

## Domain taxonomy defense

Five-domain chosen over four-domain. The four-domain alternative (`Identity | Endpoints | Security | Operations`) puts observability, backup/DR, and governance into a single "Operations" bucket — too broad, loses discriminatory value for the /platform page. Five-domain separates data protection (backup/DR) from operations governance (runbooks/change-mgmt), which are distinct operational concerns a sysadmin recognizes immediately.

## Verification

- 4/4 `just check` gates green
- 16/16 tests pass (new: `project_status_distribution_matches_intent`)
- Trunk serve builds and serves all six project JSON files

## Remaining gaps

- Telemetry page still uses `SystemStatus` for its own purposes — correct, no change needed
- `/platform` page not yet built — consumes `ProjectDomain`, Phase 2 or Phase 3
- Identity strings still say "Systems Administrator & DevOps Engineer" in 28 locations — Phase 2 content sweep

## Phase 2 scope unchanged

Content audit + voice reframes, identity sweep, writeup verdict execution, /platform page, resume split. All remain Phase 2. Phase 1.6 resolved only the structural honesty gaps.
