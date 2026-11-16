// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-release-engineering: release engineering component model.
//!
//! Real implementation: releases, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Release {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReleaseEngineeringError {
    EmptyReleaseName,
    DuplicateRelease(String),
    EmptyEventSubject,
}

impl fmt::Display for ReleaseEngineeringError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyReleaseName => write!(f, "release name must not be empty"),
            Self::DuplicateRelease(id) => write!(f, "duplicate release {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for ReleaseEngineeringError {}

pub struct ReleaseEngineering {
    pub name: String,
    releases: BTreeMap<String, Release>,
    events: BTreeMap<String, Event>,
}

impl ReleaseEngineering {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            releases: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_release(&mut self, item: Release) -> Result<(), ReleaseEngineeringError> {
        if item.name.trim().is_empty() {
            return Err(ReleaseEngineeringError::EmptyReleaseName);
        }
        let id = item.id.clone();
        if self.releases.contains_key(&id) {
            return Err(ReleaseEngineeringError::DuplicateRelease(id.clone()));
        }
        self.releases.insert(id, item);
        Ok(())
    }

    pub fn release_count(&self) -> usize {
        self.releases.len()
    }

    pub fn get_release(&self, id: &str) -> Option<&Release> {
        self.releases.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), ReleaseEngineeringError> {
        if event.subject.trim().is_empty() {
            return Err(ReleaseEngineeringError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_release(id: &str, name: &str) -> Release {
        Release {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = ReleaseEngineering::new("ReleaseEngineering");
        assert_eq!(m.name, "ReleaseEngineering");
        assert_eq!(m.release_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_release() {
        let mut m = ReleaseEngineering::new("ReleaseEngineering");
        m.add_release(sample_release("a1", "Alpha")).unwrap();
        assert_eq!(m.release_count(), 1);
        assert!(m.get_release("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = ReleaseEngineering::new("ReleaseEngineering");
        assert!(m.add_release(sample_release("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = ReleaseEngineering::new("ReleaseEngineering");
        m.add_release(sample_release("x", "One")).unwrap();
        assert!(m.add_release(sample_release("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = ReleaseEngineering::new("ReleaseEngineering");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
