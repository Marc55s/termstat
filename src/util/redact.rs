use regex::{Regex, Captures};
use rusqlite::Error as RusqliteError;

// --- Setup: Custom Error Type and Result ---
// Define a simple custom error for use in the Result<T, E> type.
#[derive(Debug)]
pub enum RedactionError {
    Regex(regex::Error),
}

impl From<regex::Error> for RedactionError {
    fn from(err: regex::Error) -> Self {
        RedactionError::Regex(err)
    }
}


// Result alias for convenience
pub type Result<T> = std::result::Result<T, RedactionError>;

const REDACTION_PLACEHOLDER: &str = "[REDACTED]";

pub fn redact_command(cmd: &str) -> Result<String> {
    let mut censored_command = cmd.to_owned();

    let env_regex = Regex::new(
        r#"(\b(?:KEY|SECRET|PASS|GITHUB_SECRET|PASSWORD|POSTGRES|DB)[_A-Z0-9]*=)(?:['"])?([^'\s"]*)(?:['"])?"#
    )?;

    censored_command = env_regex.replace_all(&censored_command, |caps: &Captures| {
        format!("{}{}", &caps[1], REDACTION_PLACEHOLDER)
    }).to_string();

    let flag_regex = Regex::new(
        r#"(\s--?(?:password|p))(?:[\s=])?(?:['"])?([^'\s"]*)(?:['"])?"#
    )?;
    
    censored_command = flag_regex.replace_all(&censored_command, |caps: &Captures| {
        format!("{} {}", &caps[1], REDACTION_PLACEHOLDER)
    }).to_string();

    println!("Censored Command: {}", censored_command);

    Ok(censored_command)
}


#[cfg(test)] 
mod tests {
    use super::*; 

    macro_rules! assert_redaction {
        ($input:expr, $expected:expr) => {
            assert_eq!(redact_command($input).unwrap(), $expected);
        };
    }

    // --- Environment Variable Tests (Simple Scope) ---

    #[test]
    fn test_redaction_simple_key_export() {
        assert_redaction!(
            "export POSTGRES_PASS=1234567890 git push",
            "export POSTGRES_PASS=[REDACTED] git push"
        );
    }

    #[test]
    fn test_redaction_key_export() {
        assert_redaction!(
            "export POSTGRES_PASS=1234567890!§$%&/()=",
            "export POSTGRES_PASS=[REDACTED]"
        );
    }

    #[test]
    fn test_redaction_secret_no_export() {
        assert_redaction!(
            "GITHUB_SECRET=abcxyz npm install",
            "GITHUB_SECRET=[REDACTED] npm install"
        );
    }

    #[test]
    fn test_redaction_password_quoted() {
        assert_redaction!(
            "DB_PASS=\"strong_password\" && psql",
            "DB_PASS=[REDACTED] && psql"
        );
    }
    
    // --- Command Flag Tests (Simple Scope) ---

    #[test]
    fn test_redaction_flag_short_p() {
        // The regex captures the value immediately following -p
        assert_redaction!(
            "mysql -u root -pmy_db_pass",
            "mysql -u root -p [REDACTED]"
        );
    }

    #[test]
    fn test_redaction_flag_long_equals() {
        assert_redaction!(
            "docker login --password=strong_password123 myregistry",
            "docker login --password [REDACTED] myregistry"
        );
    }

    #[test]
    fn test_redaction_flag_with_space() {
        assert_redaction!(
            "sftp --password mytoken user@host",
            "sftp --password [REDACTED] user@host"
        );
    }

    // --- Combination and Negative Tests ---

    #[test]
    fn test_redaction_mixed_simple() {
        assert_redaction!(
            "PASSWORD_VAR=abc psql -p mypassword",
            "PASSWORD_VAR=[REDACTED] psql -p [REDACTED]"
        );
    }
    
    #[test]
    fn test_no_redaction_safe_command() {
        assert_redaction!(
            "git status --verbose | grep file.rs",
            "git status --verbose | grep file.rs"
        );
    }
}
