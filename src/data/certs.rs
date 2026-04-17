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
    &CERTIFICATIONS
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn certifications_not_empty() {
        assert!(
            !get_certifications().is_empty(),
            "certifications list must contain at least one entry"
        );
    }

    #[test]
    fn cert_ids_unique() {
        let certs = get_certifications();
        let mut seen = HashSet::new();
        for c in certs.iter() {
            assert!(
                seen.insert(c.name.as_str()),
                "duplicate certification name detected: {}",
                c.name
            );
        }
    }
}
