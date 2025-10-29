 #![allow(warnings)] 
mod cli;
mod queries;
mod sqlite;
mod table;
mod util;
mod print_stats;

use crate::print_stats::print_stats;
use crate::util::duration::DurationExt;
use crate::cli::{Cli, Commands};
use crate::queries::{cmd_avg_runtime, cmd_runtimes, most_frequent_cmd, most_used_command};
use crate::sqlite::*;
use clap::Parser;
use dirs::data_dir;
use std::fs::{remove_file, rename};
use std::path::PathBuf;
use std::process::exit;
use std::time::{SystemTime, UNIX_EPOCH, Duration};

fn process_log_file() -> Result<(), Box<dyn std::error::Error>> {
    if let Some(state_directory) = data_dir() {
        let term_file: PathBuf = state_directory.join(LOG_DIR).join(LOG_FILE_NAME);
        let time_now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let time_as_str = time_now.as_secs().to_string();

        let moved_file = format!("{}{}", term_file.to_str().unwrap(), time_as_str);
        println!(
            "Trying to move file: {} -> {}",
            term_file.to_str().unwrap(),
            moved_file
        );

        if let Err(e) = rename(&term_file, &moved_file) {
            eprintln!("Error moving file: {}", e);
            exit(1);
        }

        let entries = match parse_log_file(&moved_file) {
            Ok(entries) => entries,
            Err(e) => {
                eprintln!("Error parsing log file: {}", e);
                let _ = rename(&moved_file, &term_file);
                return Err(e);
            }
        };

        let mut all_succeeded = true;
        for entry in &entries {
            match insert_cmd_entry(entry) {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("Error syncing log to database: {}", e);
                    all_succeeded = false;
                }
            }
        }

        if all_succeeded {
            println!(
                "Synced {} log entries to database successfully",
                entries.len()
            );
            remove_file(&moved_file).unwrap();
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Init { shell_type }) => {
            if shell_type == "zsh" {
                let shell_config = include_str!("init/termstat.zsh");
                println!("{}", shell_config);
                exit(0);
            } else {
                eprintln!("Unknown or unsupported shell type: {}", shell_type);
                exit(1);
            }
        }

        Some(Commands::Sync) => process_log_file()?,

        Some(Commands::Stats { daily , weekly, monthly }) => {
            if daily {
                let stats = vec![most_used_command(DurationExt::from_days(1))?, cmd_avg_runtime()?, cmd_runtimes()?]; 
                print_stats(stats)?;
            }
            if weekly {
                let stats = vec![most_used_command(DurationExt::from_weeks(1))?, cmd_avg_runtime()?, cmd_runtimes()?]; 
                print_stats(stats)?;
            }
            if monthly {
                let stats = vec![most_used_command(DurationExt::from_weeks(4))?, cmd_avg_runtime()?, cmd_runtimes()?]; 
                print_stats(stats)?;
            }
        }

        Some(Commands::Clean { remove_all_entries: _}) => {
            todo!("Clean command will be implemented in the future");
        }

        None => {
            println!("Welcome to Termstat!");
            println!("Usage: termstat [OPTIONS]");
            println!("Try 'termstat --help' for more information.");
        }
    }

    Ok(())
}
