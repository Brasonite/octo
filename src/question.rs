use std::path::Path;

use anyhow::{Result, anyhow};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Question {
    pub title: String,
    pub time_limit: f32,

    pub solver: String,

    pub input_folders: Vec<String>,
    pub output_folders: Vec<String>,

    pub root: Option<String>,
    pub input_format: String,
    pub output_format: String,
}

impl Question {
    pub fn load(path: &str) -> Result<Self> {
        let data = std::fs::read_to_string(path)?;
        let mut question: Self = toml::from_str(&data).map_err(|err| anyhow!(err))?;

        if question.root.is_none() {
            question.root = Some(
                Path::new(path)
                    .parent()
                    .expect("Failed to get the question's parent folder")
                    .to_string_lossy()
                    .to_string(),
            );
        }

        Ok(question)
    }
}
