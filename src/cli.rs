use clap::{Parser, Subcommand, ArgGroup};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    // Global args or flags
    #[command(subcommand)]
    pub command: Option<Commands>,
}

// Subcommands
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initializes the prehook into the .rc file of your shell
    Init {
        /// Type of shell
        #[arg(short, long)]
        shell_type: String,
    },

    /// Print statistics about your command usage
    // ArgGroups
    #[command(group = ArgGroup::new("time_interval")
        .required(false)
        .multiple(false)
    )]
    Stats {
        /// Print daily statistics about your command usage
        #[arg(short, long, group = "time_interval")]
        daily: bool,

        /// Print weekly statistics about your command usage
        #[arg(short, long, group = "time_interval")]
        weekly: bool,

        /// Print monthly statistics about your command usage
        #[arg(short, long, group = "time_interval")]
        monthly: bool,

        /// Print alltime statistics about your command usage
        #[arg(short, long, group = "time_interval")]
        all: bool,
    },

    /// Sync the log file with the database
    Sync,

    /// Remove all entries from the database
    Clean {
        /// Set Flag to true to remove the entire database
        #[arg(short, long)]
        remove_all_entries: bool
    }
}
