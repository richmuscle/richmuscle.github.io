# Stage 2.2A Complete

## Pre-execution gate: universal-dialects → RETIRE
Zero of three specific technical claims found. Content names no Linux commands, no sysctl parameters, no PowerShell cmdlets. "THE-FABRIC-CONTROLLER" and "Sentry" are fictional tool names.

## Batch 1: Retires (commits `49541eb` + `60e36cf`)
5 writeups retired, 5 paired PDFs removed, 15 files deleted. Retired pieces: orchestrated-landscape, sustainable-architect, builders-ledger, prismatic-apex, universal-dialects.

## Batch 2: Retitles (commit `21c3013`)
4 writeups retitled with new slugs and sysadmin-voiced titles. Legacy URL redirects wired via `LEGACY_WRITEUP_REDIRECTS` array + `resolve_legacy_writeup_slug()`. Compile-time test confirms redirect table well-formedness (17/17 tests pass).

Retitle map:
- orchestrator-of-intent → service-provisioning-cox-control-planes
- oceanic-visibility → soc-observability-pisces-elk-kql
- connectivity-fabric → cisco-ios-fundamentals
- mirror-universe → windows-server-lab-powershell-automatedlab

## Batch 3: Demotions (commit `4397e47`)
3 writeups demoted via `is_demoted: bool` flag on WriteUpIndex. Hidden from /writing index, URLs remain live. Demoted: rust-wasm-edge, k8s-controller, otel-ebpf.

## /writing index: 9 visible entries
- 5 KEEP (hardening-linux, nist-terraform, zero-trust-bastion, siem-alert, ebpf-from-zero-to-prod)
- 4 RETITLED (service-provisioning-cox, soc-observability-pisces, cisco-ios-fundamentals, windows-server-lab)
- 3 demoted (accessible by URL, hidden from index)
- 5 retired (404)

## Verification
All four `just check` gates green. 17/17 tests pass. Writeup portfolio aligned with Linux Systems Administrator positioning.
