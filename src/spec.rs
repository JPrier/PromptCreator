use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Spec {
    pub version: String,
    pub initial_prompt: String,
    pub title: String,
    pub slug: String,
    pub intent: Option<String>,
    pub deliverables: Vec<String>,
    pub constraints: Constraints,
    pub non_goals: Vec<String>,
    pub acceptance_criteria: Vec<String>,
    pub validation_plan: Option<String>,
    pub target_environment: TargetEnvironment,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Constraints {
    pub hard: Vec<String>,
    pub soft: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TargetEnvironment {
    pub language: Option<String>,
    pub toolchain: Option<String>,
    pub os: Option<String>,
    pub runtime: Option<String>,
}

impl Spec {
    pub fn new(title: String, initial_prompt: String) -> Self {
        let slug = slug::slugify(&title);
        Self {
            version: "1.0".to_string(),
            initial_prompt,
            title,
            slug,
            intent: None,
            deliverables: Vec::new(),
            constraints: Constraints {
                hard: Vec::new(),
                soft: Vec::new(),
            },
            non_goals: Vec::new(),
            acceptance_criteria: Vec::new(),
            validation_plan: None,
            target_environment: TargetEnvironment {
                language: None,
                toolchain: None,
                os: None,
                runtime: None,
            },
            metadata: HashMap::new(),
        }
    }

    pub fn is_complete(&self) -> bool {
        self.intent.is_some()
            && !self.deliverables.is_empty()
            && (!self.constraints.hard.is_empty() || !self.constraints.soft.is_empty())
            && !self.non_goals.is_empty()
            && !self.acceptance_criteria.is_empty()
            && self.validation_plan.is_some()
            && (self.target_environment.language.is_some()
                || self.target_environment.toolchain.is_some()
                || self.target_environment.os.is_some()
                || self.target_environment.runtime.is_some())
    }

    pub fn missing_fields(&self) -> Vec<String> {
        let mut missing = Vec::new();

        if self.intent.is_none() {
            missing.push("Intent/Objective".to_string());
        }
        if self.deliverables.is_empty() {
            missing.push("Concrete Deliverables".to_string());
        }
        if self.constraints.hard.is_empty() && self.constraints.soft.is_empty() {
            missing.push("Constraints".to_string());
        }
        if self.non_goals.is_empty() {
            missing.push("Non-goals".to_string());
        }
        if self.acceptance_criteria.is_empty() {
            missing.push("Acceptance Criteria".to_string());
        }
        if self.validation_plan.is_none() {
            missing.push("Validation Plan".to_string());
        }
        if self.target_environment.language.is_none()
            && self.target_environment.toolchain.is_none()
            && self.target_environment.os.is_none()
            && self.target_environment.runtime.is_none()
        {
            missing.push("Target Environment".to_string());
        }

        missing
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_spec_is_incomplete() {
        let spec = Spec::new("Test Project".to_string(), "Build a tool".to_string());
        assert!(!spec.is_complete());
        assert_eq!(spec.slug, "test-project");
    }

    #[test]
    fn test_missing_fields() {
        let spec = Spec::new("Test".to_string(), "Prompt".to_string());
        let missing = spec.missing_fields();
        assert!(missing.contains(&"Intent/Objective".to_string()));
        assert!(missing.contains(&"Concrete Deliverables".to_string()));
        assert!(missing.contains(&"Constraints".to_string()));
    }

    #[test]
    fn test_complete_spec() {
        let mut spec = Spec::new("Test".to_string(), "Prompt".to_string());
        spec.intent = Some("Build a thing".to_string());
        spec.deliverables.push("Feature A".to_string());
        spec.constraints.hard.push("Must be fast".to_string());
        spec.non_goals.push("Not mobile".to_string());
        spec.acceptance_criteria.push("All tests pass".to_string());
        spec.validation_plan = Some("Run tests".to_string());
        spec.target_environment.language = Some("Rust".to_string());

        assert!(spec.is_complete());
        assert!(spec.missing_fields().is_empty());
    }
}
