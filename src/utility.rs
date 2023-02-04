use std::path::PathBuf;

#[derive(Debug)]
pub enum EnchantError {
    MissingFile(PathBuf),
}

impl std::fmt::Display for EnchantError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EnchantError::MissingFile(p) => write!(f, "Missing file: {}", p.display()),
        }
    }
}

impl std::error::Error for EnchantError {}

pub type Result<T> = std::result::Result<T, EnchantError>;

pub fn read_maybe_missing_file(p: PathBuf) -> Result<String> {
    if p.exists() {
        Ok(std::fs::read_to_string(p).unwrap())
    } else {
        Err(EnchantError::MissingFile(p))
    }
}

pub fn conditional(value: &str, cond: bool) -> &str {
    if cond {
        value
    } else {
        ""
    }
}
