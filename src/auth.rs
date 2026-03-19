use crate::error::{Error, Result};
use std::env;

#[derive(Debug)]
pub struct Credentials {
    pub cookie: String,
    pub key: String,
}

pub trait AppleScriptRunner: Send + Sync {
    fn run_osascript(&self, script: &str) -> std::result::Result<String, String>;
}

pub struct OsascriptRunner;

impl AppleScriptRunner for OsascriptRunner {
    fn run_osascript(&self, script: &str) -> std::result::Result<String, String> {
        let output = std::process::Command::new("osascript")
            .arg("-e")
            .arg(script)
            .output()
            .map_err(|e| format!("Failed to run osascript: {e}"))?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
        }
    }
}

const APPLESCRIPT_REQUEST: &str = r#"tell application "iTerm2" to request cookie and key"#;

pub fn resolve_credentials(runner: &dyn AppleScriptRunner) -> Result<Credentials> {
    // Try env vars first
    if let (Ok(cookie), Ok(key)) = (env::var("ITERM2_COOKIE"), env::var("ITERM2_KEY")) {
        if !cookie.is_empty() && !key.is_empty() {
            return Ok(Credentials { cookie, key });
        }
    }

    // Fall back to osascript
    let output = runner
        .run_osascript(APPLESCRIPT_REQUEST)
        .map_err(|e| Error::Auth(format!("osascript failed: {e}")))?;

    parse_cookie_key(&output)
}

fn parse_cookie_key(output: &str) -> Result<Credentials> {
    // Expected format: "cookie\nkey" or "cookie key" (space or newline separated)
    let parts: Vec<&str> = output.split_whitespace().collect();
    if parts.len() == 2 {
        Ok(Credentials {
            cookie: parts[0].to_string(),
            key: parts[1].to_string(),
        })
    } else {
        Err(Error::Auth(format!(
            "Failed to parse cookie/key from osascript output: {output:?}"
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockRunner {
        result: std::result::Result<String, String>,
    }

    impl AppleScriptRunner for MockRunner {
        fn run_osascript(&self, _script: &str) -> std::result::Result<String, String> {
            self.result.clone()
        }
    }

    #[test]
    fn resolve_from_env_vars() {
        // Save and set env vars
        let old_cookie = env::var("ITERM2_COOKIE").ok();
        let old_key = env::var("ITERM2_KEY").ok();

        env::set_var("ITERM2_COOKIE", "test_cookie");
        env::set_var("ITERM2_KEY", "test_key");

        let runner = MockRunner {
            result: Err("should not be called".to_string()),
        };
        let creds = resolve_credentials(&runner).unwrap();
        assert_eq!(creds.cookie, "test_cookie");
        assert_eq!(creds.key, "test_key");

        // Restore
        match old_cookie {
            Some(v) => env::set_var("ITERM2_COOKIE", v),
            None => env::remove_var("ITERM2_COOKIE"),
        }
        match old_key {
            Some(v) => env::set_var("ITERM2_KEY", v),
            None => env::remove_var("ITERM2_KEY"),
        }
    }

    #[test]
    fn resolve_from_osascript() {
        // Ensure env vars are unset
        let old_cookie = env::var("ITERM2_COOKIE").ok();
        let old_key = env::var("ITERM2_KEY").ok();
        env::remove_var("ITERM2_COOKIE");
        env::remove_var("ITERM2_KEY");

        let runner = MockRunner {
            result: Ok("abc123 def456".to_string()),
        };
        let creds = resolve_credentials(&runner).unwrap();
        assert_eq!(creds.cookie, "abc123");
        assert_eq!(creds.key, "def456");

        // Restore
        if let Some(v) = old_cookie {
            env::set_var("ITERM2_COOKIE", v);
        }
        if let Some(v) = old_key {
            env::set_var("ITERM2_KEY", v);
        }
    }

    #[test]
    fn resolve_from_osascript_newline_separated() {
        let old_cookie = env::var("ITERM2_COOKIE").ok();
        let old_key = env::var("ITERM2_KEY").ok();
        env::remove_var("ITERM2_COOKIE");
        env::remove_var("ITERM2_KEY");

        let runner = MockRunner {
            result: Ok("abc123\ndef456".to_string()),
        };
        let creds = resolve_credentials(&runner).unwrap();
        assert_eq!(creds.cookie, "abc123");
        assert_eq!(creds.key, "def456");

        if let Some(v) = old_cookie {
            env::set_var("ITERM2_COOKIE", v);
        }
        if let Some(v) = old_key {
            env::set_var("ITERM2_KEY", v);
        }
    }

    #[test]
    fn osascript_failure() {
        let old_cookie = env::var("ITERM2_COOKIE").ok();
        let old_key = env::var("ITERM2_KEY").ok();
        env::remove_var("ITERM2_COOKIE");
        env::remove_var("ITERM2_KEY");

        let runner = MockRunner {
            result: Err("connection refused".to_string()),
        };
        let err = resolve_credentials(&runner).unwrap_err();
        assert!(err.to_string().contains("osascript failed"));

        if let Some(v) = old_cookie {
            env::set_var("ITERM2_COOKIE", v);
        }
        if let Some(v) = old_key {
            env::set_var("ITERM2_KEY", v);
        }
    }

    #[test]
    fn parse_bad_output() {
        let old_cookie = env::var("ITERM2_COOKIE").ok();
        let old_key = env::var("ITERM2_KEY").ok();
        env::remove_var("ITERM2_COOKIE");
        env::remove_var("ITERM2_KEY");

        let runner = MockRunner {
            result: Ok("justonetoken".to_string()),
        };
        let err = resolve_credentials(&runner).unwrap_err();
        assert!(err.to_string().contains("Failed to parse"));

        if let Some(v) = old_cookie {
            env::set_var("ITERM2_COOKIE", v);
        }
        if let Some(v) = old_key {
            env::set_var("ITERM2_KEY", v);
        }
    }
}
