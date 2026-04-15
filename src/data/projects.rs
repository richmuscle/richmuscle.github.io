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
#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct ProjectDetail {
    pub slug: String,
    pub content: String,
    #[serde(default)]
    pub demo_url: Option<String>,
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
