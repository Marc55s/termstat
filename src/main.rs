mod sqlite;

use clap::Parser;
use dirs::data_dir;
use core::time;
use std::fs::{create_dir_all, read_to_string, rename, File};
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

    // init bashrc
    if args.init {
        let _ = create_term_dir();
        let bash_config_path = PathBuf::from("init").join("termstat.bash");
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
            let _ = rename(&term_file, term_file.join(time_as_str));
            let _ = File::create(term_file).expect("Log file move over failed");
        }
    }

    println!("{}", args.init);
}
