use crate::question::Answer;
use crate::question_engine::QuestionEngine;
use crate::spec::Spec;
use crate::state::SessionState;
use anyhow::{Context, Result};
use dialoguer::{theme::ColorfulTheme, Input};
use std::path::PathBuf;

pub struct InterviewWizard {
    state: SessionState,
    max_questions_per_round: usize,
    max_rounds: usize,
}

impl InterviewWizard {
    pub fn new(spec: Spec) -> Self {
        Self {
            state: SessionState::new(spec),
            max_questions_per_round: 6,
            max_rounds: 10,
        }
    }

    pub fn from_state(state: SessionState) -> Self {
        Self {
            state,
            max_questions_per_round: 6,
            max_rounds: 10,
        }
    }

    pub fn run_interactive(&mut self) -> Result<Spec> {
        println!("\n🎯 Welcome to PromptPack Wizard!");
        println!("I'll ask you some questions to clarify your requirements.\n");

        while !self.state.spec.is_complete() && self.state.round < self.max_rounds {
            self.state.round += 1;
            println!("\n--- Round {} ---", self.state.round);

            let questions =
                QuestionEngine::generate_questions(&self.state.spec, self.max_questions_per_round);

            if questions.is_empty() {
                break;
            }

            for (idx, question) in questions.iter().enumerate() {
                println!("\nQuestion {} of {}", idx + 1, questions.len());
                println!("📋 {}", question.text);
                if let Some(hint) = &question.hint {
                    println!("   💡 Hint: {}", hint);
                }

                let answer: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Your answer")
                    .allow_empty(false)
                    .interact_text()
                    .context("Failed to read input")?;

                self.apply_answer(&question.id, &answer);

                self.state.answers.push(Answer {
                    question_id: question.id.clone(),
                    value: answer,
                });
                self.state.questions_asked.push(question.clone());
            }

            let missing = self.state.spec.missing_fields();
            if !missing.is_empty() {
                println!("\n⚠️  Still missing: {}", missing.join(", "));
            } else {
                println!("\n✅ All required fields collected!");
            }
        }

        if !self.state.spec.is_complete() {
            println!("\n⚠️  Warning: Spec is not complete. Some fields are still missing.");
            println!("Missing fields: {}", self.state.spec.missing_fields().join(", "));
        }

        Ok(self.state.spec.clone())
    }

    pub fn run_non_interactive(&mut self, answers: &[(String, String)]) -> Result<Spec> {
        for (question_id, answer_value) in answers {
            self.apply_answer(question_id, answer_value);
        }

        Ok(self.state.spec.clone())
    }

    fn apply_answer(&mut self, question_id: &str, answer: &str) {
        match question_id {
            "intent" => {
                self.state.spec.intent = Some(answer.to_string());
            }
            "deliverables" => {
                self.state.spec.deliverables = answer
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
            }
            "constraints" => {
                self.parse_constraints(answer);
            }
            "non_goals" => {
                self.state.spec.non_goals = answer
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
            }
            "acceptance_criteria" => {
                self.state.spec.acceptance_criteria = answer
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
            }
            "validation_plan" => {
                self.state.spec.validation_plan = Some(answer.to_string());
            }
            "target_environment" => {
                self.parse_target_environment(answer);
            }
            _ => {}
        }
    }

    fn parse_constraints(&mut self, input: &str) {
        // Parse format: HARD: constraint1 | SOFT: constraint2
        for part in input.split('|') {
            let part = part.trim();
            if let Some(hard_content) = part.strip_prefix("HARD:") {
                self.state
                    .spec
                    .constraints
                    .hard
                    .push(hard_content.trim().to_string());
            } else if let Some(soft_content) = part.strip_prefix("SOFT:") {
                self.state
                    .spec
                    .constraints
                    .soft
                    .push(soft_content.trim().to_string());
            } else if !part.is_empty() {
                // Default to hard constraint if no prefix
                self.state.spec.constraints.hard.push(part.to_string());
            }
        }
    }

    fn parse_target_environment(&mut self, input: &str) {
        // Parse format: LANG: Rust | TOOLCHAIN: Cargo | OS: Linux | RUNTIME: native
        for part in input.split('|') {
            let part = part.trim();
            if let Some(lang) = part.strip_prefix("LANG:") {
                self.state.spec.target_environment.language = Some(lang.trim().to_string());
            } else if let Some(toolchain) = part.strip_prefix("TOOLCHAIN:") {
                self.state.spec.target_environment.toolchain = Some(toolchain.trim().to_string());
            } else if let Some(os) = part.strip_prefix("OS:") {
                self.state.spec.target_environment.os = Some(os.trim().to_string());
            } else if let Some(runtime) = part.strip_prefix("RUNTIME:") {
                self.state.spec.target_environment.runtime = Some(runtime.trim().to_string());
            }
        }
    }

    pub fn save_state(&self, path: &PathBuf) -> Result<()> {
        self.state.save(path)
    }

    pub fn get_spec(&self) -> &Spec {
        &self.state.spec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_non_interactive_wizard() {
        let spec = Spec::new("Test".to_string(), "Build something".to_string());
        let mut wizard = InterviewWizard::new(spec);

        let answers = vec![
            ("intent".to_string(), "Build a test tool".to_string()),
            (
                "deliverables".to_string(),
                "CLI tool, Documentation".to_string(),
            ),
            (
                "constraints".to_string(),
                "HARD: Must use Rust | SOFT: Prefer async".to_string(),
            ),
            (
                "non_goals".to_string(),
                "Web interface, Mobile support".to_string(),
            ),
            (
                "acceptance_criteria".to_string(),
                "Tests pass, Tool runs successfully".to_string(),
            ),
            (
                "validation_plan".to_string(),
                "cargo test && cargo run".to_string(),
            ),
            (
                "target_environment".to_string(),
                "LANG: Rust | TOOLCHAIN: Cargo | OS: Linux".to_string(),
            ),
        ];

        let result = wizard.run_non_interactive(&answers).unwrap();

        assert!(result.is_complete());
        assert_eq!(result.intent.unwrap(), "Build a test tool");
        assert_eq!(result.deliverables.len(), 2);
        assert_eq!(result.constraints.hard.len(), 1);
        assert_eq!(result.non_goals.len(), 2);
    }

    #[test]
    fn test_parse_constraints() {
        let spec = Spec::new("Test".to_string(), "Build".to_string());
        let mut wizard = InterviewWizard::new(spec);

        wizard.parse_constraints("HARD: Must be fast | SOFT: Prefer async | HARD: Must be secure");

        assert_eq!(wizard.state.spec.constraints.hard.len(), 2);
        assert_eq!(wizard.state.spec.constraints.soft.len(), 1);
    }

    #[test]
    fn test_parse_target_environment() {
        let spec = Spec::new("Test".to_string(), "Build".to_string());
        let mut wizard = InterviewWizard::new(spec);

        wizard.parse_target_environment("LANG: Rust | TOOLCHAIN: Cargo 1.70+ | OS: Linux/macOS");

        assert_eq!(
            wizard.state.spec.target_environment.language.unwrap(),
            "Rust"
        );
        assert_eq!(
            wizard.state.spec.target_environment.toolchain.unwrap(),
            "Cargo 1.70+"
        );
        assert_eq!(
            wizard.state.spec.target_environment.os.unwrap(),
            "Linux/macOS"
        );
    }
}
