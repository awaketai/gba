//! CLI argument parsing

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "gba")]
#[command(author, version, about = "Geektime Bootcamp Agent CLI", long_about = None)]
pub struct Cli {
    /// Enable verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Initialize a new GBA project
    Init {
        /// Project name
        #[arg(short, long)]
        name: Option<String>,
    },

    /// Run the agent
    Run {
        /// Path to the repository
        #[arg(short, long, default_value = ".")]
        path: String,
    },

    /// Start interactive TUI mode
    Tui,

    /// Manage prompts
    Prompt {
        #[command(subcommand)]
        command: PromptCommands,
    },
}

#[derive(Debug, Subcommand)]
pub enum PromptCommands {
    /// List available prompts
    List,

    /// Show a specific prompt
    Show {
        /// Prompt name
        name: String,
    },
}

impl Cli {
    pub async fn run(self) -> Result<()> {
        match self.command {
            Some(Commands::Init { name }) => {
                tracing::info!("Initializing project: {:?}", name);
                // TODO: Implement init
            }
            Some(Commands::Run { path }) => {
                tracing::info!("Running agent on path: {}", path);
                // TODO: Implement run
            }
            Some(Commands::Tui) => {
                tracing::info!("Starting TUI mode");
                // TODO: Implement TUI
            }
            Some(Commands::Prompt { command }) => match command {
                PromptCommands::List => {
                    tracing::info!("Listing prompts");
                    // TODO: Implement list
                }
                PromptCommands::Show { name } => {
                    tracing::info!("Showing prompt: {}", name);
                    // TODO: Implement show
                }
            },
            None => {
                tracing::info!("No command specified, starting TUI mode");
                // TODO: Start TUI by default
            }
        }
        Ok(())
    }
}
