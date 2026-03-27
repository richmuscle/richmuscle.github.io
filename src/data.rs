//! Static data, types, and shared constants.
use std::sync::LazyLock;

pub const EMAIL: &str = "Richard.Mussell@yahoo.com";
pub const GITHUB_URL: &str = "https://github.com/richardmussell";
pub const LINKEDIN_URL: &str = "https://www.linkedin.com/in/richard-mussell/";
pub const PROFESSIONAL_TITLE: &str = "Information Technology & Systems Professional";
#[allow(dead_code)] // reserved for canonical URLs and sitemap
pub const SITE_URL: &str = "https://richardmussell.dev";

// (Performance budget / Lighthouse metrics removed to align with the IT systems persona.)
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, PartialEq)]
#[allow(dead_code)] // Degraded, Maintenance used for project status in data and UI
pub enum SystemStatus { Operational, Degraded, Maintenance }

#[derive(Clone, PartialEq)]
#[allow(dead_code)]
pub struct Challenge { pub title: String, pub detail: String }

#[derive(Clone, PartialEq)]
#[allow(dead_code)]
pub struct Outcome { pub metric: String, pub value: String, pub unit: String }

#[derive(Clone, PartialEq)]
pub struct TimelineEntry { pub date: String, pub title: String, pub body: String }

#[derive(Clone, PartialEq)]
pub struct CodeSnippet { pub lang: String, pub label: String, pub code: String }

#[derive(Clone, PartialEq)]
pub struct BeforeAfter { pub label: String, pub before: String, pub after: String }

#[derive(Clone, Debug, PartialEq)]
pub enum ProjectCategory {
    CyberSecurity,
    CloudInfrastructure,
    SystemsAdmin,
    Networking,
}

impl ProjectCategory {
    pub fn label(&self) -> &'static str {
        match self {
            Self::CyberSecurity       => "Cyber Security",
            Self::CloudInfrastructure => "Cloud Infrastructure",
            Self::SystemsAdmin        => "Systems Admin",
            Self::Networking          => "Network Operations",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::CyberSecurity => "Zero-trust architecture, encrypted administration, and security governance.",
            Self::CloudInfrastructure => "Hardened cloud infrastructure delivered through infrastructure-as-code.",
            Self::SystemsAdmin => "Linux administration automation via scripting and repeatable runbooks.",
            Self::Networking => "Network-focused observability and resilient connectivity troubleshooting.",
        }
    }

    pub fn accent(&self) -> &'static str {
        match self {
            Self::CyberSecurity       => "#f59e0b", // amber
            Self::CloudInfrastructure => "#3b82f6", // blue
            Self::SystemsAdmin        => "#10b981", // emerald
            Self::Networking          => "#22d3ee", // cyan
        }
    }

    #[allow(dead_code)] // reserved for category ordering in UI
    pub fn order(&self) -> u8 {
        match self {
            Self::CyberSecurity       => 1,
            Self::CloudInfrastructure => 2,
            Self::SystemsAdmin        => 3,
            Self::Networking          => 4,
        }
    }
}

/// Project card and list — compiled into Wasm. No long-form content.
#[derive(Clone, PartialEq)]
pub struct ProjectIndex {
    pub slug: &'static str,
    pub title: &'static str,
    pub subtitle: &'static str,
    pub description: &'static str,
    pub category: ProjectCategory,
    pub status: SystemStatus,
    pub tech_stack: &'static [&'static str],
}

/// Fetched at runtime from /projects/{slug}.json
#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct ProjectDetail {
    pub slug: String,
    pub content: String,
    #[serde(default)]
    pub demo_url: Option<String>,
}

#[derive(Clone, PartialEq)]
pub struct Certification {
    pub name: String,
    pub issuer: String,
    pub status: String,
}

fn init_certifications() -> Vec<Certification> {
    vec![
        Certification { name: "CKA — Certified Kubernetes Administrator".into(),        issuer: "CNCF".into(),     status: "Interested".into() },
        Certification { name: "CKAD — Kubernetes Application Developer".into(),         issuer: "CNCF".into(),     status: "Interested".into() },
        Certification { name: "CKS — Kubernetes Security Specialist".into(),            issuer: "CNCF".into(),     status: "Interested".into() },
        Certification { name: "RHCSA — Red Hat Certified System Administrator".into(),  issuer: "Red Hat".into(),  status: "Pursuing".into() },
        Certification { name: "Google Cloud — Professional Cloud Architect".into(),     issuer: "Google Cloud".into(), status: "Pursuing".into() },
    ]
}
static CERTIFICATIONS: LazyLock<Vec<Certification>> = LazyLock::new(init_certifications);

pub fn get_certifications() -> &'static Vec<Certification> {
    &*CERTIFICATIONS
}

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
            slug: "the-orchestrated-landscape-building-a-high-integrity-ecosystem",
            title: "The Orchestrated Landscape: Building a High-Integrity Ecosystem",
            date: "2024",
            tags: &["SYSTEMS-ORCHESTRATION", "IDENTITY-GOVERNANCE", "REMEDIATION-ENGINES", "OBSERVABILITY"],
            category: "PLATFORM ARCHITECTURE",
            is_core: true,
            pdf_url: Some("/pdfs/orchestrated-landscape.pdf"),
            summary: "A deep-dive into the architectural philosophy behind a self-healing, self-governing infrastructure. Exploring the intersection of Identity-as-Code, global connectivity fabrics, and automated remediation engines.",
            read_time: "9 min read",
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
        WriteUpIndex {
            slug: "the-sustainable-architect-engineering-low-entropy-landscapes-for-the-50-year-lookout",
            title: "The Sustainable Architect: Engineering Low-Entropy Landscapes for the 50-Year Lookout",
            date: "2024",
            tags: &["SUSTAINABILITY", "SYSTEM-ENTROPY", "ESG", "PLATFORM-GOVERNANCE"],
            category: "STRATEGY & GOVERNANCE",
            is_core: true,
            pdf_url: Some("/pdfs/sustainable-architect.pdf"),
            summary: "A deep-dive into the intersection of Sustainability and Platform Engineering. Treating technical debt as system entropy and utilizing declarative intent as the ultimate tool for architectural stewardship.",
            read_time: "10 min read",
        },
        WriteUpIndex {
            slug: "the-builders-ledger-orchestrating-technical-outcomes-through-project-governance",
            title: "The Builder's Ledger: Orchestrating Technical Outcomes through Project Governance",
            date: "2024",
            tags: &["PROJECT-GOVERNANCE", "SYSTEMS-LEADERSHIP", "ADMISSION-CONTROL", "STRATEGIC-PLANNING"],
            category: "STRATEGY & GOVERNANCE",
            is_core: true,
            pdf_url: Some("/pdfs/builders-ledger.pdf"),
            summary: "Reconceptualizing Project Management as the Governance Layer of systems architecture. Applying control plane logic, declarative intent, and admission control to organizational initiatives and high-velocity engineering teams.",
            read_time: "9 min read",
        },
        WriteUpIndex {
            slug: "universal-dialects-the-role-of-linux-and-shell-in-the-unified-control-plane",
            title: "Universal Dialects: The Role of Linux and Shell in the Unified Control Plane",
            date: "2024",
            tags: &["LINUX-ADMIN", "POWERSHELL", "CONFIGURATION-AS-CODE", "HYBRID-CLOUD"],
            category: "PLATFORM ARCHITECTURE",
            is_core: true,
            pdf_url: Some("/pdfs/universal-dialects.pdf"),
            summary: "Analyzing the fundamental roles of Linux and PowerShell as the 'hammer and nails' of systems architecture. Transitioning from 'Script-Ops' into declarative configuration-as-code for high-integrity governance.",
            read_time: "9 min read",
        },
        WriteUpIndex {
            slug: "the-architect-of-the-prismatic-apex-orchestrating-equilibrium-in-a-holographic-landscape",
            title: "The Architect of the Prismatic Apex: Orchestrating Equilibrium in a Holographic Landscape",
            date: "2024",
            tags: &["SYSTEMS-METAPHYSICS", "EQUILIBRIUM", "PLATFORM-ARCHITECTURE", "50-YEAR-LOOKOUT"],
            category: "STRATEGY & GOVERNANCE",
            is_core: true,
            pdf_url: Some("/pdfs/platform-architecture-blueprint.pdf"),
            summary: "A high-level philosophical analysis of systems design as the convergence of opposing forces. Reconciling absolute security (Silver) with developer velocity (Gold) to architect a holographic, 50-year stable ecosystem.",
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
    ]
}

// Long-form writeup content lives in static/writeups/{slug}.json and is fetched at runtime.

pub(crate) static WRITEUPS: LazyLock<Vec<WriteUpIndex>> = LazyLock::new(init_writeups_index);

fn init_projects_index() -> Vec<ProjectIndex> {
    vec![
        ProjectIndex {
            slug: "terraform-gcp",
            title: "Hardened Cloud Landing Zone (IaC)",
            subtitle: "Modular Infrastructure Provisioning via Terraform & GCP",
            description: "Manual cloud provisioning leads to configuration drift, security 'dark debt,' and non-deterministic environments. This project engineers a secure, version-controlled Landing Zone on GCP that enforces immutability and follows NIST 800-53 security controls from the first line of code.",
            category: ProjectCategory::CloudInfrastructure,
            status: SystemStatus::Operational,
            tech_stack: &[
                "Terraform",
                "GCP",
                "GCS Backend",
                "IAM",
                "VPC Peering",
                "Secret Manager",
                "Cloud NAT",
                "NIST 800-53",
            ],
        },
        ProjectIndex {
            slug: "linux-admin-scripting",
            title: "Systems Lifecycle Automation Framework",
            subtitle: "Idempotent Shell Engineering & Linux Systems Hardening",
            description: "Manual server administration and ad-hoc scripting introduce high operational toil and 'configuration drift.' This project engineers a suite of idempotent, POSIX-compliant automation tools designed to enforce a deterministic system state and eliminate manual intervention in the systems lifecycle.",
            category: ProjectCategory::SystemsAdmin,
            status: SystemStatus::Operational,
            tech_stack: &[
                "Bash (POSIX)",
                "Linux (RHEL/Ubuntu)",
                "Cron",
                "RBAC",
                "Systems Hardening",
                "GPO for Linux",
                "Sysctl",
            ],
        },
        ProjectIndex {
            slug: "monitoring-observability",
            title: "Performance Telemetry Pipeline",
            subtitle: "Engineering Data-Driven System Tuning with Automated Lifecycle Scripts",
            description: "Engineered a data-driven performance telemetry pipeline that converts time-series signals into deterministic tuning actions. Automated lifecycle scripts translate telemetry into repeatable system state changes, reducing manual intervention and improving operational stability.",
            category: ProjectCategory::Networking,
            status: SystemStatus::Operational,
            tech_stack: &[
                "Prometheus",
                "Grafana",
                "Telemetry",
                "SNMP",
                "Performance Tuning",
                "Automation",
                "SLO/SLI",
            ],
        },
        ProjectIndex {
            slug: "zero-trust-networking",
            title: "Zero-Trust Administrative Fabric",
            subtitle: "SASE Architecture via WireGuard & AWS Identity Integration",
            description: "Legacy 'Castle-and-Moat' VPNs grant excessive lateral trust once a perimeter is breached. This project engineers a Zero-Trust Network Access (ZTNA) solution that enforces 'Verify Explicitly' and 'Least Privilege' for administrative access to sensitive cloud infrastructure.",
            category: ProjectCategory::CyberSecurity,
            status: SystemStatus::Operational,
            tech_stack: &[
                "WireGuard",
                "AWS VPC",
                "Active Directory",
                "NIST 800-207",
                "Micro-segmentation",
                "MSS Clamping",
                "IAM",
            ],
        },
    ]
}

// Full project content moved to static/projects/{slug}.json and fetched at runtime.

static PROJECTS: LazyLock<Vec<ProjectIndex>> = LazyLock::new(init_projects_index);

pub fn get_infrastructure_fleet() -> &'static Vec<ProjectIndex> {
    &*PROJECTS
}

pub fn find_project(slug: &str) -> Option<ProjectIndex> {
    PROJECTS.iter().find(|p| p.slug == slug).cloned()
}

pub fn one_liner_for_project(slug: &str) -> &'static str {
    match slug {
        "terraform-gcp" => "Engineered an idempotent framework for a deterministic GCP Landing Zone with state-safe Terraform modules and governance-ready outputs.",
        "linux-admin-scripting" => "Engineered an idempotent framework enforcing deterministic Linux state with strict-mode execution and lifecycle drift control.",
        "monitoring-observability" => "Engineered an idempotent framework for data-driven performance telemetry, deterministic tuning actions, and automated lifecycle scripts aligned to SLO impact.",
        "zero-trust-networking" => "Engineered an idempotent framework for a ZTNA administrative fabric with identity controls, micro-segmentation, and MSS-clamped stability.",
        _ => "Engineered an idempotent framework for IT operations",
    }
}

#[derive(Clone)]
pub struct ProjectCardSignals {
    pub expanded_slug: leptos::ReadSignal<Option<String>>,
    pub set_expanded_slug: leptos::WriteSignal<Option<String>>,
    pub did_drag: leptos::RwSignal<bool>,
}

#[derive(Clone)]
pub struct ReadProgressSignals {
    pub progress: leptos::ReadSignal<f64>,
    pub set_progress: leptos::WriteSignal<f64>,
}

// (Old init_projects and writeup section content removed — see static/projects/*.json and static/writeups/*.json)
