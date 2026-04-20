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
pub enum ProjectStatus {
    Shipped,
    InDevelopment,
    Planned,
}

impl ProjectStatus {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Shipped => "LIVE",
            Self::InDevelopment => "IN DEVELOPMENT",
            Self::Planned => "PLANNED",
        }
    }

    pub fn chip_style(&self) -> &'static str {
        match self {
            Self::Shipped => "color:#10b981;",
            Self::InDevelopment => "color:#94a3b8;",
            Self::Planned => "color:#475569;",
        }
    }

    pub fn chip_glyph(&self) -> &'static str {
        match self {
            Self::Shipped => "●",
            Self::InDevelopment => "◐",
            Self::Planned => "○",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ProjectDomain {
    Identity,
    Endpoints,
    Security,
    DataProtection,
    Operations,
}

impl ProjectDomain {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Identity => "Identity & Access",
            Self::Endpoints => "Endpoint Management",
            Self::Security => "Security & Compliance",
            Self::DataProtection => "Data Protection",
            Self::Operations => "Operations & Governance",
        }
    }
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
    pub one_liner: &'static str,
    pub category: ProjectCategory,
    pub project_status: ProjectStatus,
    pub domain: ProjectDomain,
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
            one_liner: "CIS GCP 87/92 controls passing, Terraform compliance gates, Workload Identity Federation, nightly drift detection with automated reconciliation.",
            category: ProjectCategory::SystemsAdmin,
            project_status: ProjectStatus::Shipped,
            domain: ProjectDomain::Security,
            status: SystemStatus::Operational,
            tech_stack: &["Terraform", "GCP", "tfsec", "Checkov", "CIS Benchmark", "Workload Identity", "NIST 800-53"],
        },
        ProjectIndex {
            slug: "observability-operational-intelligence",
            title: "Observability & Operational Intelligence Platform",
            subtitle: "Prometheus, ELK Stack & Grafana SLO Pipeline with Automated Alerting",
            description: "A multi-tier observability pipeline that converts time-series signals into operational decisions. Prometheus metrics, ELK log aggregation with Logstash filters, and Grafana dashboards tied to SLOs — built to reduce alert noise and surface actionable signals.",
            one_liner: "Multi-tier alerting pipeline: Prometheus metrics, ELK log enrichment, Grafana SLO dashboards, and automated anomaly detection.",
            category: ProjectCategory::Networking,
            project_status: ProjectStatus::InDevelopment,
            domain: ProjectDomain::Operations,
            status: SystemStatus::Operational,
            tech_stack: &["Prometheus", "Grafana", "Telemetry", "SNMP", "Performance Tuning", "Automation", "SLO/SLI"],
        },
        ProjectIndex {
            slug: "identity-access-lifecycle",
            title: "Identity & Access Lifecycle Platform",
            subtitle: "Zero-Trust Admin Access via WireGuard, Active Directory & IAM Lifecycle Automation",
            description: "Legacy VPNs grant excessive lateral trust once a perimeter is breached. This platform enforces identity-based network access with WireGuard tunnels, micro-segmentation, and out-of-band peer authorization via Active Directory — verify explicitly, least privilege.",
            one_liner: "Identity-based admin access via WireGuard tunnels, AD-gated authorization, micro-segmentation, and instant credential revocation.",
            category: ProjectCategory::CyberSecurity,
            project_status: ProjectStatus::InDevelopment,
            domain: ProjectDomain::Identity,
            status: SystemStatus::Operational,
            tech_stack: &["WireGuard", "AWS VPC", "Active Directory", "NIST 800-207", "Micro-segmentation", "MSS Clamping", "IAM"],
        },
        ProjectIndex {
            slug: "endpoint-management-compliance",
            title: "Endpoint Management & Compliance System",
            subtitle: "Managed endpoint provisioning, compliance evaluation, and policy enforcement across Windows, macOS, and iOS devices via Intune and Autopilot.",
            description: "Centralized endpoint lifecycle management: zero-touch provisioning, OS patching, compliance scoring, and GPO-equivalent policy enforcement for hybrid Windows/macOS/iOS fleets.",
            one_liner: "Managed endpoint provisioning, compliance evaluation, and policy enforcement across Windows, macOS, and iOS devices via Intune and Autopilot.",
            category: ProjectCategory::SystemsAdmin,
            project_status: ProjectStatus::Planned,
            domain: ProjectDomain::Endpoints,
            status: SystemStatus::Operational,
            tech_stack: &[],
        },
        ProjectIndex {
            slug: "backup-recovery-continuity",
            title: "Backup, Recovery & Business Continuity System",
            subtitle: "Tiered backup, tested restore, and deliberate-disaster recovery exercises for the operational infrastructure of the other projects in this portfolio.",
            description: "3-2-1 backup strategy with scheduled restore validation, documented RTO/RPO targets per service tier, and quarterly disaster-recovery drills against the live operational stack.",
            one_liner: "Tiered backup, tested restore, and deliberate-disaster recovery exercises for the operational infrastructure.",
            category: ProjectCategory::SystemsAdmin,
            project_status: ProjectStatus::Planned,
            domain: ProjectDomain::DataProtection,
            status: SystemStatus::Operational,
            tech_stack: &[],
        },
        ProjectIndex {
            slug: "operational-foundation",
            title: "Operational Foundation",
            subtitle: "The wiki, runbook library, change-management process, and incident-response procedures that govern the other projects as a system.",
            description: "Operational governance layer: structured runbooks, change-advisory process, incident-response playbooks, and a documentation wiki that ties the portfolio projects into a single managed platform.",
            one_liner: "Wiki, runbook library, change-management process, and incident-response procedures governing the portfolio as a system.",
            category: ProjectCategory::SystemsAdmin,
            project_status: ProjectStatus::Planned,
            domain: ProjectDomain::Operations,
            status: SystemStatus::Operational,
            tech_stack: &[],
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

pub const LEGACY_REDIRECTS: &[(&str, &str)] = &[
    ("linux-admin-scripting", "security-baseline-audit"),
    ("zero-trust-networking", "identity-access-lifecycle"),
    (
        "monitoring-observability",
        "observability-operational-intelligence",
    ),
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
    fn all_six_canonical_slugs_in_registry() {
        let fleet = get_infrastructure_fleet();
        let canonical = [
            "security-baseline-audit",
            "observability-operational-intelligence",
            "identity-access-lifecycle",
            "endpoint-management-compliance",
            "backup-recovery-continuity",
            "operational-foundation",
        ];
        for slug in &canonical {
            let entry = fleet.iter().find(|p| p.slug == *slug);
            assert!(
                entry.is_some(),
                "canonical slug '{}' missing from registry",
                slug
            );
            let e = entry.unwrap();
            assert!(
                !e.title.is_empty(),
                "canonical slug '{}' has empty title",
                slug
            );
        }
    }

    #[test]
    fn all_canonical_slugs_have_one_liner() {
        let fleet = get_infrastructure_fleet();
        for p in fleet.iter() {
            assert!(
                !p.one_liner.is_empty(),
                "project '{}' has empty one_liner",
                p.slug
            );
        }
    }

    #[test]
    fn project_status_distribution_matches_intent() {
        let fleet = get_infrastructure_fleet();
        let shipped = fleet
            .iter()
            .filter(|p| p.project_status == ProjectStatus::Shipped)
            .count();
        let in_dev = fleet
            .iter()
            .filter(|p| p.project_status == ProjectStatus::InDevelopment)
            .count();
        let planned = fleet
            .iter()
            .filter(|p| p.project_status == ProjectStatus::Planned)
            .count();
        assert_eq!(shipped, 1, "expected 1 Shipped project");
        assert_eq!(in_dev, 2, "expected 2 InDevelopment projects");
        assert_eq!(planned, 3, "expected 3 Planned projects");
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
