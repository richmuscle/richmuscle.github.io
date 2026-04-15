//! Professional certifications.
use std::sync::LazyLock;

#[derive(Clone, PartialEq)]
pub struct Certification {
    pub name: String,
    pub issuer: String,
    pub status: String,
}

fn init_certifications() -> Vec<Certification> {
    vec![
        Certification {
            name: "Google Cloud Associate Cloud Engineer · Target Q3 2026".into(),
            issuer: "Google Cloud".into(),
            status: "In Progress".into(),
        },
        Certification {
            name: "Certified Kubernetes Administrator (CKA) · Target Q4 2026".into(),
            issuer: "CNCF / Linux Foundation".into(),
            status: "In Progress".into(),
        },
        Certification {
            name: "Cisco Networking Academy — Introduction to Networks (2018–2019)".into(),
            issuer: "Cisco Networking Academy".into(),
            status: "Coursework".into(),
        },
    ]
}
static CERTIFICATIONS: LazyLock<Vec<Certification>> = LazyLock::new(init_certifications);

pub fn get_certifications() -> &'static Vec<Certification> {
    &*CERTIFICATIONS
}
