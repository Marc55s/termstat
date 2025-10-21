pub mod sqlite;

use uuid::Uuid;
use crate::sqlite::*;
use clap::Parser;
use dirs::data_dir;
use std::fs::{create_dir_all, read_to_string, rename};
use std::io::Error;
use std::path::PathBuf;
use std::process::exit;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Init shell into bashrc file
    #[arg(short, long)]
    init: bool,

    /// Sync Log file with database
    #[arg(short, long)]
    sync: bool,
}

const LOG_DIR: &str = "termstat";
const LOG_FILE_NAME: &str = "termstat.log";

fn create_term_dir() -> Result<(), Error> {
    if let Some(state_directory) = data_dir() {
        let term_dir: PathBuf = state_directory.join(LOG_DIR);

        if !term_dir.exists() {
            create_dir_all(&term_dir)?;
        }
    }
    Ok(())
}

fn main() {
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
        // move file
        // create new file as drop in replacement
        if let Some(state_directory) = data_dir() {
            let term_file: PathBuf = state_directory.join(LOG_DIR).join(LOG_FILE_NAME);
            let time_now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
            let time_as_str = time_now.as_secs().to_string();

            let moved_file = format!("{}{}",term_file.to_str().unwrap(), time_as_str);
            println!("{} {}", term_file.to_str().unwrap(), moved_file);
            rename(&term_file, &moved_file).unwrap();

            let example_entry = CommandEntry {
               timestamp: 0,
               user: "test".to_string(),
               session: Uuid::new_v4(),
               shell_type: ShellType::Zsh,
               cmd: "test".to_string(),
               cwd: "test".to_string().into(),
               exit_code: 0,
               duration_sec: 0
            };

            match parse_log_file(&moved_file) {
                Ok(entries) => {
                    for entry in entries {
                        match sync_log_to_db(&entry) {
                            Ok(_) => println!("Log synced to database"),
                            Err(e) => println!("Error syncing log to database: {}", e)
                        }
                    }
                },
                Err(e) => println!("Error parsing log file: {}", e)
            }

            match sync_log_to_db(&example_entry) {
                Ok(_) => println!("Log synced to database"),
                Err(e) => println!("Error syncing log to database: {}", e)
            }
        }
    }
    println!("{}", args.init);
}
