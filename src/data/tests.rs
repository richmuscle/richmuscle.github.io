//! Unit tests for the data module — index integrity and lookup correctness.
use super::*;
use std::collections::HashSet;

#[test]
fn projects_index_not_empty() {
    assert!(
        get_infrastructure_fleet().len() > 0,
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
fn certifications_not_empty() {
    assert!(
        get_certifications().len() > 0,
        "certifications list must contain at least one entry"
    );
}

#[test]
fn writeups_index_not_empty() {
    assert!(
        all_writeups().len() > 0,
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
