use super::errors::FxError;
use super::layout::Split;
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;
use std::path::Path;

pub const SESSION_FILE: &str = ".session";
pub const SESSION_EXAMPLE: &str = "sort_by = \"Name\"
show_hidden = false
preview = false
split = Vertical
";

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Session {
    pub sort_by: SortKey,
    pub show_hidden: bool,
    pub preview: Option<bool>,
    pub split: Option<Split>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum SortKey {
    Name,
    Time,
}

pub fn read_session(session_path: &Path) -> Result<Session, FxError> {
    match read_to_string(session_path) {
        Ok(s) => match serde_yaml::from_str(&s) {
            Ok(de) => Ok(de),
            Err(_) => Ok(Session {
                sort_by: SortKey::Name,
                show_hidden: true,
                preview: Some(false),
                split: Some(Split::Vertical),
            }),
        },
        Err(_) => Ok(Session {
            sort_by: SortKey::Name,
            show_hidden: true,
            preview: Some(false),
            split: Some(Split::Vertical),
        }),
    }
}

pub fn make_session(session_file: &Path) -> Result<(), FxError> {
    std::fs::write(session_file, SESSION_EXAMPLE)?;
    Ok(())
}
