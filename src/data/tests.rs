//! Unit tests for the data module — index integrity and lookup correctness.
use super::*;
use std::collections::HashSet;

#[test]
fn projects_index_not_empty() {
    assert!(get_infrastructure_fleet().len() > 0, "PROJECTS index must contain at least one project");
}

#[test]
fn project_slugs_unique() {
    let projects = get_infrastructure_fleet();
    let mut seen = HashSet::new();
    for p in projects.iter() {
        assert!(seen.insert(p.slug), "duplicate project slug detected: {}", p.slug);
    }
}

#[test]
fn find_project_returns_correct() {
    let first = get_infrastructure_fleet().first().expect("projects index has at least one entry");
    let result = find_project(first.slug);
    assert!(result.is_some(), "find_project must return Some for known slug {}", first.slug);
    assert_eq!(result.unwrap().slug, first.slug, "find_project returned wrong project for slug {}", first.slug);
}

#[test]
fn find_project_unknown_slug_returns_none() {
    let result = find_project("not-a-real-slug");
    assert!(result.is_none(), "find_project must return None for unknown slug");
}

#[test]
fn certifications_not_empty() {
    assert!(get_certifications().len() > 0, "certifications list must contain at least one entry");
}
