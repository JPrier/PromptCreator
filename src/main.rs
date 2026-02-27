mod generator;
mod question;
mod question_engine;
mod spec;
mod state;
mod wizard;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use generator::FileGenerator;
use spec::Spec;
use state::SessionState;
use std::path::PathBuf;
use wizard::InterviewWizard;

#[derive(Parser)]
#[command(name = "promptpack")]
#[command(about = "A tool for turning one-shot ideas into Codex long-horizon prompt packs", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run interactive wizard to create a prompt pack
    Wizard {
        /// Output directory for the generated prompt pack
        #[arg(short, long, default_value = "outputs")]
        out: PathBuf,

        /// Project title
        #[arg(short, long)]
        title: String,

        /// Initial one-shot prompt
        #[arg(short, long)]
        prompt: String,

        /// LLM assistance mode (not yet implemented)
        #[arg(long, default_value = "off")]
        llm: String,

        /// Force generation even if spec is incomplete
        #[arg(long)]
        force: bool,
    },

    /// Resume a previous wizard session
    Resume {
        /// Path to STATE.json file
        #[arg(short, long)]
        state: PathBuf,

        /// Force generation even if spec is incomplete
        #[arg(long)]
        force: bool,
    },

    /// Generate files from an existing SPEC.json
    Generate {
        /// Path to SPEC.json file
        #[arg(short, long)]
        spec: PathBuf,

        /// Output directory
        #[arg(short, long)]
        out: PathBuf,
    },

    /// Validate a SPEC.json file
    Validate {
        /// Path to SPEC.json file
        #[arg(short, long)]
        spec: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Wizard {
            out,
            title,
            prompt,
            llm: _,
            force,
        } => {
            let spec = Spec::new(title, prompt);
            let mut wizard = InterviewWizard::new(spec);

            let final_spec = wizard
                .run_interactive()
                .context("Failed to run interactive wizard")?;

            if !final_spec.is_complete() && !force {
                eprintln!("\n❌ Error: Spec is incomplete. Missing fields:");
                for field in final_spec.missing_fields() {
                    eprintln!("  - {}", field);
                }
                eprintln!("\nUse --force to generate anyway (will document assumptions).");
                std::process::exit(1);
            }

            let output_dir = out.join(&final_spec.slug);
            std::fs::create_dir_all(&output_dir).context("Failed to create output directory")?;

            // Save state
            let state_path = output_dir.join("STATE.json");
            wizard
                .save_state(&state_path)
                .context("Failed to save state")?;
            println!("\n💾 State saved to: {}", state_path.display());

            // Generate all files
            println!("\n📝 Generating prompt pack files...");
            let files = FileGenerator::generate_all(&final_spec, &output_dir)
                .context("Failed to generate files")?;

            println!("\n✅ Successfully generated {} files in:", files.len());
            println!("   {}", output_dir.display());
            println!("\n📦 Files created:");
            for file in &files {
                println!("   - {}", file);
            }

            println!("\n🚀 Next steps:");
            println!("   1. Read {}/README.md", output_dir.display());
            println!(
                "   2. Copy {}/ONE_SHOT_PROMPT.md into Codex",
                output_dir.display()
            );
            println!("   3. Watch Codex execute the plan!\n");
        }

        Commands::Resume { state, force } => {
            let session_state =
                SessionState::load(&state).context("Failed to load session state")?;
            let mut wizard = InterviewWizard::from_state(session_state);

            println!("\n🔄 Resuming session for: {}", wizard.get_spec().title);

            let final_spec = wizard
                .run_interactive()
                .context("Failed to run interactive wizard")?;

            if !final_spec.is_complete() && !force {
                eprintln!("\n❌ Error: Spec is incomplete. Missing fields:");
                for field in final_spec.missing_fields() {
                    eprintln!("  - {}", field);
                }
                eprintln!("\nUse --force to generate anyway.");
                std::process::exit(1);
            }

            // Use the same directory as the state file
            let output_dir = state.parent().unwrap().to_path_buf();

            // Update state
            wizard
                .save_state(&state)
                .context("Failed to update state")?;
            println!("\n💾 State updated");

            // Generate all files
            println!("\n📝 Generating prompt pack files...");
            let files = FileGenerator::generate_all(&final_spec, &output_dir)
                .context("Failed to generate files")?;

            println!("\n✅ Successfully generated {} files", files.len());
        }

        Commands::Generate { spec, out } => {
            let spec_content = std::fs::read_to_string(&spec)
                .context(format!("Failed to read spec file: {}", spec.display()))?;
            let spec: Spec =
                serde_json::from_str(&spec_content).context("Failed to parse SPEC.json")?;

            println!("\n📝 Generating prompt pack for: {}", spec.title);

            std::fs::create_dir_all(&out).context("Failed to create output directory")?;

            let files =
                FileGenerator::generate_all(&spec, &out).context("Failed to generate files")?;

            println!("\n✅ Successfully generated {} files in:", files.len());
            println!("   {}", out.display());
        }

        Commands::Validate { spec } => {
            let spec_content = std::fs::read_to_string(&spec)
                .context(format!("Failed to read spec file: {}", spec.display()))?;
            let spec: Spec =
                serde_json::from_str(&spec_content).context("Failed to parse SPEC.json")?;

            println!("\n🔍 Validating spec: {}", spec.title);

            if spec.is_complete() {
                println!("✅ Spec is complete!");
                println!("\nProject: {}", spec.title);
                println!("Intent: {}", spec.intent.unwrap_or_default());
                println!("Deliverables: {}", spec.deliverables.len());
                println!(
                    "Constraints: {} hard, {} soft",
                    spec.constraints.hard.len(),
                    spec.constraints.soft.len()
                );
            } else {
                println!("❌ Spec is incomplete. Missing fields:");
                for field in spec.missing_fields() {
                    println!("  - {}", field);
                }
                std::process::exit(1);
            }
        }
    }

    Ok(())
}
