//! Project index, detail, and category taxonomy.
use std::sync::LazyLock;

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, PartialEq)]
#[allow(dead_code)] // Degraded, Maintenance used for project status in data and UI
pub enum SystemStatus {
    Operational,
    Degraded,
    Maintenance,
}

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
            Self::CyberSecurity => "Cyber Security",
            Self::CloudInfrastructure => "Cloud Infrastructure",
            Self::SystemsAdmin => "Systems Admin",
            Self::Networking => "Network Operations",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::CyberSecurity => {
                "Zero-trust architecture, encrypted administration, and security governance."
            }
            Self::CloudInfrastructure => {
                "Hardened cloud infrastructure delivered through infrastructure-as-code."
            }
            Self::SystemsAdmin => {
                "Linux administration automation via scripting and repeatable runbooks."
            }
            Self::Networking => {
                "Network-focused observability and resilient connectivity troubleshooting."
            }
        }
    }

    pub fn accent(&self) -> &'static str {
        match self {
            Self::CyberSecurity => "#f59e0b",       // amber
            Self::CloudInfrastructure => "#3b82f6", // blue
            Self::SystemsAdmin => "#10b981",        // emerald
            Self::Networking => "#22d3ee",          // cyan
        }
    }

    #[allow(dead_code)] // reserved for category ordering in UI
    pub fn order(&self) -> u8 {
        match self {
            Self::CyberSecurity => 1,
            Self::CloudInfrastructure => 2,
            Self::SystemsAdmin => 3,
            Self::Networking => 4,
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
///
/// Dual-format support:
/// - Legacy projects set only `content` (pre-rendered HTML string).
/// - V2 projects populate the structured fields below (problem, decisions,
///   outcomes, etc.) and leave `content` empty. The renderer checks
///   `problem.is_some()` to pick the format.
#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct ProjectDetail {
    pub slug: String,
    #[serde(default)]
    pub content: String,
    #[serde(default)]
    pub demo_url: Option<String>,

    // ── V2 structured fields ──────────────────────────────────
    #[serde(default)]
    pub status_label: Option<String>,
    #[serde(default)]
    pub hero_metrics: Vec<HeroMetric>,
    #[serde(default)]
    pub problem: Option<String>,
    #[serde(default)]
    pub constraints_in: Vec<String>,
    #[serde(default)]
    pub constraints_out: Vec<String>,
    #[serde(default)]
    pub approach: Option<String>,
    #[serde(default)]
    pub approach_diagram_src: Option<String>,
    #[serde(default)]
    pub decisions: Vec<Decision>,
    #[serde(default)]
    pub highlights: Vec<CodeHighlight>,
    #[serde(default)]
    pub outcomes: Vec<Outcome>,
    #[serde(default)]
    pub lessons: Vec<String>,
    #[serde(default)]
    pub artifact_links: Vec<ArtifactLink>,
}

impl ProjectDetail {
    pub fn is_structured(&self) -> bool {
        self.problem.is_some()
    }
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct HeroMetric {
    pub value: String,
    pub label: String,
    #[serde(default)]
    pub color: Option<String>,
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct Decision {
    pub title: String,
    pub options_considered: Vec<String>,
    pub chose: String,
    pub because: String,
    pub tradeoff: String,
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct CodeHighlight {
    pub filename: String,
    pub lang: String,
    pub code: String,
    pub why: String,
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct Outcome {
    pub metric: String,
    pub baseline: String,
    pub result: String,
    pub method: String,
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct ArtifactLink {
    pub label: String,
    pub url: String,
    #[serde(default)]
    pub external: bool,
}

// ============================================================
//   DOCS — V2 structured doc page
// ============================================================

/// Fetched at runtime from /docs/{slug}.json
/// V2 structure with TOC + ADRs + runbook + threat model.
/// Legacy docs still populate only `content`.
#[derive(Clone, serde::Deserialize, serde::Serialize, Default)]
pub struct DocsDetail {
    pub slug: String,
    #[serde(default)]
    pub content: String,

    #[serde(default)]
    pub toc: Vec<TocEntry>,
    #[serde(default)]
    pub overview: Option<DocsOverview>,
    #[serde(default)]
    pub adrs: Vec<Adr>,
    #[serde(default)]
    pub config_walkthrough: Vec<CodeHighlight>,
    #[serde(default)]
    pub runbook: Option<Runbook>,
    #[serde(default)]
    pub security: Option<SecurityPosture>,
    #[serde(default)]
    pub observability: Option<Observability>,
    #[serde(default)]
    pub testing: Option<String>,
    #[serde(default)]
    pub limitations: Vec<String>,
    #[serde(default)]
    pub references: Vec<ArtifactLink>,
}

impl DocsDetail {
    pub fn is_structured(&self) -> bool {
        !self.toc.is_empty() || self.overview.is_some()
    }
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct TocEntry {
    pub id: String,
    pub label: String,
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct DocsOverview {
    pub description: String,
    #[serde(default)]
    pub components: Vec<ComponentSpec>,
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct ComponentSpec {
    pub name: String,
    pub purpose: String,
    #[serde(default)]
    pub inputs: Option<String>,
    #[serde(default)]
    pub outputs: Option<String>,
    #[serde(default)]
    pub slo: Option<String>,
    #[serde(default)]
    pub failure_mode: Option<String>,
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct Adr {
    pub id: String,
    pub title: String,
    pub status: String,
    pub context: String,
    pub decision: String,
    pub consequences: String,
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct Runbook {
    #[serde(default)]
    pub deploy: Vec<String>,
    #[serde(default)]
    pub rollback: Vec<String>,
    #[serde(default)]
    pub common_ops: Vec<RunbookOp>,
    #[serde(default)]
    pub incidents: Vec<RunbookIncident>,
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct RunbookOp {
    pub task: String,
    pub steps: Vec<String>,
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct RunbookIncident {
    pub scenario: String,
    pub response: String,
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct SecurityPosture {
    #[serde(default)]
    pub threat_model: Vec<ThreatMitigation>,
    #[serde(default)]
    pub compliance: Vec<ComplianceFramework>,
    #[serde(default)]
    pub unresolved_risks: Vec<String>,
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct ThreatMitigation {
    pub threat: String,
    pub mitigation: String,
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct ComplianceFramework {
    pub framework: String,
    pub controls: Vec<String>,
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct Observability {
    #[serde(default)]
    pub metrics: Vec<ObservabilityMetric>,
    #[serde(default)]
    pub alerts: Vec<ObservabilityAlert>,
    #[serde(default)]
    pub dashboards: Vec<String>,
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct ObservabilityMetric {
    pub name: String,
    pub threshold: String,
    pub source: String,
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct ObservabilityAlert {
    pub name: String,
    pub condition: String,
    pub action: String,
}

// ============================================================
//   DEMO — V2 structured demo page
// ============================================================

/// Fetched at runtime from /demos/{slug}.json (new file tree).
#[derive(Clone, serde::Deserialize, serde::Serialize, Default)]
pub struct DemoDetail {
    pub slug: String,
    #[serde(default)]
    pub hero_media_type: Option<String>, // "svg" | "gif" | "screenshot" | "video"
    #[serde(default)]
    pub hero_media_src: Option<String>,
    #[serde(default)]
    pub hero_caption: Option<String>,
    #[serde(default)]
    pub narration: Vec<NarrationCue>,
    #[serde(default)]
    pub verification: Vec<VerificationCheck>,
    #[serde(default)]
    pub reproduce: Option<ReproduceSteps>,
    #[serde(default)]
    pub output_snippets: Vec<OutputSnippet>,
    #[serde(default)]
    pub not_demonstrated: Vec<String>,
}

impl DemoDetail {
    pub fn is_populated(&self) -> bool {
        self.hero_media_src.is_some()
    }
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct NarrationCue {
    pub timestamp: String,
    pub caption: String,
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct VerificationCheck {
    pub criterion: String,
    pub method: String,
    pub observed: String,
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct ReproduceSteps {
    #[serde(default)]
    pub prereqs: Vec<String>,
    #[serde(default)]
    pub env_vars: Vec<String>,
    #[serde(default)]
    pub steps: Vec<String>,
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct OutputSnippet {
    pub label: String,
    pub lang: String,
    pub output: String,
}

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
    &PROJECTS
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
