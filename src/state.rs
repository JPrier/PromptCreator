use crate::question::{Answer, Question};
use crate::spec::Spec;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionState {
    pub spec: Spec,
    pub questions_asked: Vec<Question>,
    pub answers: Vec<Answer>,
    pub round: usize,
}

impl SessionState {
    pub fn new(spec: Spec) -> Self {
        Self {
            spec,
            questions_asked: Vec::new(),
            answers: Vec::new(),
            round: 0,
        }
    }

    pub fn save(&self, path: &PathBuf) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    pub fn load(path: &PathBuf) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let state = serde_json::from_str(&json)?;
        Ok(state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_save_and_load() {
        let spec = Spec::new("Test".to_string(), "Build something".to_string());
        let state = SessionState::new(spec.clone());

        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_path_buf();

        state.save(&path).unwrap();
        let loaded = SessionState::load(&path).unwrap();

        assert_eq!(loaded.spec.title, spec.title);
        assert_eq!(loaded.round, 0);
    }
}
