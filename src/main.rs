pub mod sqlite;

use crate::sqlite::*;
use clap::Parser;
use dirs::data_dir;
use std::fs::{create_dir_all, read_to_string, remove_file, rename};
use std::io::Error;
use std::path::PathBuf;
use std::process::exit;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Init shell into .SHELLrc file
    #[arg(short, long)]
    init: bool,

    /// Sync Log file with database
    #[arg(short, long)]
    sync: bool,
}

fn create_term_dir() -> Result<(), Error> {
    if let Some(state_directory) = data_dir() {
        let term_dir: PathBuf = state_directory.join(LOG_DIR);

        if !term_dir.exists() {
            create_dir_all(&term_dir)?;
        }
    }
    Ok(())
}

fn process_log_file() -> Result<(), Box<dyn std::error::Error>> {
    if let Some(state_directory) = data_dir() {
        let term_file: PathBuf = state_directory.join(LOG_DIR).join(LOG_FILE_NAME);
        let time_now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let time_as_str = time_now.as_secs().to_string();

        let moved_file = format!("{}{}", term_file.to_str().unwrap(), time_as_str);
        println!(
            "Moving file: {} {}",
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
                eprintln!("Error parsing log file: {}. Renaming back.", e);
                let _ = rename(&moved_file, &term_file);
                return Err(e);
            }
        };

        let mut all_succeeded = true;
        for entry in entries {
            match insert_cmd_entry(&entry) {
                Ok(_) => {
                    println!("Log-Entry synced to database");
                }
                Err(e) => {
                    eprintln!("Error syncing log to database: {}", e);
                    all_succeeded = false;
                }
            }
        }

        if all_succeeded {
            println!("Synced all log entries to database successfully");
            remove_file(&moved_file).unwrap();
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let _ = create_term_dir();

    // init zshrc
    if args.init {
        let _ = create_term_dir();
        let bash_config_path = PathBuf::from("init").join("termstat.zsh");
        if let Ok(bash_config) = read_to_string(bash_config_path) {
            println!("{}", bash_config);
        }
        exit(0);
    } else if args.sync {
        process_log_file()?;
    } else {
        todo!("Fetch Statistics from db");
    }

    Ok(())
}
