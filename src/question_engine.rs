use crate::question::Question;
use crate::spec::Spec;

pub struct QuestionEngine;

impl QuestionEngine {
    pub fn generate_questions(spec: &Spec, max_questions: usize) -> Vec<Question> {
        let missing = spec.missing_fields();
        let mut questions = Vec::new();

        for field in missing.iter().take(max_questions) {
            let question = match field.as_str() {
                "Intent/Objective" => Question::new(
                    "intent",
                    "What is the primary intent or objective of this project? (1-2 sentences)",
                    "intent",
                )
                .with_hint("Describe what you want to accomplish and why"),

                "Concrete Deliverables" => Question::new(
                    "deliverables",
                    "What are the concrete deliverables? (comma-separated list)",
                    "deliverables",
                )
                .with_hint("Example: CLI tool, REST API, documentation"),

                "Constraints" => Question::new(
                    "constraints",
                    "What are the hard constraints (must-haves) and soft constraints (nice-to-haves)? Format: HARD: constraint1 | SOFT: constraint2",
                    "constraints",
                )
                .with_hint("HARD: must use Python 3.9+ | SOFT: prefer async where possible"),

                "Non-goals" => Question::new(
                    "non_goals",
                    "What are the non-goals (things explicitly out of scope)? (comma-separated)",
                    "non_goals",
                )
                .with_hint("Example: mobile support, internationalization, Windows support"),

                "Acceptance Criteria" => Question::new(
                    "acceptance_criteria",
                    "What are the acceptance criteria? (comma-separated list of 'done when...' statements)",
                    "acceptance_criteria",
                )
                .with_hint("Example: all tests pass, CLI can execute main workflow, docs are complete"),

                "Validation Plan" => Question::new(
                    "validation_plan",
                    "How will you verify success? (commands to run or validation rubric)",
                    "validation_plan",
                )
                .with_hint("Example: cargo test && cargo run -- --help"),

                "Target Environment" => Question::new(
                    "target_environment",
                    "What is your target environment? Format: LANG: <language> | TOOLCHAIN: <toolchain> | OS: <os> | RUNTIME: <runtime>",
                    "target_environment",
                )
                .with_hint("Example: LANG: Rust | TOOLCHAIN: Cargo 1.70+ | OS: Linux/macOS | RUNTIME: native"),

                _ => continue,
            };
            questions.push(question);
        }

        questions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_questions_for_empty_spec() {
        let spec = Spec::new("Test".to_string(), "Build".to_string());
        let questions = QuestionEngine::generate_questions(&spec, 6);

        assert!(!questions.is_empty());
        assert!(questions.len() <= 6);
        assert!(questions.iter().any(|q| q.field == "intent"));
    }

    #[test]
    fn test_respects_max_questions() {
        let spec = Spec::new("Test".to_string(), "Build".to_string());
        let questions = QuestionEngine::generate_questions(&spec, 3);

        assert!(questions.len() <= 3);
    }
}
