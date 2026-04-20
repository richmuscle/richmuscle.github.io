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
    #[serde(default)]
    pub last_updated: Option<String>,
    #[serde(default)]
    pub reading_time_minutes: Option<u32>,
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
    #[serde(default)]
    pub last_updated: Option<String>,
    #[serde(default)]
    pub reading_time_minutes: Option<u32>,
    #[serde(default)]
    pub status_label: Option<String>,
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
    #[serde(default)]
    pub last_updated: Option<String>,
    #[serde(default)]
    pub reading_time_minutes: Option<u32>,
    #[serde(default)]
    pub status_label: Option<String>,
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
            slug: "security-baseline-audit",
            title: "Security Baseline & Continuous Audit Toolkit",
            subtitle: "CIS-Aligned Hardening, Terraform Compliance Gates & Continuous Drift Detection",
            description: "CIS GCP Foundations Benchmark compliance (87/92 controls passing), idempotent Terraform modules with tfsec/Checkov gates, Workload Identity Federation for zero-credential CI, and nightly drift detection with automated reconciliation.",
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
            slug: "observability-operational-intelligence",
            title: "Observability & Operational Intelligence Platform",
            subtitle: "Prometheus, ELK Stack & Grafana SLO Pipeline with Automated Alerting",
            description: "A multi-tier observability pipeline that converts time-series signals into operational decisions. Prometheus metrics, ELK log aggregation with Logstash filters, and Grafana dashboards tied to SLOs — built to reduce alert noise and surface actionable signals.",
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
            slug: "identity-access-lifecycle",
            title: "Identity & Access Lifecycle Platform",
            subtitle: "Zero-Trust Admin Access via WireGuard, Active Directory & IAM Lifecycle Automation",
            description: "Legacy VPNs grant excessive lateral trust once a perimeter is breached. This platform enforces identity-based network access with WireGuard tunnels, micro-segmentation, and out-of-band peer authorization via Active Directory — verify explicitly, least privilege.",
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
        "security-baseline-audit" => "CIS GCP 87/92 controls passing, Terraform compliance gates, Workload Identity Federation, nightly drift detection with automated reconciliation.",
        "observability-operational-intelligence" => "Multi-tier alerting pipeline: Prometheus metrics, ELK log enrichment, Grafana SLO dashboards, and automated anomaly detection.",
        "identity-access-lifecycle" => "Identity-based admin access via WireGuard tunnels, AD-gated authorization, micro-segmentation, and instant credential revocation.",
        _ => "Operational infrastructure project",
    }
}

pub const LEGACY_REDIRECTS: &[(&str, &str)] = &[
    ("linux-admin-scripting", "security-baseline-audit"),
    ("zero-trust-networking", "identity-access-lifecycle"),
    ("monitoring-observability", "observability-operational-intelligence"),
    ("terraform-gcp", "security-baseline-audit"),
];

pub fn resolve_legacy_slug(slug: &str) -> Option<&'static str> {
    LEGACY_REDIRECTS
        .iter()
        .find(|(old, _)| *old == slug)
        .map(|(_, new)| *new)
}

#[derive(Clone)]
pub struct ProjectCardSignals {
    pub expanded_slug: leptos::ReadSignal<Option<String>>,
    pub set_expanded_slug: leptos::WriteSignal<Option<String>>,
    pub did_drag: leptos::RwSignal<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn projects_index_not_empty() {
        assert!(
            !get_infrastructure_fleet().is_empty(),
            "PROJECTS index must contain at least one project"
        );
    }

    #[test]
    fn project_slugs_unique() {
        let projects = get_infrastructure_fleet();
        let mut seen = HashSet::new();
        for p in projects.iter() {
            assert!(
                seen.insert(p.slug),
                "duplicate project slug detected: {}",
                p.slug
            );
        }
    }

    #[test]
    fn find_project_returns_correct() {
        let first = get_infrastructure_fleet()
            .first()
            .expect("projects index has at least one entry");
        let result = find_project(first.slug);
        assert!(
            result.is_some(),
            "find_project must return Some for known slug {}",
            first.slug
        );
        assert_eq!(
            result.unwrap().slug,
            first.slug,
            "find_project returned wrong project for slug {}",
            first.slug
        );
    }

    #[test]
    fn find_project_unknown_slug_returns_none() {
        let result = find_project("not-a-real-slug");
        assert!(
            result.is_none(),
            "find_project must return None for unknown slug"
        );
    }

    #[test]
    fn legacy_redirects_target_known_projects() {
        let fleet = get_infrastructure_fleet();
        for (old, new) in LEGACY_REDIRECTS {
            assert!(
                fleet.iter().any(|p| p.slug == *new),
                "redirect target '{}' (from '{}') not found in project registry",
                new,
                old
            );
        }
    }

    #[test]
    fn legacy_redirect_sources_not_in_registry() {
        let fleet = get_infrastructure_fleet();
        for (old, _) in LEGACY_REDIRECTS {
            assert!(
                !fleet.iter().any(|p| p.slug == *old),
                "redirect source '{}' still exists in registry — rename incomplete",
                old
            );
        }
    }

    #[test]
    fn resolve_legacy_slug_returns_correct_target() {
        assert_eq!(
            resolve_legacy_slug("linux-admin-scripting"),
            Some("security-baseline-audit")
        );
        assert_eq!(
            resolve_legacy_slug("zero-trust-networking"),
            Some("identity-access-lifecycle")
        );
        assert_eq!(
            resolve_legacy_slug("terraform-gcp"),
            Some("security-baseline-audit")
        );
        assert_eq!(resolve_legacy_slug("security-baseline-audit"), None);
    }
}
