// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-3d-modeling: 3d modeling component model.
//!
//! Real implementation: modelings, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Modeling {
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
pub enum ThreeDModelingError {
    EmptyModelingName,
    DuplicateModeling(String),
    EmptyEventSubject,
}

impl fmt::Display for ThreeDModelingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyModelingName => write!(f, "modeling name must not be empty"),
            Self::DuplicateModeling(id) => write!(f, "duplicate modeling {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for ThreeDModelingError {}

pub struct ThreeDModeling {
    pub name: String,
    modelings: BTreeMap<String, Modeling>,
    events: BTreeMap<String, Event>,
}

impl ThreeDModeling {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            modelings: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_modeling(&mut self, item: Modeling) -> Result<(), ThreeDModelingError> {
        if item.name.trim().is_empty() {
            return Err(ThreeDModelingError::EmptyModelingName);
        }
        let id = item.id.clone();
        if self.modelings.contains_key(&id) {
            return Err(ThreeDModelingError::DuplicateModeling(id.clone()));
        }
        self.modelings.insert(id, item);
        Ok(())
    }

    pub fn modeling_count(&self) -> usize {
        self.modelings.len()
    }

    pub fn get_modeling(&self, id: &str) -> Option<&Modeling> {
        self.modelings.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), ThreeDModelingError> {
        if event.subject.trim().is_empty() {
            return Err(ThreeDModelingError::EmptyEventSubject);
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

    fn sample_modeling(id: &str, name: &str) -> Modeling {
        Modeling {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = ThreeDModeling::new("ThreeDModeling");
        assert_eq!(m.name, "ThreeDModeling");
        assert_eq!(m.modeling_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_modeling() {
        let mut m = ThreeDModeling::new("ThreeDModeling");
        m.add_modeling(sample_modeling("a1", "Alpha")).unwrap();
        assert_eq!(m.modeling_count(), 1);
        assert!(m.get_modeling("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = ThreeDModeling::new("ThreeDModeling");
        assert!(m.add_modeling(sample_modeling("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = ThreeDModeling::new("ThreeDModeling");
        m.add_modeling(sample_modeling("x", "One")).unwrap();
        assert!(m.add_modeling(sample_modeling("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = ThreeDModeling::new("ThreeDModeling");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
