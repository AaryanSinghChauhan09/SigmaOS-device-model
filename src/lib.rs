// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-device-model: device model component model.
//!
//! Real implementation: models, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Model {
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
pub enum DeviceModelError {
    EmptyModelName,
    DuplicateModel(String),
    EmptyEventSubject,
}

impl fmt::Display for DeviceModelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyModelName => write!(f, "model name must not be empty"),
            Self::DuplicateModel(id) => write!(f, "duplicate model {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for DeviceModelError {}

pub struct DeviceModel {
    pub name: String,
    models: BTreeMap<String, Model>,
    events: BTreeMap<String, Event>,
}

impl DeviceModel {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            models: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_model(&mut self, item: Model) -> Result<(), DeviceModelError> {
        if item.name.trim().is_empty() {
            return Err(DeviceModelError::EmptyModelName);
        }
        let id = item.id.clone();
        if self.models.contains_key(&id) {
            return Err(DeviceModelError::DuplicateModel(id.clone()));
        }
        self.models.insert(id, item);
        Ok(())
    }

    pub fn model_count(&self) -> usize {
        self.models.len()
    }

    pub fn get_model(&self, id: &str) -> Option<&Model> {
        self.models.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), DeviceModelError> {
        if event.subject.trim().is_empty() {
            return Err(DeviceModelError::EmptyEventSubject);
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

    fn sample_model(id: &str, name: &str) -> Model {
        Model {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = DeviceModel::new("DeviceModel");
        assert_eq!(m.name, "DeviceModel");
        assert_eq!(m.model_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_model() {
        let mut m = DeviceModel::new("DeviceModel");
        m.add_model(sample_model("a1", "Alpha")).unwrap();
        assert_eq!(m.model_count(), 1);
        assert!(m.get_model("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = DeviceModel::new("DeviceModel");
        assert!(m.add_model(sample_model("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = DeviceModel::new("DeviceModel");
        m.add_model(sample_model("x", "One")).unwrap();
        assert!(m.add_model(sample_model("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = DeviceModel::new("DeviceModel");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
