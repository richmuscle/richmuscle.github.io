//! Writeup index and detail.
use std::sync::LazyLock;

/// Write-up card and list — compiled into Wasm. No long-form content.
#[derive(Clone, PartialEq)]
pub struct WriteUpIndex {
    pub slug: &'static str,
    pub title: &'static str,
    pub date: &'static str,
    pub tags: &'static [&'static str],
    pub category: &'static str,
    pub is_core: bool,
    pub is_demoted: bool,
    pub pdf_url: Option<&'static str>,
    pub summary: &'static str,
    pub read_time: &'static str,
}

/// Fetched at runtime from /writeups/{slug}.json
#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct WriteUpDetail {
    pub slug: String,
    pub content: String,
}

fn init_writeups_index() -> Vec<WriteUpIndex> {
    vec![
        WriteUpIndex {
            slug: "hardening-linux-municipal-environments",
            title: "Hardening Linux for Municipal Environments",
            date: "2026",
            tags: &["Cybersecurity", "Linux", "SOC"],
            category: "CYBERSECURITY & NIST",
            is_core: false,
            is_demoted: false,
            pdf_url: None,
            summary: "A fleet-ready hardening framework for municipal environments: CIS-aligned posture, idempotent enforcement, and audit-evidence that supports SOC operations.",
            read_time: "12 min read",
        },
        WriteUpIndex {
            slug: "automating-nist-800-53-compliance-with-terraform",
            title: "Automating NIST 800-53 Compliance with Terraform",
            date: "2026",
            tags: &["IaC", "Compliance", "Terraform"],
            category: "CYBERSECURITY & NIST",
            is_core: false,
            is_demoted: false,
            pdf_url: None,
            summary: "Translate NIST 800-53 security control intent into deterministic Terraform module patterns with state safety, drift-aware reconciliation, and governance-ready outputs.",
            read_time: "13 min read",
        },
        WriteUpIndex {
            slug: "zero-trust-moving-beyond-bastion-hosts",
            title: "Zero-Trust: Moving Beyond Bastion Hosts",
            date: "2026",
            tags: &["Networking", "ZeroTrust", "AWS"],
            category: "CYBERSECURITY & NIST",
            is_core: false,
            is_demoted: false,
            pdf_url: None,
            summary: "Operational patterns for zero-trust administrative access that replace shared bastion trust with identity-governed connectivity, revocation safety, and packet stability.",
            read_time: "11 min read",
        },
        WriteUpIndex {
            slug: "siem-alert-hygiene-reducing-noise-in-the-soc",
            title: "SIEM Alert Hygiene: Reducing Noise in the SOC",
            date: "2026",
            tags: &["SOC", "SIEM", "Monitoring"],
            category: "OBSERVABILITY & SOC-OPS",
            is_core: false,
            is_demoted: false,
            pdf_url: None,
            summary: "Reduce SOC alert fatigue using enrichment, cardinality-aware signal design, and SLO/MTTR-aligned dispatch logic so analysts see fewer, better alerts.",
            read_time: "10 min read",
        },
        WriteUpIndex {
            slug: "kubernetes-controller-reconciliation-deep-dive",
            title: "Why Your Kubernetes Controller Is Lying to You",
            date: "2026",
            tags: &["Kubernetes", "Go", "CloudNative"],
            category: "PLATFORM ARCHITECTURE",
            is_core: false,
            is_demoted: true,
            pdf_url: None,
            summary: "A systems-focused teardown of controller-runtime reconciliation pitfalls: informer cache staleness, optimistic locking behavior, and crash-safe convergence patterns.",
            read_time: "11 min read",
        },
        WriteUpIndex {
            slug: "otel-ebpf-tracing-without-instrumentation",
            title: "Distributed Tracing Without Touching Your App Code",
            date: "2026",
            tags: &["OTel", "eBPF", "Tracing"],
            category: "OBSERVABILITY & SOC-OPS",
            is_core: false,
            is_demoted: true,
            pdf_url: None,
            summary: "How syscall-layer eBPF tracing can generate spans and propagate context into OTel-compatible pipelines—without SDK instrumentation or code changes.",
            read_time: "9 min read",
        },
        WriteUpIndex {
            slug: "rust-wasm-edge-runtime-internals",
            title: "Building a Zero-Copy Wasm Edge Runtime in Rust",
            date: "2026",
            tags: &["Systems", "Rust", "Wasm"],
            category: "PLATFORM ARCHITECTURE",
            is_core: false,
            is_demoted: true,
            pdf_url: None,
            summary: "A deep dive into boundary-layer batching and deterministic execution for constrained Wasm edge environments—where latency is dominated by system crossings, not CPU.",
            read_time: "14 min read",
        },
        WriteUpIndex {
            slug: "ebpf-from-zero-to-prod",
            title: "eBPF From Zero to Production",
            date: "2026",
            tags: &["eBPF", "Kernel", "Observability"],
            category: "OBSERVABILITY & SOC-OPS",
            is_core: false,
            is_demoted: false,
            pdf_url: None,
            summary: "A production-minded guide to verifier constraints, CO-RE portability, and ring-buffer operational tuning—focused on what breaks under real load.",
            read_time: "10 min read",
        },
        WriteUpIndex {
            slug: "service-provisioning-cox-control-planes",
            title: "Service Provisioning at Cox: Lessons for Infrastructure Control Planes",
            date: "2024",
            tags: &["TELECOM", "PROVISIONING", "CONTROL PLANES"],
            category: "OPERATIONS",
            is_core: true,
            is_demoted: false,
            pdf_url: None,
            summary: "How navigating a telecom provisioning database (ICOMS) at Cox Communications parallels modern infrastructure control planes: declarative intent, status propagation, and admission control.",
            read_time: "8 min read",
        },
        WriteUpIndex {
            slug: "soc-observability-pisces-elk-kql",
            title: "Building SOC Observability at PISCES: ELK, KQL, and Threat Correlation",
            date: "2024",
            tags: &["SOC", "ELK-STACK", "KQL", "THREAT-DETECTION"],
            category: "OBSERVABILITY & SOC-OPS",
            is_core: true,
            is_demoted: false,
            pdf_url: None,
            summary: "Operational SOC observability at PISCES International: ELK Stack log aggregation, KQL semantic search for APT detection, temporal correlation of IDS alerts with outbound traffic, and MantisBT for incident state tracking.",
            read_time: "10 min read",
        },
        WriteUpIndex {
            slug: "cisco-ios-fundamentals",
            title: "Cisco IOS Fundamentals: Subnetting, Port Security, and OSI Troubleshooting",
            date: "2024",
            tags: &["NETWORKING", "CISCO-IOS", "VLSM", "OSI"],
            category: "NETWORKING & INFRA",
            is_core: true,
            is_demoted: false,
            pdf_url: None,
            summary: "Foundational networking via Cisco IOS: VLSM subnetting for environment isolation, MAC-based port security, and bottom-up OSI troubleshooting as the bedrock of infrastructure operations.",
            read_time: "7 min read",
        },
        WriteUpIndex {
            slug: "windows-server-lab-powershell-automatedlab",
            title: "Building a Windows Server Lab with PowerShell and AutomatedLab",
            date: "2024",
            tags: &["ACTIVE-DIRECTORY", "POWERSHELL", "AUTOMATEDLAB", "GPO"],
            category: "SYSTEMS ADMINISTRATION",
            is_core: true,
            is_demoted: false,
            pdf_url: None,
            summary: "Declarative lab provisioning with PowerShell and AutomatedLab: Windows Server 2022, Active Directory schema architecture, GPO governance testing, and virtual network sovereignty for safe experimentation.",
            read_time: "8 min read",
        },
    ]
}

// Long-form writeup content lives in static/writeups/{slug}.json and is fetched at runtime.

pub(crate) static WRITEUPS: LazyLock<Vec<WriteUpIndex>> = LazyLock::new(init_writeups_index);

pub fn all_writeups() -> &'static Vec<WriteUpIndex> {
    &WRITEUPS
}

pub fn find_writeup(slug: &str) -> Option<WriteUpIndex> {
    WRITEUPS.iter().find(|w| w.slug == slug).cloned()
}

pub const LEGACY_WRITEUP_REDIRECTS: &[(&str, &str)] = &[
    ("the-orchestrator-of-intent-reflections-on-service-provisioning", "service-provisioning-cox-control-planes"),
    ("the-architect-of-oceanic-visibility-soc-operations-at-universal-scale", "soc-observability-pisces-elk-kql"),
    ("the-connectivity-fabric-mastering-the-bedrock-of-the-universal-control-plane", "cisco-ios-fundamentals"),
    ("the-mirror-universe-architecting-deterministic-enterprise-simulations", "windows-server-lab-powershell-automatedlab"),
];

pub fn resolve_legacy_writeup_slug(slug: &str) -> Option<&'static str> {
    LEGACY_WRITEUP_REDIRECTS
        .iter()
        .find(|(old, _)| *old == slug)
        .map(|(_, new)| *new)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn writeups_index_not_empty() {
        assert!(
            !all_writeups().is_empty(),
            "WRITEUPS index must contain at least one entry"
        );
    }

    #[test]
    fn writeup_slugs_unique() {
        let writeups = all_writeups();
        let mut seen = HashSet::new();
        for w in writeups.iter() {
            assert!(
                seen.insert(w.slug),
                "duplicate writeup slug detected: {}",
                w.slug
            );
        }
    }

    #[test]
    fn find_writeup_returns_correct() {
        let first = all_writeups()
            .first()
            .expect("writeups index has at least one entry");
        let result = find_writeup(first.slug);
        assert!(
            result.is_some(),
            "find_writeup must return Some for known slug {}",
            first.slug
        );
        assert_eq!(
            result.unwrap().slug,
            first.slug,
            "find_writeup returned wrong writeup for slug {}",
            first.slug
        );
    }

    #[test]
    fn find_writeup_unknown_slug_returns_none() {
        let result = find_writeup("not-a-real-slug");
        assert!(
            result.is_none(),
            "find_writeup must return None for unknown slug"
        );
    }

    #[test]
    fn writeup_redirects_target_known_slugs() {
        let all = all_writeups();
        for (old, new) in LEGACY_WRITEUP_REDIRECTS {
            assert!(
                all.iter().any(|w| w.slug == *new),
                "writeup redirect target '{}' (from '{}') not in registry",
                new, old
            );
            assert!(
                !all.iter().any(|w| w.slug == *old),
                "writeup redirect source '{}' still in registry",
                old
            );
        }
    }
}
