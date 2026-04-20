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
            pdf_url: None,
            summary: "A production-minded guide to verifier constraints, CO-RE portability, and ring-buffer operational tuning—focused on what breaks under real load.",
            read_time: "10 min read",
        },
        WriteUpIndex {
            slug: "the-orchestrator-of-intent-reflections-on-service-provisioning",
            title: "The Orchestrator of Intent: Reflections on Service Provisioning",
            date: "2024",
            tags: &["PLATFORM ENGINEERING", "SYSTEMS ARCHITECTURE", "CONTROL PLANES"],
            category: "PLATFORM ARCHITECTURE",
            is_core: true,
            pdf_url: Some("#"),
            summary: "A deep-dive into how high-volume telecommunications operations function as a precursor to modern platform engineering-utilizing legacy databases as declarative control planes for service intent.",
            read_time: "8 min read",
        },
        WriteUpIndex {
            slug: "the-architect-of-oceanic-visibility-soc-operations-at-universal-scale",
            title: "The Architect of High-Fidelity Observability: SOC Operations at Universal Scale",
            date: "2024",
            tags: &["CYBERSECURITY", "OBSERVABILITY", "SOC-OPS", "ELK-STACK"],
            category: "OBSERVABILITY & SOC-OPS",
            is_core: true,
            pdf_url: Some("#"),
            summary: "A deep-dive into SOC 3.0 observability. Transforming raw telemetry into high-fidelity actionable intelligence through the ELK Stack, KQL semantic search, and the MantisBT ledger of truth.",
            read_time: "10 min read",
        },
        WriteUpIndex {
            slug: "the-connectivity-fabric-mastering-the-bedrock-of-the-universal-control-plane",
            title: "The Connectivity Fabric: Mastering the Bedrock of the Universal Control Plane",
            date: "2024",
            tags: &["NETWORKING", "CISCO-IOS", "INFRASTRUCTURE", "CONTROL-PLANES"],
            category: "NETWORKING & INFRA",
            is_core: true,
            pdf_url: Some("#"),
            summary: "Exploring the \"physics\" of the digital world through Cisco IOS. A deep-dive into how hardware-level configuration, VLSM, and port security form the original declarative control plane for modern infrastructure.",
            read_time: "7 min read",
        },
        WriteUpIndex {
            slug: "the-mirror-universe-architecting-deterministic-enterprise-simulations",
            title: "The Mirror Universe: Architecting Deterministic Enterprise Simulations",
            date: "2024",
            tags: &["ACTIVE-DIRECTORY", "POWERSHELL", "AUTOMATION", "HYPER-V"],
            category: "PLATFORM ARCHITECTURE",
            is_core: true,
            pdf_url: Some("#"),
            summary: "A deep-dive into high-fidelity enterprise sandboxing. Utilizing PowerShell and AutomatedLab to build a deterministic Windows Server 2022 simulation for testing identity governance, GPO enforcement, and network sovereignty.",
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
}
